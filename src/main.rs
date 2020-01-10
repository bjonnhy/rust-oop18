

#[derive(Debug)]
struct Player {
    id: usize,
    name: String,
    address: String
}

#[derive(Debug)]
struct Sport {
    name: String,
    divisions: Vec<Division>,
    table_type: TableType
}

#[derive(Debug)]
struct Division {
    name: String,
    teams: Vec<Team>,
    fixtures: Vec<Vec<GameResult>>
}

#[derive(Debug)]
enum TableType {
}

#[derive(Debug)]
struct Team {
    name: String,
    postal: String,
    players: Vec<usize>
}

const DATELEN: usize = 11;

#[derive(Debug)]
struct GameResult {
    date: [char; DATELEN]
}

fn main() {
    let mut players: Vec<Player>;
    let mut sports: Vec<Sport>;
    
}
