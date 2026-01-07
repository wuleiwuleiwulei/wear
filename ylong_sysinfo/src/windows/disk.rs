use crate::error::InnerError::{CannotConvert, GetDiskFreeSpaceFailed, GetVolumeInfoFailed};
use crate::error::{Error, InnerError};
use crate::sys::ffi::{
    GetDiskFreeSpaceExW, GetDriveTypeW, GetLogicalDrives, GetVolumeInformationW,
};
use std::ffi::{c_uint, c_ulong, c_ulonglong};
use std::mem::zeroed;
use std::path::Path;
use std::slice::{Iter, IterMut};

// In the Windows API (with some exceptions discussed in the following paragraphs),
// the maximum length for a path is MAX_PATH,
// which is defined as 260 characters.
const MAX_PATH: usize = 260;

pub struct DisksInfo {
    disks: Vec<Disk>,
}

impl DisksInfo {
    /// Creates a list structure of information about all disks.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ylong_sysinfo::{DisksInfo, Error};
    ///
    /// // Gets basic information about the disk list.
    /// let disks_info = DisksInfo::new()?;
    /// # Ok::<(), Error>(())
    /// ```
    pub fn new() -> Result<Self, Error> {
        let logical_driver = unsafe { GetLogicalDrives() };
        let logical_driver_number = logical_driver.count_ones();
        let logical_driver_start = logical_driver.trailing_zeros();

        let mut disks = Vec::with_capacity(logical_driver_number as usize);
        for index in logical_driver_start..logical_driver_start + logical_driver_number {
            let mut disk = Disk::default();

            let mount_point = [b'A' as u16 + index as u16, b':' as u16, b'\\' as u16, 0];
            let driver_type = unsafe { GetDriveTypeW(mount_point.as_ptr()) };
            // DriveRemovable == 2
            let is_removable = driver_type == 2;
            // The length of a volume name buffer, in TCHARs. The maximum buffer size is MAX_PATH+1.
            let mut name = [0_u16; MAX_PATH + 1];
            // The length of the file system name buffer, in TCHARs. The maximum buffer size is MAX_PATH+1.
            let mut file_system = [0_u16; MAX_PATH + 1];

            if unsafe {
                GetVolumeInformationW(
                    mount_point.as_ptr(),
                    name.as_mut_ptr(),
                    name.len() as c_ulong,
                    std::ptr::null_mut(),
                    std::ptr::null_mut(),
                    std::ptr::null_mut(),
                    file_system.as_mut_ptr(),
                    file_system.len() as c_ulong,
                )
            } == 0
            {
                return Err(Error::Internal(GetVolumeInfoFailed));
            }

            disk.set_device_name(String::from_utf16_lossy(
                name.split(|num| *num == 0).next().unwrap(),
            ));
            disk.set_drive_type(DriveType::try_from(driver_type).unwrap());
            disk.set_is_removable(is_removable);
            disk.set_file_system(String::from_utf16_lossy(
                file_system.split(|num| *num == 0).next().unwrap(),
            ));
            disk.set_mount_point(String::from_utf16_lossy(
                &mount_point[..mount_point.len() - 1],
            ));
            disk.update()?;

            disks.push(disk);
        }

        Ok(Self { disks })
    }

    /// Gets disks in `DisksInfo`.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ylong_sysinfo::{DisksInfo, Error};
    ///
    /// let disks_info = DisksInfo::new()?;
    /// let mut disks = disks_info.disks();
    /// disks.sort();
    /// for disk in disks {
    ///     println!("{:?}", disk);
    /// }
    /// # Ok::<(), Error>(())
    /// ```
    pub fn disks(&self) -> Vec<Disk> {
        self.disks.clone()
    }

    /// Gets disk information at a given path.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ylong_sysinfo::{DisksInfo, Error};
    ///
    /// let disks_info = DisksInfo::new()?;
    /// let disk = disks_info.disk_at("D:\\")?;
    /// println!("{:?}", disk);
    /// # Ok::<(), Error>(())
    /// ```
    pub fn disk_at<P: AsRef<Path>>(&self, path: P) -> Result<Disk, Error> {
        self.disks()
            .into_iter()
            .find(|disk| Path::new(&disk.mount_point) == path.as_ref())
            .ok_or(Error::Internal(InnerError::NotFound))
    }

