#[derive(Debug)]
pub struct ErrorWrapper<E>
where
    E: std::fmt::Display,
{
    err: E,
    location: String,
}

impl<E> ErrorWrapper<E>
where
    E: std::fmt::Display,
{
    #[cfg(feature = "full-file-path")]
    pub fn new(err: E, file: &str, line: u32, column: u32) -> Self {
        let location = format!("{file}({line}/{column})");
        Self { err, location }
    }

    #[cfg(not(feature = "full-file-path"))]
    pub fn new(err: E, file: &str, line: u32, _: u32) -> Self {
        #[cfg(unix)]
        const PATH_SEP: char = '/';
        #[cfg(windows)]
        const PATH_SEP: char = '\\';

        let filename = match file.rfind(PATH_SEP) {
            Some(start) => match file.rfind('.') {
                Some(end) => &file[(start + 1)..end],
                None => &file[(start + 1)..],
            },
            None => file,
        };

        let location = format!("{filename}({line})");
        Self { err, location }
    }
}

impl<E> ErrorWrapper<E>
where
    E: std::fmt::Display,
{
    pub fn error(&self) -> &E {
        &self.err
    }
}

impl<E> std::fmt::Display for ErrorWrapper<E>
where
    E: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} at {}", self.err, self.location)
    }
}

#[cfg(feature = "prefix-error")]
impl<E> ErrorWrapper<E>
where
    E: std::fmt::Display,
{
    pub fn format(&self, f: &mut std::fmt::Formatter<'_>, name: &'static str) -> std::fmt::Result {
        write!(f, "{}-error: {}", name.to_lowercase(), self)
    }
}

#[cfg(not(feature = "prefix-error"))]
impl<E> ErrorWrapper<E>
where
    E: std::fmt::Display,
{
    pub fn format(&self, f: &mut std::fmt::Formatter<'_>, _: &'static str) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

#[derive(Debug)]
pub struct CustomMessageError {
    message: String,
}

impl std::fmt::Display for CustomMessageError {
    #[cfg(feature = "prefix-error")]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "custom-error: {}", self.message)
    }
    #[cfg(not(feature = "prefix-error"))]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl CustomMessageError {
    pub fn new(message: String) -> CustomMessageError {
        Self { message }
    }
}

#[macro_export]
macro_rules! raise {
    ($($arg:tt)*) => {
       return Result::Err(Error::custom_message(format!($($arg)*)))
    }
}

#[macro_export]
macro_rules! throw {
    ($err:expr) => {
        return Result::Err(Error::custom_error($err))
    };
}

#[macro_export]
macro_rules! define_error {
    ($( $(#[$field_attr:meta])* ($name:ident,$error:ty ) ),* ) => {
        #[derive(Debug)]
        pub enum Error {
            $( $(#[$field_attr])* $name(ErrorWrapper::<$error>) ),*,
            CustomMessage(ErrorWrapper<CustomMessageError>)
        }

        impl std::error::Error for Error{}

        $(
            $(#[$field_attr])*
            impl std::convert::From<$error> for Error {
                #[track_caller]
                fn from(err:$error) -> Self {
                    let caller = core::panic::Location::caller();
                    Error::$name(ErrorWrapper::new(err, caller.file(), caller.line(), caller.column()))
                }
            }
        )*

        impl Error{
            #[track_caller]
            pub fn custom_message<S>(message:S)->Self where S: Into<String>,{
                let caller = core::panic::Location::caller();
                Error::CustomMessage(ErrorWrapper::new(CustomMessageError::new(message.into()), caller.file(), caller.line(), caller.column()))
            }
            #[track_caller]
            pub fn custom_error<E:std::fmt::Display>(error:E)->Self{
                let caller = core::panic::Location::caller();
                let message = format!("{}: {}",std::any::type_name::<E>(),error);
                Error::CustomMessage(ErrorWrapper::new(CustomMessageError::new(message), caller.file(), caller.line(), caller.column()))
            }
        }

        impl std::fmt::Display for Error {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    $($(#[$field_attr])* Self::$name(err) =>err.format(f,stringify!($name)),)*
                    Self::CustomMessage(err) => err.fmt(f),
                }
            }
        }
    };
}
