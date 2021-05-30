use super::{super::util, Division, GameResult, Sports, Team};
use std::fmt::{Debug, Formatter, Result};
use arr_macro::arr;

impl Debug for Division {
    fn fmt(&self, formatter: &mut Formatter) -> Result {
        write!(
            formatter,
            "Division {{\n  {:?},\n  teams: [{}],\n  fixtures: [{}]}}",
            self.name,
            self.format_teams(),
            self.format_fixtures()
        )
    }
}

impl Division {
    fn format_teams(&self) -> String {
        self.teams
            .iter()
            .map(|t| format!("{:?}", t))
            .collect::<Vec<String>>()
            .join(",")
    }

    fn format_fixtures(&self) -> String {
        self.fixtures
            .iter()
            .map(|i| {
                format!(
                    "[{}]",
                    i.iter()
                        .map(|j| format!("{:?}", j))
                        .collect::<Vec<String>>()
                        .join(",")
                )
            })
            .collect::<Vec<String>>()
            .join(",")
    }

    pub fn print_fixtures(&self) {
        let mut round_idx = 0;
        let mut game_idx = 0;

        print_table_header();
        self.fixtures.iter().for_each(|round| {
            print_round(round_idx + 1);

            round.iter().for_each(|game_result| {
                let home = self.get_team(game_idx);
                let away = self.get_team(round_idx);

                match (home, away) {
                    (Some(home), Some(away)) => print_fixture(home, away, game_result),
                    _ => print_no_mathces()
                }
                game_idx += 1;
            });
            round_idx += 1;
            game_idx = 0;
        });
        print_table_footer()
    }

    pub fn get_team(&self, idx: usize) -> Option<&Team> {
        match self.teams.get(idx) {
            Some(Some(team)) => Some(team),
            _ => None
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
        let (_, month_day) = self.date.split_at(5);
        // []---
        // 01234
        // mm-dd
        let (month, _) = month_day.split_at(2);
        month
    }

    fn _year(&self) -> &[char] {
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
}

fn print_table_footer() {
    println!("\\ - - - - - - - - + - - - -- - - - + - - - - - - - - /");
}

fn print_no_mathces() {
    println!("|                     NO RESULT                      |")
}

fn print_round(round: usize) {
    if round != 1 {
        print_divider();
    }
    println!(
        "|                      ROUND {:>2}                      |",
        round
    );
    print_divider();
    println!("|       HOME      |      AWAY      |      DATE       |");
    print_divider();
}

fn print_fixture(home: &Team, away: &Team, game_result: &Option<GameResult>) {
    let date: String;
    match game_result {
        Some(gr) => {
            date =
                gr.day().iter().collect::<String>() + "/" + &gr.month().iter().collect::<String>()
        }
        None => date = String::from("X      "),
    }
    println!(
        "| {:>cw$} | {:<cw$} | {:>cw$} |",
        home.name,
        away.name,
        date,
        cw = CELL_WIDTH
    );
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
                        _last_team: 0,
                        name: div_name,
                        teams: arr![None; 50],
                        fixtures: arr![[None,None,None,None,None,None,None,None,None,None,
                                        None,None,None,None,None,None,None,None,None,None,
                                        None,None,None,None,None,None,None,None,None,None,
                                        None,None,None,None,None,None,None,None,None,None,
                                        None,None,None,None,None,None,None,None,None,None
                                    ]; 50],
                    };
                    sport.add_division(div);
                    sports.add_sport(sport);
                }
            }
            None => println!("No sport with name: {}", &input),
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
                        None => println!(
                            "No division with the name {} for sport {:?}",
                            div_name, sport
                        ),
                    };
                }
                sports.add_sport(sport)
            }
            None => println!("No sport with the name {}", input),
        };
    }
    sports
}