    /// Gets iterators of `DisksInfo`.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ylong_sysinfo::{DisksInfo, Error};
    ///
    /// let disks_info = DisksInfo::new()?;
    /// for disk in disks_info.iter() {
    ///     println!("{:?}", disk);
    /// }
    /// # Ok::<(), Error>(())
    /// ```
    pub fn iter(&self) -> Iter<Disk> {
        self.disks.iter()
    }

    /// Gets mutable iterators of 'DisksInfo'.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ylong_sysinfo::{DisksInfo, Error};
    ///
    /// let mut disks_info = DisksInfo::new()?;
    /// for disk in disks_info.iter_mut() {
    ///     println!("{:?}", disk);
    /// }
    /// # Ok::<(), Error>(())
    /// ```
    pub fn iter_mut(&mut self) -> IterMut<Disk> {
        self.disks.iter_mut()
    }
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct Disk {
    device_name: String,
    drive_type: DriveType,
    is_removable: bool,
    file_system: String,
    mount_point: String,
    disk_usage: DiskUsage,
}

impl Default for Disk {
    fn default() -> Self {
        Self {
            device_name: "".to_string(),
            drive_type: DriveType::Unknown,
            is_removable: false,
            file_system: "".to_string(),
            mount_point: "".to_string(),
            disk_usage: Default::default(),
        }
    }
}

impl Disk {
    /// Gets the device name.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ylong_sysinfo::{DisksInfo, Error};
    ///
    /// let mut disks_info = DisksInfo::new()?;
    /// let mut name = vec![];
    /// for disk in disks_info.iter() {
    ///     name.push(disk.device_name());
    /// }
    /// # Ok::<(), Error>(())
    /// ```
    pub fn device_name(&self) -> &str {
        &self.device_name
    }

    /// Gets the drive type.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ylong_sysinfo::{DisksInfo, Error};
    ///
    /// let mut disks_info = DisksInfo::new()?;
    /// let mut drive_type = vec![];
    /// for disk in disks_info.iter() {
    ///     drive_type.push(disk.drive_type());
    /// }
    /// # Ok::<(), Error>(())
    /// ```
    pub fn drive_type(&self) -> &DriveType {
        &self.drive_type
    }

    /// Determines if the disk is a removable device.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ylong_sysinfo::{DisksInfo, Error};
    ///
    /// let mut disks_info = DisksInfo::new()?;
    /// let mut is_removable = vec![];
    /// for disk in disks_info.iter() {
    ///     is_removable.push(disk.is_removable());
    /// }
    /// # Ok::<(), Error>(())
    /// ```
    pub fn is_removable(&self) -> bool {
        self.is_removable
    }

    /// Gets the file system.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ylong_sysinfo::{DisksInfo, Error};
    ///
    /// let mut disks_info = DisksInfo::new()?;
    /// let mut file_system = vec![];
    /// for disk in disks_info.iter() {
    ///     file_system.push(disk.file_system());
    /// }
    /// # Ok::<(), Error>(())
    /// ```
    pub fn file_system(&self) -> &str {
        &self.file_system
    }

    /// Gets the mount point.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ylong_sysinfo::{DisksInfo, Error};
    ///
    /// let mut disks_info = DisksInfo::new()?;
    /// let mut mount_point = vec![];
    /// for disk in disks_info.iter() {
    ///     mount_point.push(disk.mount_point());
    /// }
    /// # Ok::<(), Error>(())
    /// ```
    pub fn mount_point(&self) -> &str {
        &self.mount_point
    }

