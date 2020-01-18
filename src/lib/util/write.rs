use super::super::structs::{Players, Sports};
use super::super::enums::MenuCommand;

fn divider() {
    println!("********************************************************************************");
}

pub fn print_menu() {
    println!("                   <---  Sport Management System  --->");
    divider();
    println!("  P -> A | <num> | <name> : Print (A)ll players, player <num> or player <name>");
    println!("  S -> A | <navn>         : Print (A)ll sports or sport <name>");
    println!("  C -> P | S | D          : Prompt (C)reation of (P)layer, (S)port or (D)ivision");
    println!("  D -> P | S | D          : Prompt (D)eletion of (P)layer, (S)port or (D)ivision");
    println!("  F                       : Print (F)ixtures");
    println!("  M                       : Print results of (M)atches");
    println!("  T                       : Print (T)abels");
    println!("  R                       : (R)ead from file");
    println!("  I                       : Print player (I)nfo");
    println!("  E                       : (E)dit team");
    println!("  L                       : Print top (L)ists");
    println!("  Q                       : (Q)uit");
    divider();
    println!("Input command: ");
}

pub fn error_response(read_command: &str) {
    println!("Invalid command: {}", read_command);
    // wait a bit so the user ses the errormessage
    std::thread::sleep(std::time::Duration::from_millis(1000));
}


pub fn handle_print_players(players: &Players) {
    loop {
        println!("Print (A)ll, player <nr> or player <name>: ");
        let input = super::read::read_string();
        let command = MenuCommand::from_string(&input.to_uppercase());

        if command == MenuCommand::A {
            players.print();
        } else if command == MenuCommand::Q {
            break;
        } else if super::valid_name(&input) {
            match players.find(&input) {
                Some(p) => p.print(),
                None => println!("No player with given name")
            }

        } else if super::valid_number(&input) {
            let number_from_str: usize = input.parse().unwrap();

            match players.find_by(number_from_str) {
                Some(p) => p.print(),
                None => println!("No player with given id")
            }
        } else {
            println!("Not a valid input");
        }
    }
}

pub fn handle_print_sports(sports: &Sports) {
    loop {
        println!("Print (A)ll or <name>:");
        let input = super::read::read_string();
        let command = MenuCommand::from_string(&input.to_uppercase());

        if command == MenuCommand::A {
            sports.print();
        } else if command == MenuCommand::Q {
            break;
        } else if super::valid_name(&input) {
            match sports.find(&input) {
                Some(s) => s.print(),
                None => println!("No sport with name {}", input)
            }
        } else {
            println!("Not a valid input")
        }
    }
}

pub fn handle_print_fixtures(sports: &Sports) {
    println!("Print fixtures for which");
    println!(" - Sport:");
    let sport_input = super::read::read_string();

    match sports.find(&sport_input) {
        Some(sport) => {
            println!(" - Division");
            let div_input = super::read::read_string();
            match sport.find(&div_input) {
                Some(div) => div.print_fixtures(),
                None => println!("No division {} for sport {}", sport.name, div_input)
            }
        },
        None => println!("Found no sport with name {}", sport_input)
    }
}