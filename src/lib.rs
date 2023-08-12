/// Defining an error with set of reason.
/// # Usage
/// ```
/// use error_ex::{create_error};
/// create_error!(ErrorType => ErrorReason1, ErrorReason2, ErrorReason3);
/// ```
/// ## Examples
///
/// ```
/// use error_ex::{create_error};
///
/// create_error!(InputError => IllegalArgument, InvalidInput, MissingArgument);
/// //Now, you can use the following code to instantiate this error
/// InputError::illegal_argument(format!("Your message here"));
///
/// ```
///
/// ## Error Mapping
///
/// The explicit way
/// ```
/// use std::fs;
/// use error_ex::{create_error};
///
/// create_error!(FileError => IoError);
/// create_error!(SchemaError => ParseError);
///
/// let error: Result<(), FileError> = Err(FileError::io_error("".to_string()));
///
/// let mapped = error.map_err(|error| {
///     SchemaError::parse_error(format!("SchemaError::ParseError error {error}"))
/// });
///
/// ```
/// ## Function wrapper
/// The above code can be simplified using `map_to_error!`
/// macro using the build in lower order function
/// ```
///
/// use std::fs;
/// use std::io::Error;
/// use error_ex::{create_error};
///
/// create_error!(FileError => IoError);
/// create_error!(SchemaError => ParseError);
///
/// let error: Result<(), FileError> = Err(FileError::io_error("".to_string()));
///
/// let mapped = error.map_err(SchemaError::map_parse_error);
///
/// ```
///
#[macro_export]
macro_rules! create_error {
    ( $error:ident => $( $error_reason:ident ),* ) => {
        paste::paste! {

            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct $error {
                pub reason: [<$error Reason>],
                pub message: String,
            }

            #[derive(Debug, Clone, PartialEq, Eq)]
            #[allow(unused_qualifications)]
            pub enum [<$error Reason>] {
                $(
                    [<$error_reason>],
                )*
            }

            impl $error {
                $(
                    #[allow(unused_qualifications)]
                    pub fn [<$error_reason:snake>](message: String) -> $error {
                        $error {
                            reason: [<$error Reason>]::$error_reason,
                            message,
                        }
                    }
                )*

                $(
                    #[allow(unused_qualifications)]
                    pub fn [<map_ $error_reason:snake>]<E>(error: E) -> $error
                    where
                        E: std::error::Error,
                    {
                        let error_name = stringify!($error);
                        let error_reason_name = stringify!($error_reason);
                        let error_string =
                            format!("{}::{} caused by {}", error_name, error_reason_name, error);
                        $error {
                            reason: [<$error Reason>]::$error_reason,
                            message: error_string,
                        }
                    }
                )*
            }


            impl std::error::Error for $error {}

            impl std::fmt::Display for $error {
                fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
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
    };
}
