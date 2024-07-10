#[derive(Debug)]
pub enum UtpType {
    StData,
    StFin,
    StState,
    StReset,
    StSyn
}

impl UtpType {

    pub fn from_value(value: &u8) -> Result<Self, ()> {
        for _type in [Self::StData, Self::StFin, Self::StState, Self::StReset, Self::StSyn] {
            if _type.value().eq(value) {
                return Ok(_type);
            }
        }

        Err(())
    }

    pub fn value(&self) -> u8 {
        match self {
            Self::StData => 0,
            Self::StFin => 1,
            Self::StState => 2,
            Self::StReset => 3,
            Self::StSyn => 4,
        }
    }
}
