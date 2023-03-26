// errors4.rs
// Execute `rustlings hint errors4` or use the `hint` watch subcommand for a hint.

#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative(String),
    Zero(String),
}

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
        // Hmm...? Why is this only returning an Ok value?
        match value {
            v if v < 0 => Err(CreationError::Negative(format!(
                "Error creating PositiveNonzeroInteger, negative value of \"{}\" is not allowed.",
                v
            ))),
            0 => Err(CreationError::Zero(format!(
                "Error creating PositiveNonzeroInteger, zero value of 0 is not allowed."
            ))),
            _ => Ok(PositiveNonzeroInteger(value as u64)),
        }
    }
}

#[test]
fn test_creation() {
    assert!(PositiveNonzeroInteger::new(10).is_ok());
    assert_eq!(
        Err(CreationError::Negative(
            "Error creating PositiveNonzeroInteger, negative value of \"-10\" is not allowed."
                .to_string()
        )),
        PositiveNonzeroInteger::new(-10)
    );
    assert_eq!(
        Err(CreationError::Zero(
            "Error creating PositiveNonzeroInteger, zero value of 0 is not allowed.".to_string()
        )),
        PositiveNonzeroInteger::new(0)
    );
}
