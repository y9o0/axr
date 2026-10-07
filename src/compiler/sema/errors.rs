use super::*;

#[derive(Debug)]
pub enum SemaErr {
    TypeMismatch,
    Unexpected,
    CantAssign,
    InvalidTarget,
    StackCorruption,
}

pub type SemaResult<T> = std::result::Result<T, SemaErr>;

impl Sema {
    pub fn error(message: &str, err_type: SemaErr) -> SemaErr {
        eprintln!("{}", message);
        err_type
    }
}
