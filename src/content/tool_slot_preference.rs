#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum ToolSlotPreference {
    NoPreference,
    Medkit,
    Melee,
    Throwable,
    Tripmines,
    Decoys,
    Others,
}

impl ToString for ToolSlotPreference {
    fn to_string(&self) -> String {
        match self {
            Self::NoPreference => "NoPreference".to_string(),
            Self::Medkit => "Medkit".to_string(),
            Self::Melee => "Melee".to_string(),
            Self::Throwable => "Throwable".to_string(),
            Self::Tripmines => "Tripmines".to_string(),
            Self::Decoys => "Decoys".to_string(),
            Self::Others => "Others".to_string(),
        }
    }
}

impl TryFrom<String> for ToolSlotPreference {
    type Error = &'static str;

    fn try_from(val: String) -> Result<Self, Self::Error> {
        match &*val {
            "NoPreference" => Ok(Self::NoPreference),
            "Medkit" => Ok(Self::Medkit),
            "Melee" => Ok(Self::Melee),
            "Throwable" => Ok(Self::Throwable),
            "Tripmines" => Ok(Self::Tripmines),
            "Decoys" => Ok(Self::Decoys),
            "Others" => Ok(Self::Others),
            _ => Err("Invalid tool slot preference"),
        }
    }
}
