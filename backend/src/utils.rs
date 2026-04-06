use anyhow::{Error};
use log::error;

pub fn log_and_context_error<E>(err: E, msg: &str, file: &str, function: &str) -> Error
where
    E: std::error::Error + Send + Sync + 'static,
{
    error!("{}: {}. At {}::{}", msg, err, file, function);
    Error::new(err).context(msg.to_string())
}


#[macro_export]
macro_rules! create_error {
    ($($arg:tt)*) => {
        Err(anyhow::anyhow!($($arg)*))
    }
}