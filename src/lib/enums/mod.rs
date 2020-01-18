#[derive(PartialEq, Debug)]
pub enum TableType {
    ThreePointTable,
    TwoPointTable,
    Unknown
}

#[derive(PartialEq, Debug)]
pub enum MenuCommand {
    P,
    S,
    C,
    D,
    F,
    M,
    T,
    R,
    I,
    E,
    L,
    Q,
    U,
    A,
}

pub mod table_type;
pub mod menu_command;