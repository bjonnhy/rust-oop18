use super::TableType;

impl TableType {
    pub fn from_string(s: &str) -> TableType {
        match s.to_uppercase().as_str() {
            "THREEPOINTTABLE" => TableType::ThreePointTable,
            "3PT" => TableType::ThreePointTable,
            "TWOPOINTTABLE" => TableType::TwoPointTable,
            "2PT" => TableType::TwoPointTable,
            _ => TableType::Unknown
        }
    }
}