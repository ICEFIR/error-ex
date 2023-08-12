#[cfg(test)]
mod tests {
    use error_ex::{create_error, map_to_error};
    // Given
    create_error!(InputError => IllegalArgument, InvalidInput, MissingArgument);
    create_error!(ResponseError => ParamError);

    #[test]
    fn should_able_to_construct_error() {
        // when
        let invalid_input = InputError::InvalidInput(format!("Test"));

        let illegal_argument = InputError::IllegalArgument(format!("Test 2"));

        let missing_argument = InputError::Error {
            reason: InputError::Reason::MissingArgument,
            message: "Test 3".to_string(),
        };

        // then
        assert_eq!(invalid_input.reason, InputError::Reason::InvalidInput);
        assert_eq!(invalid_input.message, "Test");
        assert_eq!(illegal_argument.reason, InputError::Reason::IllegalArgument);
        assert_eq!(illegal_argument.message, "Test 2");
        assert_eq!(missing_argument.reason, InputError::Reason::MissingArgument);
        assert_eq!(missing_argument.message, "Test 3");
    }

    #[test]
    fn should_able_to_map_an_error_to_another_error() {
        // given
        let res: Result<(), InputError::Error> = Err(InputError::InvalidInput(format!("Test")));

        // when
        let mapped_res = res.map_err(map_to_error!(ResponseError::ParamError));

        // then
        assert_eq!(mapped_res.is_err(), true);
        let error = mapped_res.unwrap_err();
        assert_eq!(error.reason, ResponseError::Reason::ParamError);
        assert_eq!(
            error.message,
            format!(
                "ResponseError::ParamError caused by \
                \"InputError\" error, reason: InvalidInput, message \"Test\""
            )
        )
    }
}
