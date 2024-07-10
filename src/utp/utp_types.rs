pub enum UtpTypes {
    StData,
    StFin,
    StState,
    StReset,
    StSyn
}

impl UtpTypes {

    pub fn from_value(value: &u32) -> Result<Self, ()> {
        for _type in [Self::StData, Self::StFin, Self::StState, Self::StReset, Self::StSyn] {
            if _type.value().eq(value) {
                return Ok(_type);
            }
        }

        Err(())
    }

    pub fn value(&self) -> u32 {
        match self {
            Self::StData => 0,
            Self::StFin => 1,
            Self::StState => 2,
            Self::StReset => 3,
            Self::StSyn => 4,
        }
    }
}
