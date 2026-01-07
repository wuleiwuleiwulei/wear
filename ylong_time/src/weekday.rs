use crate::error::{BaseErrorKind, TimeError};
use std::fmt::{Display, Formatter};

/// The day of a week.
/// By default, Zero represents Monday.
#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum Weekday {
    /// Monday.
    Monday,
    /// Tuesday.
    Tuesday,
    /// Wednesday.
    Wednesday,
    /// Thursday.
    Thursday,
    /// Friday.
    Friday,
    /// Saturday.
    Saturday,
    /// Sunday.
    Sunday,
}

impl Weekday {
    /// Return the previous day in the week.
    ///
    /// # Examples
    ///
    /// ```
    /// use ylong_time::Weekday;
    ///
    /// let tuesday = Weekday::Tuesday;
    /// assert_eq!(Weekday::Monday, tuesday.prev());
    /// ```
    pub fn prev(&self) -> Weekday {
        match self {
            Weekday::Monday => Weekday::Sunday,
            Weekday::Tuesday => Weekday::Monday,
            Weekday::Wednesday => Weekday::Tuesday,
            Weekday::Thursday => Weekday::Wednesday,
            Weekday::Friday => Weekday::Thursday,
            Weekday::Saturday => Weekday::Friday,
            Weekday::Sunday => Weekday::Saturday,
        }
    }

    /// Return the next day in the week.
    ///
    /// # Examples
    ///
    /// ```
    /// use ylong_time::Weekday;
    ///
    /// let monday = Weekday::Monday;
    /// assert_eq!(Weekday::Tuesday, monday.next());
    /// ```
    pub fn next(&self) -> Weekday {
        match self {
            Weekday::Monday => Weekday::Tuesday,
            Weekday::Tuesday => Weekday::Wednesday,
            Weekday::Wednesday => Weekday::Thursday,
            Weekday::Thursday => Weekday::Friday,
            Weekday::Friday => Weekday::Saturday,
            Weekday::Saturday => Weekday::Sunday,
            Weekday::Sunday => Weekday::Monday,
        }
    }

    /// Get the abbreviated name of the week.
    ///
    /// # Examples
    ///
    /// ```
    /// use ylong_time::Weekday;
    ///
    /// let monday = Weekday::Monday;
    /// assert_eq!(monday.abbr_name(), "Mon");
    /// ```
    pub fn abbr_name(&self) -> &str {
        match self {
            Weekday::Monday => "Mon",
            Weekday::Tuesday => "Tue",
            Weekday::Wednesday => "Wed",
            Weekday::Thursday => "Thu",
            Weekday::Friday => "Fri",
            Weekday::Saturday => "Sat",
            Weekday::Sunday => "Sun",
        }
    }

    /// Get the full name of the week.
    ///
    /// # Examples
    ///
    /// ```
    /// use ylong_time::Weekday;
    ///
    /// let monday = Weekday::Monday;
    /// assert_eq!(monday.full_name(), "Monday");
    /// ```
    pub fn full_name(&self) -> &str {
        match self {
            Weekday::Monday => "Monday",
            Weekday::Tuesday => "Tuesday",
            Weekday::Wednesday => "Wednesday",
            Weekday::Thursday => "Thursday",
            Weekday::Friday => "Friday",
            Weekday::Saturday => "Saturday",
            Weekday::Sunday => "Sunday",
        }
    }

    /// Monday of the week is represented by 0_u8.
    /// Weekday will be converted to corresponding u8.
    ///
    /// # Examples
    ///
    /// ```
    /// use ylong_time::Weekday;
    ///
    /// let monday = Weekday::Monday;
    /// assert_eq!(monday.weekday0_to_u8(), 0);
    /// ```
    pub fn weekday0_to_u8(&self) -> u8 {
        match self {
            Weekday::Monday => 0,
            Weekday::Tuesday => 1,
            Weekday::Wednesday => 2,
            Weekday::Thursday => 3,
            Weekday::Friday => 4,
            Weekday::Saturday => 5,
            Weekday::Sunday => 6,
        }
    }

    /// Monday of the week is represented by 0_u8.
    /// U8 will be converted to corresponding weekday.
    ///
    /// # Examples
    ///
    /// ```
    /// use ylong_time::Weekday;
    ///
    /// let monday = Weekday::weekday0_from_u8(0).unwrap();
    /// assert_eq!(monday, Weekday::Monday);
    /// ```
    pub fn weekday0_from_u8(num: u8) -> Result<Self, TimeError> {
        match num {
            0 => Ok(Self::Monday),
            1 => Ok(Self::Tuesday),
            2 => Ok(Self::Wednesday),
            3 => Ok(Self::Thursday),
            4 => Ok(Self::Friday),
            5 => Ok(Self::Saturday),
            6 => Ok(Self::Sunday),
            _ => Err(TimeError::from(BaseErrorKind::InvalidWeekday)),
        }
    }

    /// Monday of the week is represented by 1_u8.
    /// Weekday will be converted to corresponding u8.
    ///
    /// # Examples
    ///
    /// ```
    /// use ylong_time::Weekday;
    ///
    /// let monday = Weekday::Monday;
    /// assert_eq!(monday.weekday_to_u8(), 1);
    /// ```
    pub fn weekday_to_u8(&self) -> u8 {
        match self {
            Weekday::Monday => 1,
            Weekday::Tuesday => 2,
            Weekday::Wednesday => 3,
            Weekday::Thursday => 4,
            Weekday::Friday => 5,
            Weekday::Saturday => 6,
            Weekday::Sunday => 7,
        }
    }

    /// Monday of the week is represented by 1_u8.
    /// U8 will be converted to corresponding weekday.
    ///
    /// # Examples
    ///
    /// ```
    /// use ylong_time::Weekday;
    ///
    /// let monday = Weekday::weekday_from_u8(1).unwrap();
    /// assert_eq!(monday, Weekday::Monday);
    /// ```
    pub fn weekday_from_u8(num: u8) -> Result<Self, TimeError> {
        match num {
            1 => Ok(Self::Monday),
            2 => Ok(Self::Tuesday),
            3 => Ok(Self::Wednesday),
            4 => Ok(Self::Thursday),
            5 => Ok(Self::Friday),
            6 => Ok(Self::Saturday),
            7 => Ok(Self::Sunday),
            _ => Err(TimeError::from(BaseErrorKind::InvalidWeekday)),
        }
    }
}

impl Display for Weekday {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.abbr_name())
    }
}

#[cfg(test)]
mod test {
    include!("../tests/ut/ut_weekday.rs");
}
