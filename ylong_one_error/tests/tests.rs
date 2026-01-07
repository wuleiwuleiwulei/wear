#[cfg(test)]
mod tests {
    use ylong_one_error::*;

    define_error! {
        (IO, std::io::Error),
        #[cfg(not(windows))]
        (ParseInt, std::num::ParseIntError),
        #[cfg(windows)]
        (ConvertInt, std::num::ParseIntError)
    }

    #[cfg(feature = "prefix-error")]
    #[test]
    fn test_raise_error() {
        fn biz_code(value: &str) -> Result<(), Error> {
            if value.is_empty() {
                raise!("value can not be empty");
            }
            Ok(())
        }
        assert!(biz_code("demo").is_ok());
        let error = biz_code("").err().unwrap();

        #[cfg(feature = "full-file-path")]
        assert_eq!(
            match cfg!(windows) {
                true => "custom-error: value can not be empty at src\\rhrl\\stdx\\ylong_one_error\\tests\\tests.rs(18/17)",
                false => "custom-error: value can not be empty at src/rhrl/stdx/ylong_one_error/tests/tests.rs(18/17)",
            },
            error.to_string().as_str().trim()
        );
        #[cfg(not(feature = "full-file-path"))]
        assert_eq!(
            match cfg!(windows) {
                true => "custom-error: value can not be empty at tests(18)",
                false => "custom-error: value can not be empty at tests(18)",
            },
            error.to_string().as_str().trim()
        );
    }

    #[cfg(not(feature = "prefix-error"))]
    #[test]
    fn test_raise_error() {
        fn biz_code(value: &str) -> Result<(), Error> {
            if value.is_empty() {
                raise!("value can not be empty");
            }
            Ok(())
        }
        assert!(biz_code("demo").is_ok());
        let error = biz_code("").err().unwrap();

        #[cfg(feature = "full-file-path")]
        assert_eq!(
            match cfg!(windows) {
                true =>
                    "value can not be empty at src\\rhrl\\stdx\\ylong_one_error\\tests\\tests.rs(48/17)",
                false =>
                    "value can not be empty at src/rhrl/stdx/ylong_one_error/tests/tests.rs(48/17)",
            },
            error.to_string().as_str().trim()
        );
        #[cfg(not(feature = "full-file-path"))]
        assert_eq!(
            match cfg!(windows) {
                true => "value can not be empty at tests(48)",
                false => "value can not be empty at tests(48)",
            },
            error.to_string().as_str().trim()
        );
    }

    #[cfg(feature = "prefix-error")]
    #[test]
    fn test_throw_error() {
        fn biz_code() -> Result<(), Error> {
            let value = "aa";
            let value = value.parse::<i32>();
            throw!(value.err().unwrap());
        }
        assert!(biz_code().is_err());
        let error = biz_code().err().unwrap();

        #[cfg(feature = "full-file-path")]
        assert_eq!(
            match cfg!(windows) {
                true =>  "custom-error: core::num::error::ParseIntError: invalid digit found in string at src\\rhrl\\stdx\\ylong_one_error\\tests\\tests.rs(81/13)",
                false => "custom-error: core::num::error::ParseIntError: invalid digit found in string at src/rhrl/stdx/ylong_one_error/tests/tests.rs(81/13)",
            },
            error.to_string().as_str().trim()
        );
        #[cfg(not(feature = "full-file-path"))]
        assert_eq!(
            match cfg!(windows) {
                true =>  "custom-error: core::num::error::ParseIntError: invalid digit found in string at tests(81)",
                false => "custom-error: core::num::error::ParseIntError: invalid digit found in string at tests(81)",
            },
            error.to_string().as_str().trim()
        );
    }

