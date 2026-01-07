mod error;

macro_rules! use_linux_feature {
    ($($item:item)*) => {
        $(
            #[cfg(target_os = "linux")]
            $item
        )*
    }
}

macro_rules! use_windows_feature {
    ($($item:item)*) => {
        $(
            #[cfg(target_os = "windows")]
            $item
        )*
    }
}

use_linux_feature! {
    mod linux;
    use linux as sys;
}

use_windows_feature! {
    mod windows;
    use windows as sys;
}

pub use error::Error;
pub use sys::*;
