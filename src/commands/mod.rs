use crate::models::error::Error;

pub(crate) mod list;
pub(crate) mod add;

pub(crate) trait CommandExecutor {
    fn execute(&self) -> Result<(), Error>;
}
