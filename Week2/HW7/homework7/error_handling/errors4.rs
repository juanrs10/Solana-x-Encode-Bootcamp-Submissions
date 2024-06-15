#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative(i64),
    Zero(i64),
}

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
        if value < 0 {
            Err(CreationError::Negative(value))
        } else if value == 0 {
            Err(CreationError::Zero(value))
        } else {
            Ok(PositiveNonzeroInteger(value as u64))
        }
    }
}

#[test]
fn test_creation() {
    assert!(PositiveNonzeroInteger::new(10).is_ok());
    assert_eq!(
        Err(CreationError::Negative(-10)),
        PositiveNonzeroInteger::new(-10)
    );
    assert_eq!(Err(CreationError::Zero(0)), PositiveNonzeroInteger::new(0));
}
