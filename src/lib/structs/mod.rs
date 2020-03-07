use super::enums::TableType;

#[derive(Debug)]
pub struct Players {
    pub player_list: Vec<Player>,
    pub last_player: usize
}

#[derive(Debug)]
pub struct Player {
    pub id: usize,
    pub name: String,
    pub address: String
}

#[derive(Debug)]
pub struct Sports {
    pub sports_list: Vec<Sport>
}

#[derive(Debug)]
pub struct Sport {
    pub name: String,
    pub divisions: Vec<Division>,
    pub table_type: TableType
}

pub const MAX_TEAMS: usize = 50;

pub struct Division {
    _last_team: u8,
    pub name: String,
    pub teams: [Option<Team>; MAX_TEAMS],
    pub fixtures: [[Option<GameResult>; MAX_TEAMS]; MAX_TEAMS]

    /*
     * / - - - - - -- - - - - - \
     * |  home |  away  | date  |
     * + - - - - - -- - - - - - +
     * |     0 |      0 |     - | // no game against self
     * |     0 |      1 | 01/11 |
     * |     0 |      2 | 01/18 |
     * |   ... |    ... |   ... |
     * |     1 |      0 | 01/11 | // no game against self
     * + - - - - - -- - - - - - +
     * |        ROUND XX        |
     * + - - - - - -- - - - - - +
     * |     N |      M |   ... |
     * \ - - - - - -- - - - - - /
     */
}

#[derive(Debug)]
pub struct Team {
    pub name: String,
    pub postal: String,
    pub players: Vec<usize>
}


pub const DATELEN: usize = 11;

#[derive(Debug)]
pub struct GameResult {
    pub date: [char; DATELEN],
    pub home_scorers: Vec<u8>,
    pub away_scorers: Vec<u8>,
    pub is_ordinary_finish: bool
}


pub mod player_impl;
pub mod sport_impl;
pub mod division_impl;