#[derive(Copy, Clone)]
pub enum ExitStatus {
    SafeRm,
    NonSafeRm,
    Err,
}