    /// Gets the total space in bytes.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ylong_sysinfo::{DisksInfo, Error};
    ///
    /// let mut disks_info = DisksInfo::new()?;
    /// let mut total_space = vec![];
    /// for disk in disks_info.iter() {
    ///     total_space.push(disk.total_space());
    /// }
    /// # Ok::<(), Error>(())
    /// ```
    pub fn total_space(&self) -> u64 {
        self.disk_usage.total_space
    }

    /// Gets the free space in bytes.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ylong_sysinfo::{DisksInfo, Error};
    ///
    /// let mut disks_info = DisksInfo::new()?;
    /// let mut free_space = vec![];
    /// for disk in disks_info.iter() {
    ///     free_space.push(disk.free_space());
    /// }
    /// # Ok::<(), Error>(())
    /// ```
    pub fn free_space(&self) -> u64 {
        self.disk_usage.free_space
    }

    /// Gets the avail space in bytes.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ylong_sysinfo::{DisksInfo, Error};
    ///
    /// let mut disks_info = DisksInfo::new()?;
    /// let mut avail_space = vec![];
    /// for disk in disks_info.iter() {
    ///     avail_space.push(disk.avail_space());
    /// }
    /// # Ok::<(), Error>(())
    /// ```
    pub fn avail_space(&self) -> u64 {
        self.disk_usage.avail_space
    }

    /// Gets the used space in bytes.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ylong_sysinfo::{DisksInfo, Error};
    ///
    /// let mut disks_info = DisksInfo::new()?;
    /// let mut used_space = vec![];
    /// for disk in disks_info.iter() {
    ///     used_space.push(disk.used_space());
    /// }
    /// # Ok::<(), Error>(())
    /// ```
    pub fn used_space(&self) -> u64 {
        self.disk_usage.used_space
    }

    /// Updates the disk info.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ylong_sysinfo::{DisksInfo, Error};
    ///
    /// let mut disks_info = DisksInfo::new()?;
    /// for disk in disks_info.iter_mut() {
    ///     disk.update().unwrap();
    /// }
    /// # Ok::<(), Error>(())
    /// ```
    pub fn update(&mut self) -> Result<(), Error> {
        unsafe {
            let mut mount_point = self.mount_point.encode_utf16().collect::<Vec<_>>();
            mount_point.push(0);
            let mut total_space: c_ulonglong = zeroed();
            let mut free_space: c_ulonglong = zeroed();
            let mut avail_space: c_ulonglong = zeroed();

            if GetDiskFreeSpaceExW(
                mount_point.as_ptr(),
                &mut avail_space,
                &mut total_space,
                &mut free_space,
            ) == 0
            {
                return Err(Error::Internal(GetDiskFreeSpaceFailed));
            }

            let mut disk_usage = DiskUsage::default();
            disk_usage.set_total_space(total_space);
            disk_usage.set_free_space(free_space);
            disk_usage.set_avail_space(avail_space);
            disk_usage.set_used_space(total_space.wrapping_sub(free_space));
            self.set_disk_usage(disk_usage);
        }

        Ok(())
    }

    fn set_device_name(&mut self, device_name: String) {
        self.device_name = device_name;
    }

    fn set_drive_type(&mut self, drive_type: DriveType) {
        self.drive_type = drive_type;
    }

    fn set_is_removable(&mut self, is_removable: bool) {
        self.is_removable = is_removable;
    }

    fn set_file_system(&mut self, file_system: String) {
        self.file_system = file_system;
    }

    fn set_mount_point(&mut self, mount_point: String) {
        self.mount_point = mount_point;
    }

    fn set_disk_usage(&mut self, disk_usage: DiskUsage) {
        self.disk_usage = disk_usage;
    }
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum DriveType {
    // The drive type cannot be determined.
    Unknown,

    // The root path is invalid.
    // For example, there is no volume mounted at the specified path.
    NoRootDir,

    // The drive has removable media.
    // For example, a floppy drive, thumb drive, or flash card reader.
    Removable,

    // The drive has fixed media.
    // For example, a hard disk drive or flash drive.
    Fixed,

    // The drive is a remote (network) drive.
    Remote,

    // The drive is a CD-ROM drive.
    CDRom,

    // The drive is a RAM disk.
    RamDisk,
}

impl TryFrom<c_uint> for DriveType {
    type Error = Error;

