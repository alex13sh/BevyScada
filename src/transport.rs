use bevy::prelude::*;
use derivative::Derivative;
use serde::{Serialize, Deserialize, de::DeserializeOwned};

pub type Address = u16;
pub type Register = u32;

pub type TarnsportDynamic = dynamic::Dynamic;
#[derive(Component)]
#[derive(Serialize, Deserialize)]
#[derive(Derivative)]
#[derivative(Debug)]
pub struct Transport<T: TransportExt> {
    config: T::Config,
    #[derivative(Debug="ignore")]
    #[serde(skip)]
    context: Option<T::Context>,
    #[serde(skip)]
    errors: Vec<<Self as TransportContext>::Error>,
}

pub trait TransportExt: Sync + Send + 'static {
    type Config: core::fmt::Debug + Sync + Send + Serialize + DeserializeOwned;
    type Context: TransportContext + Sync + Send;
    // type Error = Self::Context::Error;
}

pub trait TransportContext {
    type Error: core::fmt::Debug + Sync + Send ;
    fn read(&mut self, adr: Address) -> Result<Register, Self::Error>;
    fn write(&mut self, adr: Address, reg: Register) -> Result<(), Self::Error>;
}

impl <T: TransportExt> Transport<T> {
    pub fn last_error(&self) -> Option<&<Self as TransportContext>::Error> {
        self.errors.last()
    }
}

#[derive(Debug)]
pub enum Error<T: core::fmt::Debug> {
    Disconnect,
    Context(T),
}
impl <T: TransportExt> TransportContext for Transport<T> {
    type Error = Error<<T::Context as TransportContext>::Error>; //T::Context::Error;
    fn read(&mut self, adr: Address) -> Result<Register, Self::Error> {
        if let Some(context) = &mut self.context {
            Ok(context.read(adr)
                .map_err(|e| Error::Context(e))?)
        } else {
            Err(Error::Disconnect)
        }
    }
    fn write(&mut self, adr: Address, reg: Register) -> Result<(), Self::Error> {
        if let Some(context) = &mut self.context {
            Ok(context.write(adr, reg)
                .map_err(|e| Error::Context(e))?)
        } else {
            Err(Error::Disconnect)
        }
    }
}

impl TransportExt for () {
    type Config = ();
    type Context = ();
    // type Error = ();
}

impl TransportContext for () {
    type Error = ();
    fn read(&mut self, _adr: Address) -> Result<Register, Self::Error> {
        Err(())
    }
    fn write(&mut self, _adr: Address, _reg: Register) -> Result<(), Self::Error> {
        Err(())
    }
}

impl From<()> for Transport<()> {
    fn from(():()) -> Self {
        Transport { config: (), context: None, errors: vec![] }
    }
}

pub  mod dynamic {
    use super::{
        TransportExt,
        TransportContext
    };

    use serde::{Serialize, Deserialize};
    #[derive(Default)]
    pub enum Dynamic {
        // mqtt, modbus, canbus,
        #[default]
        None,
    }

    #[derive(Serialize, Deserialize)]
    #[derive(Debug)]
    pub enum Config {
        // for modbus, canbus ...
    }
    pub enum Context {
        // for modbus, canbus
    }

    #[derive(Debug)]
    pub enum Error {
        None,
    }

    impl TransportExt for Dynamic {
        type Config = Config;
        type Context = Context;
    }

    impl TransportContext for Context {
        type Error = Error;
        fn read(&mut self, _adr: super::Address) -> Result<super::Register, Self::Error> {
            // Err(Error::None)
            unimplemented!()
        }
        fn write(&mut self, _adr: super::Address, _reg: super::Register) -> Result<(), Self::Error> {
            unimplemented!()
        }
    }
}