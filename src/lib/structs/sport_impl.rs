use super::{Sport, Sports, Division, TableType, super::util};

impl Sport {
    pub fn print(&self) {
        println!(" <-- Sport -->");
        println!("Name: {}", self.name);
        println!("Divisions: {}", self.divisions.len());
        println!("Tabletype: {:?}", self.table_type);
    }

    pub fn find(&self, division_name: &str) -> Option<&Division> {
        self.divisions.iter()
            .find(|&division| util::eq_ignore_case(&division.name, division_name))
    }

    pub fn add_division(&mut self, division: Division) {
        self.divisions.push(division);
    }

    pub fn remove_division(&mut self, division_name: &str) -> Option<Division> {
        let pred = |division: &Division| util::eq_ignore_case(&division.name, division_name);
        match self.divisions.iter().position(pred) {
            Some(index) => Some(self.divisions.remove(index)),
            None => None
        }
    }
}

impl Sports {

    pub fn print(&self) {
        self.sports_list.iter()
            .for_each(|sport| sport.print())
    }

    pub fn add_sport(&mut self, sport: Sport) {
        self.sports_list.push(sport);
    }

    pub fn find(&self, name: &str) -> Option<&Sport> {
        self.sports_list.iter()
            .find(|sport| util::eq_ignore_case(&sport.name, name))
    }

    pub fn remove(&mut self, name: &str) -> Option<Sport> {
        let pred = |sport: &Sport| util::eq_ignore_case(&sport.name, name);
        match self.sports_list.iter().position(pred) {
            Some(index) => Some(self.sports_list.remove(index)),
            None => None
        }
    }
}

pub fn create_sport() -> Sport {
    loop {
        println!("Please input the sports name");
        let name = util::read::read_string();

        println!("Decide tabletype: ");
        println!(" - {:?} (3pt)", TableType::ThreePointTable);
        println!(" - {:?} (2pt)", TableType::TwoPointTable);
        let table_type = TableType::from_string(util::read::read_string().as_str());

        if util::valid_name(name.as_str()) && table_type != TableType::Unknown {
            return Sport {
                name: name,
                divisions: Vec::new(),
                table_type: table_type
            }
        }
    }
}