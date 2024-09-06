#[derive(Debug, PartialEq, Eq)]
pub enum UtpType {
    Data,
    Fin,
    Ack, //Also known as State
    Reset,
    Syn
}

impl UtpType {

    pub fn from_value(value: &u8) -> Result<Self, ()> {
        for _type in [Self::Data, Self::Fin, Self::Ack, Self::Reset, Self::Syn] {
            if _type.value().eq(value) {
                return Ok(_type);
            }
        }

        Err(())
    }

    pub fn value(&self) -> u8 {
        match self {
            Self::Data => 0,
            Self::Fin => 1,
            Self::Ack => 2,
            Self::Reset => 3,
            Self::Syn => 4,
        }
    }
}