    #[cfg(not(feature = "prefix-error"))]
    #[test]
    fn test_throw_error() {
        fn biz_code() -> Result<(), Error> {
            let value = "aa";
            let value = value.parse::<i32>();
            throw!(value.err().unwrap());
        }
        assert!(biz_code().is_err());
        let error = biz_code().err().unwrap();

        #[cfg(feature = "full-file-path")]
        assert_eq!(
            match cfg!(windows) {
                true =>  "core::num::error::ParseIntError: invalid digit found in string at src\\rhrl\\stdx\\ylong_one_error\\tests\\tests.rs(110/13)",
                false => "core::num::error::ParseIntError: invalid digit found in string at src/rhrl/stdx/ylong_one_error/tests/tests.rs(110/13)",
            },
            error.to_string().as_str().trim()
        );
        #[cfg(not(feature = "full-file-path"))]
        assert_eq!(
            match cfg!(windows) {
                true =>
                    "core::num::error::ParseIntError: invalid digit found in string at tests(110)",
                false =>
                    "core::num::error::ParseIntError: invalid digit found in string at tests(110)",
            },
            error.to_string().as_str().trim()
        );
    }

    #[cfg(feature = "prefix-error")]
    #[test]
    fn test_parse_error() {
        fn biz_code(value: &str) -> Result<i32, Error> {
            let num = value.parse::<i32>()?;
            Ok(num + 10)
        }

        assert_eq!(21, biz_code("11").unwrap());
        let error = biz_code("abc").err();
        let target_kind = &std::num::IntErrorKind::InvalidDigit;

        #[cfg(not(windows))]
        {
            let is_my_error = matches!(error, Some(x) if matches!(&x, Error::ParseInt(e) if e.error().kind()==target_kind));
            assert!(is_my_error);
            let error = biz_code("abc").err();

            #[cfg(feature = "full-file-path")]
            assert_eq!(
                "parseint-error: invalid digit found in string at src/rhrl/stdx/ylong_one_error/tests/tests.rs(139/23)",
                format!("{}", error.unwrap())
            );
            #[cfg(not(feature = "full-file-path"))]
            assert_eq!(
                "parseint-error: invalid digit found in string at tests(139)",
                format!("{}", error.unwrap())
            );
        }

        #[cfg(windows)]
        {
            let is_my_error = matches!(error, Some(x) if matches!(&x, Error::ConvertInt(e) if e.error().kind()==target_kind));
            assert!(is_my_error);
            let error = biz_code("abc").err();

            #[cfg(feature = "full-file-path")]
            assert_eq!(
                "convertint-error: invalid digit found in string at src\\rhrl\\stdx\\ylong_one_error\\tests\\tests.rs(139/23)",
                format!("{}", error.unwrap())
            );
            #[cfg(not(feature = "full-file-path"))]
            assert_eq!(
                "convertint-error: invalid digit found in string at tests(139)",
                format!("{}", error.unwrap())
            );
        }
    }

    #[cfg(not(feature = "prefix-error"))]
    #[test]
    fn test_parse_error() {
        fn biz_code(value: &str) -> Result<i32, Error> {
            let num = value.parse::<i32>()?;
            Ok(num + 10)
        }

        assert_eq!(21, biz_code("11").unwrap());
        let error = biz_code("abc").err();
        let target_kind = &std::num::IntErrorKind::InvalidDigit;

        #[cfg(not(windows))]
        {
            let is_my_error = matches!(error, Some(x) if matches!(&x, Error::ParseInt(e) if e.error().kind()==target_kind));
            assert!(is_my_error);
            let error = biz_code("abc").err();

            #[cfg(feature = "full-file-path")]
            assert_eq!(
                "invalid digit found in string at src/rhrl/stdx/ylong_one_error/tests/tests.rs(188/23)",
                format!("{}", error.unwrap())
            );
            #[cfg(not(feature = "full-file-path"))]
            assert_eq!(
                "invalid digit found in string at tests(188)",
                format!("{}", error.unwrap())
            );
        }

        #[cfg(windows)]
        {
            let is_my_error = matches!(error, Some(x) if matches!(&x, Error::ConvertInt(e) if e.error().kind()==target_kind));
            assert!(is_my_error);
            let error = biz_code("abc").err();

            #[cfg(feature = "full-file-path")]
            assert_eq!(
                "invalid digit found in string at src\\rhrl\\stdx\\ylong_one_error\\tests\\tests.rs(188/23)",
                format!("{}", error.unwrap())
            );
            #[cfg(not(feature = "full-file-path"))]
            assert_eq!(
                "invalid digit found in string at tests(188)",
                format!("{}", error.unwrap())
            );
        }
    }
}
