#[derive(Copy, Clone)]
pub enum ExitStatus {
    Ok,
    Err,
    SafeRm,
    NonSafeRm,
}
