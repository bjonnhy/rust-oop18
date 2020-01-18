use super::{Player, Players, super::util};

impl Player {
    pub fn print(&self) {    
        println!(" <-- Player -->");
        println!("Name    : {}", self.name);
        println!("Id      : {}", self.id);
        println!("Address : {}", self.address);
    }
}

impl Players {

    pub fn print(&self) {
        self.player_list.iter()
            .for_each(|player| player.print())
    }

    pub fn add_player(&mut self, mut player: Player) {
        self.last_player += 1;
        player.id = self.last_player;
        self.player_list.push(player);
    }

    pub fn find(&self, name: &str) -> Option<&Player> {
        self.player_list.iter()
            .find(|&player| player.name.to_lowercase() == name.to_lowercase())
    }

    pub fn find_by(&self, number: usize) -> Option<&Player> {
        self.player_list.iter()
            .find(|&player| player.id == number)
    }

    pub fn remove(&mut self, name: &str) -> Option<Player> {
        let pred = |player: &Player| player.name.to_lowercase() == name.to_lowercase();
        match self.player_list.iter().position(pred) {
            Some(index) => Some(self.player_list.remove(index)),
            None => None
        }
    }
}

pub fn create_player() -> Player {
    loop {
        println!("Please input the players name: ");
        let name = util::read::read_string();

        println!("Please input the players address");
        let address = util::read::read_string();
        
        if util::valid_name(name.as_str()) && util::valid_address(address.as_str()) {
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