use crate::error::{read_error, Error, InnerError::ReadError};
use std::collections::HashMap;
use std::fs;
use std::fs::metadata;
use std::os::linux::fs::MetadataExt;
use std::path::Path;
use std::str::FromStr;

pub struct User {
    name: String,
    uid: u32,
    gid: u32,
    groups: String,
}

pub struct UserInfo {
    usrs_set: HashMap<u32, User>,
}

impl UserInfo {
    /// Get all users list
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ylong_sysinfo::UserInfo;
    ///
    /// // get a hashmap contain all users' info
    /// let users_list_info = UserInfo::new().unwrap();
    /// ```
    pub fn new() -> Result<Self, Error> {
        let group_content = Self::read_sys_file("group")?;
        let group_lines = group_content.lines();
        let mut group_map = HashMap::new();
        for line in group_lines {
            let line_vec = line.split(':').collect::<Vec<&str>>();
            let line_key_str = line_vec.get(2).map(|c| c.trim()).unwrap_or_default();
            let line_value = line_vec
                .first()
                .map(|c| c.trim().to_string())
                .unwrap_or_default();
            let line_key = u32::from_str(line_key_str).map_err(|e| {
                Error::Internal(ReadError(format!("transfer type fail, error is {e}.")))
            })?;
            group_map.insert(line_key, line_value);
        }

        let mut user_map = HashMap::new();
        let passwed_content = Self::read_sys_file("passwd")?;
        let content_lines = passwed_content.lines();
        for line in content_lines {
            let line_vec = line.split(':').collect::<Vec<&str>>();
            let shell_command = line_vec.last().unwrap().trim_end();
            if shell_command.is_empty()
                || shell_command.ends_with("/false")
                || shell_command.ends_with("/nologin")
            {
                // Skip users if shell end up with false/nologin/empty
                continue;
            }
            // Skip users if parsing failed
            // uid info
            let user_id = match u32::from_str(line_vec[2]).ok() {
                Some(content) => content,
                None => {
                    continue;
                }
            };

            // username info
            let username = line_vec[0];

            // gid info
            let group_id = u32::from_str(line_vec[3]).map_err(|e| {
                Error::Internal(ReadError(format!("Parsing group_id fail, error is {e}.")))
            })?;

            // group info
            let group_info = match group_map.get(&group_id) {
                Some(value) => value.to_string(),
                None => "no_group".to_string(),
            };

            let user_content = User {
                name: username.to_string(),
                uid: user_id,
                gid: group_id,
                groups: group_info,
            };
            user_map.insert(user_content.uid, user_content);
        }
        Ok(Self { usrs_set: user_map })
    }

    /// Get all users info
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ylong_sysinfo::UserInfo;
    ///
    /// // get all users list
    /// let mut users_list_info = UserInfo::new().unwrap();
    /// let username = users_list_info.get_all_users_info();
    /// username.unwrap();
    /// ```
    pub fn get_all_users_info(&mut self) -> Result<&HashMap<u32, User>, Error> {
        Ok(&self.usrs_set)
    }

    /// Get a user id by proc path
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ylong_sysinfo::UserInfo;
    ///
    /// // get a user id
    /// let mut users_list_info = UserInfo::new().unwrap();
    /// let uid = users_list_info.get_uid_by_proc_path("/proc/1");
    /// uid.unwrap();
    /// ```
    pub fn get_uid_by_proc_path<P: AsRef<Path>>(&mut self, path: P) -> Result<u32, Error> {
        Self::get_uid_by_file_path(path)
    }

    /// Get a user name by uid
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ylong_sysinfo::UserInfo;
    ///
    /// // get a username
    /// let mut users_list_info = UserInfo::new().unwrap();
    /// let username = users_list_info.get_username_by_uid(1);
    /// username.unwrap();
    /// ```
    pub fn get_username_by_uid(&mut self, uid: u32) -> Result<&String, Error> {
        let users_set = &self.usrs_set;
        let user = users_set.get(&uid).unwrap();
        Ok(&user.name)
    }

    /// Get a user gid by uid
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ylong_sysinfo::UserInfo;
    ///
    /// // get a gid
    /// let mut users_list_info = UserInfo::new().unwrap();
    /// let gid = users_list_info.get_gid_by_uid(1);
    /// gid.unwrap();
    /// ```
    pub fn get_gid_by_uid(&mut self, uid: u32) -> Result<u32, Error> {
        let users_set = &self.usrs_set;
        let user = users_set.get(&uid).unwrap();
        Ok(user.gid)
    }

    /// Get a user group by uid
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ylong_sysinfo::UserInfo;
    ///
    /// // get a group
    /// let mut users_list_info = UserInfo::new().unwrap();
    /// let group = users_list_info.get_group_by_uid(1);
    /// group.unwrap();
    /// ```
    pub fn get_group_by_uid(&mut self, uid: u32) -> Result<&String, Error> {
        let users_set = &self.usrs_set;
        let user = users_set.get(&uid).unwrap();
        Ok(&user.groups)
    }

    fn read_sys_file(file: &str) -> Result<String, Error> {
        let file_path = format!("/etc/{file}");
        fs::read_to_string(file_path).map_err(|e| read_error("sys", file, e))
    }

    fn get_uid_by_file_path<P: AsRef<Path>>(path: P) -> Result<u32, Error> {
        let meta = metadata(path).map_err(|e| read_error("proc", "metadata", e))?;
        Ok(meta.st_uid())
    }
}

#[cfg(test)]
mod ut_user {
    use crate::sys::user::*;
    use std::fmt::{Debug, Formatter};

    impl Debug for User {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "")
        }
    }

    impl PartialEq for User {
        fn eq(&self, other: &Self) -> bool {
            self.name == other.name
                && self.uid == other.uid
                && self.gid == other.gid
                && self.groups == other.groups
        }
    }

    /// UT test for new, get_all_users_info ut test after block sys file.
    /// # Title
    /// ut_update_users_info_block
    ///
    /// # Brief
    /// 1, Creates UserInfo.
    /// 2, Gets all users list by get_all_users_info.
    /// 3, Checks the result.
    #[test]
    fn ut_update_users_info_block() {
        // todo! this test should use mock to get info after fixing mock framework
        let mut user_list = UserInfo::new().unwrap();
        let all_user_list = user_list.get_all_users_info();
        assert!(all_user_list.is_ok());
    }
}
