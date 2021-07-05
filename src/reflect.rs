use std::{convert::TryInto, marker::PhantomData};

use crate::{error::AbsorbError, value::Message};

pub trait Reflect: Sized {
    fn reflect(self) -> Reflection<Self>;
}

pub struct Reflection<T> {
    message: Message,
    _marker: PhantomData<T>,
}

impl<T> Reflection<T>
where
    T: Into<Message>,
{
    pub fn new(concrete: T) -> Self {
        Reflection {
            message: concrete.into(),
            _marker: PhantomData,
        }
    }
}

impl<T> Reflection<T> {
    pub fn absorb(self) -> Result<T, AbsorbError>
    where
        Message: TryInto<T, Error = AbsorbError>,
    {
        self.message.try_into()
    }
}

impl<T> Reflect for T
where
    T: Into<Message>,
{
    fn reflect(self) -> Reflection<Self> {
        Reflection::new(self)
    }
}
