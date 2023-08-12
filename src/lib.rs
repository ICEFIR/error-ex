/// Defining an error with set of reason.
/// # Usage
/// ```
/// create_error!(ErrorType => ErrorReason1, ErrorReason2, ErrorReason3)
/// ```
/// # Examples
///
/// ```
/// use error_ex::{create_error, map_to_error};
///
/// create_error!(InputError => IllegalArgument, InvalidInput, MissingArgument);
///
/// ```
///
/// Now, you can use the following code to instantiate this error
/// ```
/// InputError::IllegalArgument(format!("Your message here"))
/// ```
///
#[macro_export]
macro_rules! create_error {
    ( $error:ident => $( $error_reason:ident ),* ) => {
        #[allow(non_snake_case)]
        pub mod $error {
            pub use paste::paste;
            pub use std::error::Error;
            pub use std::fmt;

            paste! {
                #[derive(Debug, Clone, PartialEq, Eq)]
                pub struct [<$error:camel Ex>] {
                    pub reason: Reason,
                    pub message: String,
                }

                #[derive(Debug, Clone, PartialEq, Eq)]
                #[allow(dead_code)]
                pub enum Reason {
                    $(
                        [<$error_reason>],
                    )*
                }

                $(
                    #[allow(dead_code)]
                    pub fn [<$error_reason>](message: String) -> [<$error:camel Ex>] {
                        [<$error:camel Ex>] {
                            reason: Reason::$error_reason,
                            message,
                        }
                    }
                )*

                impl Error for [<$error:camel Ex>] {}

                impl fmt::Display for [<$error:camel Ex>] {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        let error_name = stringify!(self);
                        write!(
                            f,
                            "{:?} error, reason: {:?}, message {:?}",
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
/// let asset = fs::read_to_string(path).map_err(|error| {
/// SchemaError::IoError(format!("SchemaError::IoError cased by {error}"))
/// })?;
/// ```

/// The above code can be simplified using `map_to_error!` macro
/// ```rust
/// let result: Result<(), Error> = Err(Error("Test".to_string()));
/// let mapped_result = result.map_err(map_to_error!(InputError::IllegalArgument));
/// ```
///
#[macro_export]
macro_rules! map_to_error {
    ($error:ident :: $error_reason:ident) => {
        |error| {
            let error_name = stringify!($error);
            let error_reason_name = stringify!($error_reason);
            let error_string = format!(
                "{}::{} caused by {}",
                { error_name },
                { error_reason_name },
                { error }
            );
            $error::$error_reason(error_string)
        }
    };
}
