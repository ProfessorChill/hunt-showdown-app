#[derive(Debug, Clone, PartialEq, Copy)]
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
            ToolSlotPreference::NoPreference => "NoPreference".to_string(),
            ToolSlotPreference::Medkit => "Medkit".to_string(),
            ToolSlotPreference::Melee => "Melee".to_string(),
            ToolSlotPreference::Throwable => "Throwable".to_string(),
            ToolSlotPreference::Tripmines => "Tripmines".to_string(),
            ToolSlotPreference::Decoys => "Decoys".to_string(),
            ToolSlotPreference::Others => "Others".to_string(),
        }
    }
}

impl From<String> for ToolSlotPreference {
    fn from(val: String) -> Self {
        match &*val {
            "NoPreference" => ToolSlotPreference::NoPreference,
            "Medkit" => ToolSlotPreference::Medkit,
            "Melee" => ToolSlotPreference::Melee,
            "Throwable" => ToolSlotPreference::Throwable,
            "Tripmines" => ToolSlotPreference::Tripmines,
            "Decoys" => ToolSlotPreference::Decoys,
            "Others" => ToolSlotPreference::Others,
            _ => panic!("Invalid tool slot preference from String: {}", val),
        }
    }
}
