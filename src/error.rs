use std::error::Error;

#[derive(Error, Debug)]
pub enum SarmioError {
    #[error("machine_id can not be negative!")]
    NegativeMachineId,

    #[error("system time before unix epoch")]
    BrokenSystemTime,
}
