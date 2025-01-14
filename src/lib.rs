use rand::seq::SliceRandom;
use rand::thread_rng;
use std::io::{self, Write};

#[cfg(test)]
mod tests;

#[derive(Debug)]
pub struct Revolver {
    live_rounds: u8,
    cylinder: [bool; 7],
    has_rounds: [bool; 7],
}

impl Revolver {
    pub fn new(live_rounds: u8) -> Self {
        Revolver {
            live_rounds,
            cylinder: [false; 7],
            has_rounds: [true; 7],
        }
    }

    pub fn set_live_rounds(&mut self, n: u8) {
        self.live_rounds = n;
    }

    pub fn reload(&mut self) {
        self.cylinder = [false; 7];
        self.has_rounds = [true; 7];

        let mut pool: Vec<usize> = (0..7).collect();
        let mut rng = thread_rng();

        pool.shuffle(&mut rng);
        for i in pool[0..self.live_rounds as usize].iter() {
            self.cylinder[*i] = true;
        }
    }

    pub fn shot(&mut self, index: usize, target: &mut Player) {
        if self.cylinder[index] == false {
            println!("Nothing happened...");
        } else {
            println!("Player `{}` -1 hp", target.name());
            target.reduce_health(1);
        }

        self.has_rounds[index] = false;
    }

    pub fn show_stats(&self) {
        let mut remaining_live_rounds: u8 = 0;

        println!("[1] [2] [3] [4] [5] [6] [7]");
        for (index, item) in self.has_rounds.iter().enumerate() {
            if *item == true {
                if self.cylinder[index] == true {
                    remaining_live_rounds += 1;
                }
                print!("[O] ");
            } else {
                print!("[ ] ");
            }
        }

        println!("\nLive Rounds remaining: {}", remaining_live_rounds);
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Player {
    name: String,
    health: u32,
}

impl Player {
    pub fn new(name: String, health: u32) -> Self {
        Player { name, health }
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn health(&self) -> u32 {
        self.health
    }

    pub fn reduce_health(&mut self, damage: u32) {
        self.health -= damage;
    }

    pub fn show_stats(&self) {
        print!("[{}] -> ", self.name);
        for _ in 1..=self.health {
            print!("❤️ ");
        }
        println!();
    }
}

#[derive(Debug)]
pub struct GameManager<'a> {
    current_player: usize,
    player_list: Vec<&'a mut Player>,
    num_players: u32,
    pub turn_start_menu: TurnStartMenu,
}

impl<'a> GameManager<'a> {
    pub fn new() -> Self {
        GameManager {
            current_player: 0,
            player_list: Vec::<&mut Player>::new(),
            num_players: 0,
            turn_start_menu: TurnStartMenu::new(),
        }
    }

    pub fn add_player(&mut self, player: &'a mut Player) {
        self.player_list.push(player);
        self.num_players += 1;
    }

    pub fn remove_player(&mut self, index: usize) {
        self.player_list.remove(index);
        self.num_players -= 1;
    }

    pub fn current_player_index(&self) -> usize {
        self.current_player
    }
    
    pub fn get_player(&self, index: usize) -> &Player {
        self.player_list[index]
    }

    pub fn num_players(&self) -> u32 {
        self.num_players
    }

    pub fn current_player(&self) -> &Player {
        self.player_list[self.current_player]
    }

    pub fn execute_turn(&mut self, revolver: &mut Revolver) -> TurnStatus {
        let index = self.turn_start_menu.nth_cylinder;
        // let target = self.player_list[self.turn_start_menu.index_target];

        revolver.shot(index, self.player_list[self.turn_start_menu.index_target]);
        if self.player_list[self.turn_start_menu.index_target].health() == 0 {
            return TurnStatus::SomeoneDead;
        }
        TurnStatus::Continue
    }

    pub fn switch(&mut self) {
        self.current_player += 1;
        self.current_player %= self.num_players as usize;
    }

    pub fn prompt_turn_start_menu(&mut self, revolver: &mut Revolver) {
        let mut nth_cylinder = Default::default();
        let mut index_target = Default::default();

        println!("--- {}'s turn ---", self.current_player().name());
        // todo! display cylinder number allowed to shot
        revolver.show_stats();
        print!("Which cylinder? ");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut nth_cylinder)
            .expect("Should be a number around 1~7");

        for player in self.player_list.iter() {
            player.show_stats();
        }

        print!("Who to shot? ");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut index_target)
            .expect("Should be a number in valid range");

        let nth_cylinder = nth_cylinder.trim().parse::<usize>().unwrap();
        let index_target = index_target.trim().parse::<usize>().unwrap();

        self.turn_start_menu.nth_cylinder = nth_cylinder - 1; // convert 1~7 -> 0~6
        self.turn_start_menu.index_target = index_target;
    }
}

#[derive(Debug)]
pub struct TurnStartMenu {
    nth_cylinder: usize,
    index_target: usize,
}

impl TurnStartMenu {
    pub fn new() -> Self {
        TurnStartMenu {
            nth_cylinder: 0,
            index_target: 0,
        }
    }
    
    pub fn nth_cylinder(&self) -> usize {
        self.nth_cylinder
    }
    
    pub fn index_target(&self) -> usize {
        self.index_target
    }
}

pub enum TurnStatus {
    Continue,
    SomeoneDead,
}
