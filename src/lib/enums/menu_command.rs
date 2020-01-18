use super::MenuCommand;

impl MenuCommand {
    
    pub fn from_str(s: &str) -> MenuCommand {
        match s {
            "P" => MenuCommand::P,
            "S" => MenuCommand::S,
            "C" => MenuCommand::C,
            "D" => MenuCommand::D,
            "F" => MenuCommand::F,
            "M" => MenuCommand::M,
            "T" => MenuCommand::T,
            "R" => MenuCommand::R,
            "I" => MenuCommand::I,
            "E" => MenuCommand::E,
            "L" => MenuCommand::L,
            "Q" => MenuCommand::Q,
            "A" => MenuCommand::A,
            _ => MenuCommand::U
        }
    }

    pub fn from_string(s: &String) -> MenuCommand {
        MenuCommand::from_str(&s.as_str())
    }
}