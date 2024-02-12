#![allow(unused)]

mod player;
mod town;

use console::{Key, Term};
use crossterm::{
    execute,
    terminal::{self, ClearType},
};
use std::fs::File;
use std::io::{self, Read};

use crate::player::*;

fn main_menu_screen() {
    println!("\\============/");
    println!("\\| RPG_Game |/");
    println!("\\============/");
    println!("1. Start Game");
    println!("2. Load Game");
    println!("3. Exit");
}

fn account_creation_screen() -> PlayerStatus {
    let mut player_status: PlayerStatus = initialize_player_status();

    println!("Welcome...");
    loop {
        println!("Input nickname [5 - 20 characters | alphabet only]: ");
        let mut nickname: String = String::new();
        io::stdin()
            .read_line(&mut nickname)
            .expect("Failed to read line");

        if nickname.len() >= 5 && nickname.len() <= 20 {
            player_status.nickname = nickname;
            break;
        }
    }

    while player_status.status_points > 0 {
        println!(
            "\nYou have {} free stats point",
            player_status.status_points
        );
        println!("1. STR (Current: {})", player_status.strength);
        println!("2. VIT (Current: {})", player_status.vitality);
        println!("3. AGI (Current: {})", player_status.agility);
        println!("4. INT (Current: {})", player_status.intelligence);
        println!("5. DEX (Current: {})", player_status.dexterity);
        println!("6. LUK (Current: {})", player_status.luck);
        println!("Choose: ");
        let mut stat_choice: String = String::new();
        io::stdin()
            .read_line(&mut stat_choice)
            .expect("Failed to read line");

        match stat_choice.trim() {
            "1" => {
                player_status.strength += 1;
                player_status.status_points -= 1;
            }
            "2" => {
                player_status.vitality += 1;
                player_status.status_points -= 1;
            }
            "3" => {
                player_status.agility += 1;
                player_status.status_points -= 1;
            }
            "4" => {
                player_status.intelligence += 1;
                player_status.status_points -= 1;
            }
            "5" => {
                player_status.dexterity += 1;
                player_status.status_points -= 1;
            }
            "6" => {
                player_status.luck += 1;
                player_status.status_points -= 1;
            }
            _ => println!("wrong input"),
        };
    }

    println!("Press Enter to Continue...");
    let mut keypress: String = String::new();
    io::stdin()
        .read_line(&mut keypress)
        .expect("Failed to read line");

    player_status
}

fn render_map(
    x: usize,
    y: usize,
    last_x: usize,
    last_y: usize,
    mut map: Vec<Vec<char>>,
    mut player_status: PlayerStatus,
) -> (PlayerStatus, Vec<Vec<char>>) {
    if map[last_y][last_x] == 'O' {
        map[last_y][last_x] = 'O';
    } else {
        map[last_y][last_x] = ' ';
    }

    if map[y][x] == 'O' {
        player_status = town::town_portal(x, y, player_status);
    } else {
        map[y][x] = 'P';
    }

    for row in &map {
        for &c in row {
            print!("{}", c);
        }
        println!();
    }

    (player_status, map)
}

fn is_traversable(character: char) -> bool {
    if character == ' ' || character == 'O' {
        return true;
    }
    false
}

fn read_map(file_path: &str, file_name: &str) -> io::Result<Vec<Vec<char>>> {
    let complete_file_path: String = format!("{}{}", file_path, file_name);

    let mut file = File::open(&complete_file_path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    let game_map: Vec<Vec<char>> = content.lines().map(|line| line.chars().collect()).collect();

    Ok(game_map)
}

fn game(mut player_status: PlayerStatus) {
    let mut map = match read_map("./map/", "Town.dat") {
        Ok(game_map) => game_map,
        Err(err) => {
            eprintln!("Error: {}", err);
            return;
        }
    };

    let (mut y, mut x, mut last_y, mut last_x) = (11, 16, 10, 15);

    let stdout = Term::buffered_stdout();
    loop {
        execute!(io::stdout(), terminal::Clear(ClearType::All),).expect("Failed to clear screen");
        (player_status, map) = render_map(x, y, last_x, last_y, map, player_status);

        if let Ok(character) = stdout.read_char() {
            match character {
                'w' => {
                    if is_traversable(map[y - 1][x]) {
                        last_y = y;
                        last_x = x;
                        y -= 1;
                    }
                }
                'a' => {
                    if is_traversable(map[y][x - 1]) {
                        last_y = y;
                        last_x = x;
                        x -= 1;
                    }
                }
                's' => {
                    if is_traversable(map[y + 1][x]) {
                        last_y = y;
                        last_x = x;
                        y += 1;
                    }
                }
                'd' => {
                    if is_traversable(map[y][x + 1]) {
                        last_y = y;
                        last_x = x;
                        x += 1;
                    }
                }
                _ => break,
            }
        }
    }
}

fn start_game() {
    // let player_status: PlayerStatus = account_creation_screen();
    let player_status: PlayerStatus = preset_player_status();
    game(player_status);
}

fn main_menu() {
    main_menu_screen();
    let mut choice: String = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");

    match choice.trim() {
        "1" => start_game(),
        "3" => println!("Thank you for playing..."),
        _ => println!("Invalid choice"),
    }
}

fn main() {
    main_menu();
}
