mod module;
mod statement;

pub use module::*;
pub use statement::*;

use super::Error;

pub(crate) trait Walker<T> {
    fn walk(&mut self, params: T) -> Result<(), Error>;
}

impl<T, U> Walker<Box<T>> for U
where
    T: Clone,
    U: Walker<T>,
{
    fn walk(&mut self, params: Box<T>) -> Result<(), Error> {
        self.walk(*params)
    }
}
impl<T, U> Walker<&'_ T> for U
where
    T: Clone,
    U: Walker<T>,
{
    fn walk(&mut self, params: &T) -> Result<(), Error> {
        self.walk(params.clone())
    }
}
