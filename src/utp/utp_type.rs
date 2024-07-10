#[derive(Debug)]
pub enum UtpType {
    Data,
    Fin,
    State,
    Reset,
    Syn
}

impl UtpType {

    pub fn from_value(value: &u8) -> Result<Self, ()> {
        for _type in [Self::Data, Self::Fin, Self::State, Self::Reset, Self::Syn] {
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
            Self::State => 2,
            Self::Reset => 3,
            Self::Syn => 4,
        }
    }
}
