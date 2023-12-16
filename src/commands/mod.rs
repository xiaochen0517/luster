use crate::models::error::Error;

pub(crate) mod add;
pub(crate) mod list;
pub(crate) mod done;

pub(crate) trait CommandExecutor {
    fn execute(&self) -> Result<(), Error>;
}
