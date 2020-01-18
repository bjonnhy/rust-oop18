
// module lib for all the main components for the program
// and some utilities for interacting with the command line (current userinterface)
mod lib;
use lib::structs::{Players, Sports, player_impl, sport_impl, division_impl};
use lib::enums::MenuCommand;
use lib::util::*;

fn main() {

    // main state
    let mut players = Players { player_list: Vec::new(), last_player: 0 };
    let mut sports = Sports { sports_list: Vec::new() };

    // THE LOOP
    loop {
        write::print_menu();
        let read_command = read::read_string().to_uppercase();
        let command = MenuCommand::from_string(&read_command);
        let is_quit = command == MenuCommand::Q;

        if is_quit {
            println!("Bye");
            break;
        } else {
            match &command {
                MenuCommand::P => write::handle_print_players(&players), // print players
                MenuCommand::S => write::handle_print_sports(&sports),   // print sports
                MenuCommand::C => {                                      // creation through UI
                    let (p, s) = handle_create(players, sports);
                    players = p;
                    sports = s;
                },
                MenuCommand::D => {                                      // deletion through UI
                    let (p, s) = handle_delete(players, sports);
                    players = p;
                    sports = s;
                },
                MenuCommand::F => write::handle_print_fixtures(&sports), // print fixtures
                MenuCommand::M => println!("NOT IMPLEMENTED"),           // print results
                MenuCommand::T => println!("NOT IMPLEMENTED"),           // print tables
                MenuCommand::R => println!("NOT IMPLEMENTED"),           // read files
                MenuCommand::I => println!("NOT IMPLEMENTED"),           // print player info
                MenuCommand::E => println!("NOT IMPLEMENTED"),           // edit team
                MenuCommand::L => println!("NOT IMPLEMENTED"),           // print top list
                _ => write::error_response(&read_command),
            };
        }
    } 
}


fn handle_create(mut players: Players, mut sports: Sports) -> (Players, Sports) {
    println!("");
    println!("Type ? (P)layer, (S)port, (D)ivision:");
    let read_command = read::read_string().to_uppercase();
    let command = MenuCommand::from_string(&read_command);

    match &command {
        MenuCommand::P => {
            players.add_player(player_impl::create_player());
            (players, sports)
        },
        MenuCommand::S => {
            sports.add_sport(sport_impl::create_sport());
            (players, sports)
        },
        MenuCommand::D => {
            let s = division_impl::create_division(sports);
            sports = s;
            (players, sports)
        },
        _ => {
            write::error_response(&read_command);
            (players, sports)
        }
    }
}

fn handle_delete(mut players: Players, mut sports: Sports) -> (Players, Sports) {
    println!("");
    println!("Type ? (P)layer, (S)port, (D)ivision:");
    let read_command = read::read_string().to_uppercase();
    let command = MenuCommand::from_string(&read_command);

    match &command {
        MenuCommand::P => {
            println!("Name of player to delete:");
            let input = read::read_string();
            match players.remove(&input) {
                Some(player) => println!("Removed player {:?}", player),
                None => println!("No player called {}", &input)
            }
            (players, sports)
        },
        MenuCommand::S => {
            println!("Name of the sport to delete");
            let input = read::read_string();
            match sports.remove(&input) {
                Some(sport) => println!("Removed sport {:?}", sport),
                None => println!("No sport called {}", &input)
            }
            (players, sports)
        },
        MenuCommand::D => {
            let s = division_impl::delete_division(sports);
            sports = s;
            (players, sports)
        },
        _ => {
            write::error_response(&read_command);
            (players, sports)
        }
    }
}