    fn try_from(value: c_uint) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(DriveType::Unknown),
            1 => Ok(DriveType::NoRootDir),
            2 => Ok(DriveType::Removable),
            3 => Ok(DriveType::Fixed),
            4 => Ok(DriveType::Remote),
            5 => Ok(DriveType::CDRom),
            6 => Ok(DriveType::RamDisk),
            _ => Err(Error::Internal(CannotConvert)),
        }
    }
}

#[derive(Debug, Default, Clone, Ord, PartialOrd, Eq, PartialEq)]
struct DiskUsage {
    total_space: u64,

    free_space: u64,

    avail_space: u64,

    used_space: u64,
}

impl DiskUsage {
    fn set_total_space(&mut self, total_space: u64) {
        self.total_space = total_space;
    }

    fn set_free_space(&mut self, free_space: u64) {
        self.free_space = free_space;
    }

    fn set_avail_space(&mut self, avail_space: u64) {
        self.avail_space = avail_space;
    }

    fn set_used_space(&mut self, used_space: u64) {
        self.used_space = used_space;
    }
}

#[cfg(test)]
mod test {
    use crate::DisksInfo;

    /// UT test for get disks information.
    ///
    /// # Title
    /// ut_disks_info_test
    ///
    /// # Brief
    /// 1. Verifies that the `DisksInfo` information structure was successfully retrieved.
    #[test]
    fn ut_disks_info_test() {
        let disks_info = DisksInfo::new();
        assert!(disks_info.is_ok());
    }

    /// UT test for get disks information.
    ///
    /// # Title
    /// ut_disk_test
    ///
    /// # Brief
    /// 1. Verifies that the `DisksInfo` information structure was successfully retrieved.
    /// 2. Verifies that the `Disk` internal data is correct.
    #[test]
    fn ut_disk_test() {
        let disks_info = DisksInfo::new().unwrap();
        let disks = disks_info.disks;
        assert_ne!(disks.len(), 0);
    }

    /// UT test for get disks information.
    ///
    /// # Title
    /// ut_disk_iter_test
    ///
    /// # Brief
    /// 1. Verifies that the `Disk` iterator was successfully fetched.
    /// 2. Verifies that the `Disk` internal data is correct.
    #[test]
    fn ut_disk_iter_test() {
        let disk_info = DisksInfo::new().unwrap();
        for disk in disk_info.iter() {
            assert!(!(disk.device_name.is_empty()));
            assert!(!(disk.file_system.is_empty()));
            assert!(!(disk.mount_point.is_empty()));
        }
    }

    /// UT test for get disks information.
    ///
    /// # Title
    /// ut_disk_iter_mut_test
    ///
    /// # Brief
    /// 1. Verifies that the `Disk` mutable iterator was successfully fetched.
    /// 2. Verifies that the `Disk` internal data is correct.
    #[test]
    fn ut_disk_iter_mut_test() {
        let mut disk_info = DisksInfo::new().unwrap();
        for disk in disk_info.iter_mut() {
            assert!(!(disk.device_name.is_empty()));
            assert!(!(disk.file_system.is_empty()));
            assert!(!(disk.mount_point.is_empty()));
        }
    }

    /// UT test for get disks information.
    ///
    /// # Title
    /// ut_update_test
    ///
    /// # Brief
    /// 1. Verifies that the `Disk` information structure can be successfully updated.
    /// 2. Verifies that the `Disk` internal data is correct.
    #[test]
    fn ut_update_test() {
        let mut disk_info = DisksInfo::new().unwrap();
        let mut before = vec![];
        for disk in disk_info.iter_mut() {
            before.push(disk.device_name.clone());
        }

        let mut after = vec![];
        for disk in disk_info.iter_mut() {
            let ret = disk.update();
            assert!(ret.is_ok())
        }
        for disk in disk_info.iter_mut() {
            after.push(disk.device_name.clone());
        }
        assert_eq!(before, after);
    }
}
