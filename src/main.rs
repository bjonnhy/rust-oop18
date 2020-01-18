use std::{io, thread, time};


#[derive(Debug)]
struct Player {
    id: usize,
    name: String,
    address: String
}

impl Player {
    fn print(&self) {        
        println!(" <-- Player -->");
        println!("Name    : {}", self.name);
        println!("Id      : {}", self.id);
        println!("Address : {}", self.address);
    }
}

#[derive(Debug)]
struct Players {
    player_list: Vec<Player>,
    last_player: usize
}

impl Players {
    fn add_player(&mut self, mut player: Player) {
        self.last_player += 1;
        player.id = self.last_player;
        self.player_list.push(player);
    }
}

#[derive(Debug)]
struct Sport {
    name: String,
    divisions: Vec<Division>,
    table_type: TableType
}

impl Sport {
    fn print(&self) {
        println!(" <-- Sport -->");
        println!("Name: {}", self.name);
        println!("Divisions: {}", self.divisions.len());
        println!("Tabletype: {:?}", self.table_type);
    }
}

#[derive(Debug)]
struct Sports {
    sports_list: Vec<Sport>
}

impl Sports {

    fn add_sport(&mut self, sport: Sport) {
        self.sports_list.push(sport);
    }
}

#[derive(Debug)]
struct Division {
    name: String,
    teams: Vec<Team>,
    fixtures: Vec<Vec<GameResult>>
}

#[derive(PartialEq, Debug)]
enum TableType {
    ThreePointTable,
    TwoPointTable,
    Unknown
}

impl TableType {

    fn from_string(s: &str) -> TableType {
        match s.to_uppercase().as_str() {
            "THREEPOINTTABLE" => TableType::ThreePointTable,
            "TWOPOINTTABLE" => TableType::TwoPointTable
            _ => TableType::Unknown
        }
    }
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
    date: [char; DATELEN],
    home_scorers: Vec<usize>,
    away_scorers: Vec<usize>,
    is_ordinary_finish: bool
}

#[derive(PartialEq, Debug)]
enum MenuCommand {
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

impl MenuCommand {
    
    fn from_str(s: &str) -> MenuCommand {
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

    fn from_string(s: &String) -> MenuCommand {
        MenuCommand::from_str(&s.as_str())
    }
}

fn main() {

    let mut players = Players { player_list: Vec::new(), last_player: 0 };
    let mut sports = Sports { sports_list: Vec::new() };

    loop {
        print_menu();
        let read_command = read_string().to_uppercase();
        let command = MenuCommand::from_string(&read_command);
        let is_quit = command == MenuCommand::Q;

        if is_quit {
            println!("Bye");
            break;
        } else {
            match &command {
                MenuCommand::P => handle_print_players(&players),
                MenuCommand::S => println!("NOT IMPLEMENTED"),
                MenuCommand::C => {
                    let (mut p, mut s) = handle_create(players, sports);
                    players = p;
                    sports = s;
                },
                MenuCommand::D => println!("NOT IMPLEMENTED"),
                MenuCommand::F => println!("NOT IMPLEMENTED"),
                MenuCommand::M => println!("NOT IMPLEMENTED"),
                MenuCommand::T => println!("NOT IMPLEMENTED"),
                MenuCommand::R => println!("NOT IMPLEMENTED"),
                MenuCommand::I => println!("NOT IMPLEMENTED"),
                MenuCommand::E => println!("NOT IMPLEMENTED"),
                MenuCommand::L => println!("NOT IMPLEMENTED"),
                _ => error_response(&read_command),
            };
        }
    } 
}


fn print_menu() {
    println!("                   <---  Sport Management System  --->");
    divider();
    println!("  P -> A | <num> | <name> : Print (A)ll players, player <num> or player <name>");
    println!("  S -> A | <navn>         : Print (A)ll sports or sport <name>");
    println!("  C -> P | S | D          : Prompt (C)reation of (P)layer, (S)port or (D)ivision");
    println!("  D -> P | S | D          : Primpt (D)eletion of (P)layer, (S)port or (D)ivision");
    println!("  F                       : Print (F)ixtures");
    println!("  M                       : Print results of (M)atches");
    println!("  T                       : Print (T)abels");
    println!("  R                       : (R)ead from file");
    println!("  I                       : Print players");
    println!("  E                       : (E)dit team");
    println!("  L                       : Print top (L)ists");
    println!("  Q                       : (Q)uit");
    divider();
    println!("Input command: ");
}

fn divider() {
    println!("********************************************************************************");
}

fn error_response(read_command: &str) {
    println!("Invalid command: {}", read_command);
    // wait a bit so the user ses the errormessage
    thread::sleep(time::Duration::from_millis(1000));
}

fn read_string() -> String {
    let mut read_str = String::new();
    io::stdin().read_line(&mut read_str);
    String::from(read_str.trim())
}

fn valid_name(name: &str) -> bool {
    name.chars().all(char::is_alphabetic)
}

fn valid_number(number: &str) -> bool {
    number.chars().all(char::is_numeric)
}

fn valid_address(address: &str) -> bool {
    true
}

fn read_number(min: u16, max: u16) -> usize {
    0
}

fn handle_create(mut players: Players, mut sports: Sports) -> (Players, Sports) {
    println!("");
    println!("Type ? (P)layer, (S)port, (D)ivision:");
    let read_command = read_string().to_uppercase();
    let command = MenuCommand::from_string(&read_command);

    match &command {
        MenuCommand::P => {
            players.add_player(create_player());
            (players, sports)
        },
        MenuCommand::S => {
            sports.add_sport(create_sport());
            (players, sports)
        },
        MenuCommand::D => {
            println!("Create new division -- NOT IMPLEMENTED");
            (players, sports)
        },
        _ => {
            error_response(&read_command);
            (players, sports)
        }
    }
}

fn create_player() -> Player {
    loop {
        println!("Please input the players name: ");
        let name = read_string();

        println!("Please input the players address");
        let address = read_string();
        
        if valid_name(name.as_str()) && valid_address(address.as_str()) {
            return Player {
                    id: 0,
                    name: name,
                    address: address
                }
        } else {
            println!("Unvalid input. Ensure name [a-zA-Z] and address [?] are correct")
        }
    }
}

fn create_sport() -> Sport {
    loop {
        println!("Please input the sports name");
        let name = read_string();

        print!("Decide tabletype: ");
        


        let table_type = TableType::from_string(read_string().as_str());

        if valid_name(name.as_str()) && table_type != TableType::Unknown {
            return Sport {
                name: name,
                divisions: Vec<Division>::new(),
                table_type:  table_type
            }
        }
    }
}

fn handle_print_players(players: &Players) {
    loop {
        println!("Print (A)ll, player <nr> or player <name>: ");
        let input = read_string();
    
        if MenuCommand::from_string(&input.to_uppercase()) == MenuCommand::A {
            for player in &players.player_list {
                player.print();
            }

        } else if valid_name(&input) {
            let some_p = players.player_list.iter().find(|&player| player.name == input);
            match some_p {
                Some(p) => p.print(),
                None => println!("No player with given name")
            }

        } else if valid_number(&input) {
            let number_from_str: usize = input.parse().unwrap();

            let some_p = players.player_list.iter().find(|&player| player.id == number_from_str);
            match some_p {
                Some(p) => p.print(),
                None => println!("No player with given id")
            }
        } else {
            println!("Not a valid input");
        }
        break;
    }
}