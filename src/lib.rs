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
