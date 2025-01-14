use russian_roulette::*;
use std::io::{self, Write};

fn main() {
    greet();
    let mut config = Config::new();
    config.setup();
    let _name = config.name;
    let health = config.health;
    let live_rounds = config.live_rounds;

    let mut game_manager = GameManager::new();
    let mut you = Player::new(String::from("cary"), health);
    let mut opponent = Player::new(String::from("Your opponent"), health);
    let mut revolver = Revolver::new(live_rounds);

    revolver.reload();
    game_manager.add_player(&mut you);
    game_manager.add_player(&mut opponent);

    loop {
        game_manager.prompt_turn_start_menu(&mut revolver);

        if let TurnStatus::SomeoneDead = game_manager.execute_turn(&mut revolver) {
            let index = game_manager.turn_start_menu.index_target();
            
            println!("Player `{}` is dead!", game_manager.get_player(index).name());
            game_manager.remove_player(index);
            break;
        }

        game_manager.switch();
    }
    
    println!("Player `{}` Win, Game over!", game_manager.get_player(0).name());
}

fn greet() {
    println!("歡迎來到俄羅斯輪盤 勇敢的參賽者!\n遊戲即將開始 做好心裡準備");
}

struct Config {
    name: String,
    health: u32,
    live_rounds: u8,
}

impl Config {
    fn new() -> Self {
        Config {
            name: String::from("A dummy"),
            health: 2,
            live_rounds: 1,
        }
    }

    fn setup(&mut self) {
        let mut name = String::new();
        let mut health = String::new();
        let mut live_rounds = String::new();

        println!("[Setup your game config]");

        print!("* Your name? ");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut name)
            .expect("Failed to read line");

        print!("* Players's Health? ");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut health)
            .expect("Failed to read line");

        print!("* Live rounds within Cylinder? ");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut live_rounds)
            .expect("Failed to read line");

        let name = name.trim().to_string();
        let health = health.trim().parse::<u32>().unwrap();
        let live_rounds = live_rounds.trim().parse::<u8>().unwrap();

        println!("[Your Game Config]");
        println!("* Your name: {}", name);
        println!("* Player Health: {}", health);
        println!("* Live rounds: {}", live_rounds);

        self.name = name.clone();
        self.health = health;
        self.live_rounds = live_rounds;
    }
}
