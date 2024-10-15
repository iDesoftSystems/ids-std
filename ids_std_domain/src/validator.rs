use validator::{Validate, ValidationErrors};

pub fn try_validate<T>(params: &T) -> Result<(), ValidationErrors>
where
    T: Validate,
{
    match params.validate() {
        Ok(()) => Ok(()),
        Err(err) => Err(err),
    }
}

#[cfg(test)]
mod tests {
    use validator::Validate;

    use crate::validator;

    #[derive(Validate, Debug)]
    struct ValidateMe {
        #[validate(length(min = 1))]
        pub name: String,
    }

    #[test]
    fn it_pass_validation() {
        let to_validate = ValidateMe {
            name: "Hello".to_string(),
        };
        let result = validator::try_validate(&to_validate);
        assert!(result.is_ok())
    }

    #[test]
    fn it_failed_validation() {
        let to_validate = ValidateMe {
            name: "".to_string(),
        };
        let result = validator::try_validate(&to_validate);

        assert!(result.is_err())
    }
}
