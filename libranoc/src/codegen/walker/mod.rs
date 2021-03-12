mod module;
mod statement;

pub use module::*;
pub use statement::*;

use super::Error;

pub(crate) trait Walker<T> {
    fn walk(&mut self, params: T) -> Result<(), Error>;
}
