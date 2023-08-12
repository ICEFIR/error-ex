/// Defining an error with set of reason.
/// # Usage
/// ```
/// use error_ex::{create_error};
/// create_error!(ErrorType => ErrorReason1, ErrorReason2, ErrorReason3);
/// ```
/// # Examples
///
/// ```
/// use error_ex::{create_error, map_to_error};
///
/// create_error!(InputError => IllegalArgument, InvalidInput, MissingArgument);
/// //Now, you can use the following code to instantiate this error
/// InputError::IllegalArgument(format!("Your message here"));
///
/// ```
///
#[macro_export]
macro_rules! create_error {
    ( $error:ident => $( $error_reason:ident ),* ) => {
        #[allow(non_snake_case)]
        pub mod $error {
            pub use paste::paste;
            pub use std::fmt;

            paste! {
                #[derive(Debug, Clone, PartialEq, Eq)]
                pub struct Error {
                    pub reason: Reason,
                    pub message: String,
                }

                #[derive(Debug, Clone, PartialEq, Eq)]
                #[allow(unused_qualifications)]
                pub enum Reason {
                    $(
                        [<$error_reason>],
                    )*
                }

                $(
                    #[allow(unused_qualifications)]
                    pub fn [<$error_reason>](message: String) -> Error {
                        Error {
                            reason: Reason::$error_reason,
                            message,
                        }
                    }
                )*

                impl std::error::Error for Error {}

                impl fmt::Display for Error {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        let error_name = stringify!([<$error:camel>]);
                        write!(
                            f,
                            "{:#?} error, reason: {:#?}, message {:#?}",
                            error_name,
                            self.reason,
                            self.message
                        )
                    }
                }
            }
        }
    };
}

/// Helper for mapping errors from one to another
/// # Usage
/// ### Error Mapping
///
/// The explicit way
/// ```
/// use std::fs;
/// use error_ex::{create_error, map_to_error};
/// create_error!(FileError => IoError);
/// create_error!(SchemaError => ParseError);
/// let error: Result<(), FileError::Error> = Err(FileError::IoError("".to_string()));
/// let mapped = error.map_err(|error| {
///     SchemaError::ParseError(format!("SchemaError::ParseError error {error}"))
/// });
///
/// ```
/// ### Macro error mapping
/// The above code can be simplified using `map_to_error!` macro
/// ```
///
/// use std::fs;
/// use std::io::Error;
/// use error_ex::{create_error, map_to_error};
/// create_error!(FileError => IoError);
/// create_error!(SchemaError => ParseError);
/// let error: Result<(), FileError::Error> = Err(FileError::IoError("".to_string()));
/// let mapped = error.map_err(map_to_error!(SchemaError::ParseError));
///
/// ```
///
#[macro_export]
macro_rules! map_to_error {
    ($error:ident :: $error_reason:ident) => {
        |error| {
            let error_name = stringify!($error);
            let error_reason_name = stringify!($error_reason);
            let error_string = format!("{}::{} caused by {}", error_name, error_reason_name, error);
            $error::$error_reason(error_string)
        }
    };
}
