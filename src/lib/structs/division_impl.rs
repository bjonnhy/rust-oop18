use super::{Sports, Division, Array, Team, GameResult, super::util};

impl<T: std::fmt::Debug> std::fmt::Debug for Array<T> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.data[..].fmt(formatter)
    }
}

impl<T> Array<T> {
    pub fn get(&self, idx: usize) -> Option<&T> {
        if idx < self.data.len() {
            Some(&self.data[idx]);
        }
        None
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }
}


impl Division {

    pub fn print_fixtures(&self) {
        match self.fixtures {
            Some(fix) => {
                for round in 0..self.num_fixtures() {
                    print_round(round + 1);
                    for team_idx in 0..self.num_fixtures() {
                        let game;
                        match fix.get(team_idx) {
                            Some(g) => game = g.get(round),
                            None => game = None;
                        }

                        let home = self.get_team(team_idx);
                        let away = self.get_team(round);
                        let pair = (home, away);

                        match pair {
                            (Some(home), Some(away)) => print_fixture(home, away, game),
                            _ => None;
                        }
                    }
                }
            },
            None => println!("No fixtures");
        }
    }

    pub fn num_fixtures(&self) -> usize {
        match self.fixtures {
            Some(fixtures) => fixtures.len(),
            None => 0
        }
    }

    pub fn num_teams(&self) -> usize {
        match self.teams {
            Some(teams) => teams.len(),
            None => 0
        }
    }

    pub fn get_team(&self, idx: usize) -> Option<&Team> {
        match self.teams {
            Some(teams) => {
                match teams.get(idx) {
                    Some(team) => return Some(team),
                    None => return None
                }
        }
    }
}

impl GameResult {
    // --------[]
    // 0123456789
    // åååå-mm-dd
    fn day(&self) -> &[char] {
        let (_, day) = self.date.split_at(8);
        day
    }
    
    fn month(&self) -> &[char] {
        // -----[   ]
        // 0123456789
        // åååå-mm-dd
        let (any, month_day) = self.date.split_at(5);
        
        // []---
        // 01234
        // mm-dd
        let (month, _) = month_day.split_at(2);
        month
    }

    fn year(&self) -> &[char] {
        // [  ]------
        // 0123456789
        // åååå-mm-dd
        let (year, _) = self.date.split_at(4);
        year
    }
}

fn print_divider() {
    println!("+ - - - - - - - - + - - - -- - - - + - - - - - - - - +");
}


const CELL_WIDTH: usize = 15;
fn print_table_header() {
    println!("/ - - - - - - - - + - - - -- - - - + - - - - - - - - \\");
    println!("|       HOME      |      AWAY      |      DATE       |");
    print_divider();
}

fn print_round(round: usize) {
    print_divider();
    println!("|                     ROUND {:>2}                    |", round);
    print_divider();
}

fn print_fixture(home: &Team, away: &Team, game_result: Option<&GameResult>) {
    let date: String;
    match game_result {
        Some(gr) => date = gr.day().iter().collect::<String>() + "/" + &gr.month().iter().collect::<String>(),
        None => date = String::from("X      ")
    }
    println!("| {:>cw$} | {:<cw$} | {:>cw$} |", home.name, away.name, date, cw=CELL_WIDTH);
    print_divider();
}

pub fn create_division(mut sports: Sports) -> Sports {
    println!("Create division for which sport?");
    let input = util::read::read_string();

    if util::valid_name(&input) {
        match sports.remove(&input) {
            Some(mut sport) => {
                println!("Name of division:");
                let div_name = util::read::read_string();

                if util::valid_name(&div_name) {
                    let div = Division {
                        last_team: 0,
                        name: div_name,
                        teams: None,
                        fixtures: None
                    };
                    sport.add_division(div);
                    sports.add_sport(sport);
                }
            },
            None => println!("No sport with name: {}", &input)
        }
    }
    sports
}

pub fn delete_division(mut sports: Sports) -> Sports {
    println!("Which sport to delete devision from:");
    let input = util::read::read_string();

    if util::valid_name(&input) {
        match sports.remove(&input) {
            Some(mut sport) => {
                println!("Name of division:");
                let div_name = util::read::read_string();

                if util::valid_name(&div_name) {
                    match sport.remove_division(&div_name) {
                        Some(division) => println!("Deleted division {:?}", division),
                        None => println!("No division with the name {} for sport {:?}", div_name, sport)
                    };
                }
                sports.add_sport(sport)
            },
            None => println!("No sport with the name {}", input)
        };
    }
    sports
}