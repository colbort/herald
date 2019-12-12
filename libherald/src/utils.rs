/// Strips `qrc` prefix from paths passed from QML.
pub fn strip_qrc(mut path: String) -> Option<String> {
    if path.len() < 7 {
        None
    } else {
        Some(path.split_off(7))
    }
}

pub(crate) fn err_string_msg(
    e: &dyn std::error::Error,
    file: &str,
    line: u32,
    msg: &'static str,
) -> String {
    format!(
        "{msg}: {error} at {file}:{line}",
        msg = msg,
        error = e,
        file = file,
        line = line,
    )
}

pub(crate) fn err_string(
    e: &dyn std::error::Error,
    file: &str,
    line: u32,
) -> String {
    format!(
        "{error} at {file}:{line}",
        error = e,
        file = file,
        line = line,
    )
}

#[macro_export]
/// Early return on error
macro_rules! err {
    ($maybe: expr) => {
        err!($maybe, ())
    };
    ($maybe: expr, $retval: expr) => {
        match $maybe {
            Ok(val) => val,
            Err(e) => {
                let err_string = crate::utils::err_string(&e, file!(), line!());

                eprintln!("{}", err_string);
                $crate::push(crate::Update::Error(err_string));
                return $retval;
            }
        }
    };
}

#[macro_export]
/// Continue on error
macro_rules! cont_err {
    ($maybe: expr) => {
        match $maybe {
            Ok(val) => val,
            Err(e) => {
                use $crate::shared::SingletonBus;
                let err_string = crate::utils::err_string(&e, file!(), line!());

                eprintln!("{}", err_string);
                $crate::push(crate::Update::Error(err_string)).ok();
                continue;
            }
        }
    };
}

#[macro_export]
/// If the value passed is an error, pushes an error to the error queue without an early return.
macro_rules! push_err {
    ($maybe: expr, $msg: expr) => {
        match $maybe {
            Ok(val) => Some(val),
            Err(e) => {
                let err_string = crate::utils::err_string_msg(&e, file!(), line!(), $msg);

                eprintln!("{}", err_string);
                $crate::push(Update::Error(err_string));
                None
            }
        }
    };
}

pub(crate) fn none_string(
    file: &str,
    line: u32,
) -> String {
    format!(
        "Unexpected `None` at {file}:{line}",
        file = file,
        line = line,
    )
}

#[macro_export]
/// Early return on unexpected `None`
macro_rules! none {
    ($maybe: expr) => {
        none!($maybe, ())
    };
    ($maybe: expr, $retval: expr) => {
        match $maybe {
            Some(val) => val,
            None => {
                let err_string = $crate::utils::none_string(file!(), line!());

                eprintln!("{}", err_string);
                $crate::push(crate::Update::Error(err_string));
                return $retval;
            }
        }
    };
}

#[macro_export]
/// Continue  on unexpected `None`
macro_rules! cont_none {
    ($maybe: expr) => {
        match $maybe {
            Some(val) => val,
            None => {
                let err_string = $crate::utils::none_string(file!(), line!());

                eprintln!("{}", err_string);
                $crate::push(crate::Update::Error(err_string));
                continue;
            }
        }
    };
}

#[macro_export]
/// Convenience macro for spawning thread
macro_rules! spawn {
    ($code: expr) => {
        spawn!($code, ())
    };
    ($code: expr, $retval: expr) => {
        err!(
            ::std::thread::Builder::new().spawn(move || { $code }),
            $retval
        )
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn strip_qrc() {
        let path = "file:///what/a/path".into();

        assert_eq!("/what/a/path", super::strip_qrc(path).unwrap());
    }
}
