use std::io;

#[derive(Debug, PartialEq, Eq)]
pub enum UtpTypes {
    Data,
    Fin,
    Ack, //Also known as State
    Reset,
    Syn
}

impl UtpTypes {

    pub fn from_code(value: &u8) -> io::Result<Self> {
        for _type in [Self::Data, Self::Fin, Self::Ack, Self::Reset, Self::Syn] {
            if _type.get_code().eq(value) {
                return Ok(_type);
            }
        }

        Err(io::Error::new(io::ErrorKind::InvalidInput, "Unknown UTP type code"))
    }

    pub fn get_code(&self) -> u8 {
        match self {
            Self::Data => 0,
            Self::Fin => 1,
            Self::Ack => 2,
            Self::Reset => 3,
            Self::Syn => 4,
        }
    }
}
