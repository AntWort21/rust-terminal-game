use std::io;

use crate::player::PlayerStatus;

pub fn town_portal(x: usize, y: usize, mut player_status: PlayerStatus) -> PlayerStatus {
    if x == 16 && y == 9 {
        player_status = goods_store(player_status);
    } else if x == 24 && y == 9 {
        player_status = gear_store(player_status);
    } else if x == 24 && y == 14 {
        player_status = inn(player_status);
    } else if x == 16 && y == 14 {
        player_status = smith(player_status);
    } else if y == 24 {
        player_status = portal_south(player_status);
    } else {
        player_status = portal_west(player_status);
    }

    player_status
}

fn goods_store(mut player_status: PlayerStatus) -> PlayerStatus {
    println!("[Goods Dealer]");
    println!(
        "Welcome {}, we have the best goods in town!",
        player_status.nickname
    );

    println!("\n[{}]", player_status.nickname);
    println!("You decide to...");
    println!("\t1. Buy\t2. Sell\t3. Back");
    println!("Action: ");

    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");

    loop {
        match choice.trim() {
            "1" => {
                println!("[Goods Dealer]");
                println!("We have HP potion and SP potion stocked up");

                println!("\n[{}]", player_status.nickname);
                println!(
                    "(You have {} zemy left). You choose to buy...",
                    player_status.money
                );
                println!("\t1. HP Potion[25 Z]\t2. SP Potion[35 Z]\t3. Back");

                loop {
                    println!("Action: ");

                    let mut choice: String = String::new();
                    io::stdin()
                        .read_line(&mut choice)
                        .expect("Failed to read line");

                    match choice.trim() {
                        "1" => {
                            println!("Input quantity [Min 1]: ");
                            let mut qty: String = String::new();
                            io::stdin()
                                .read_line(&mut choice)
                                .expect("Failed to read line");

                            let qty: i32 = qty.parse::<i32>().unwrap();

                            if qty * 25 > player_status.money {
                                println!("You don't have enough money...");
                            } else {
                                player_status.money -= qty * 25;
                            }
                        }
                        _ => break,
                    }
                }
            }
            "2" => println!("sell yer items"), //future implementation
            _ => {
                println!("[Goods Dealer]");
                println!("Thanks, come again!");
                break;
            }
        }
    }

    player_status
}

fn gear_store(mut player_status: PlayerStatus) -> PlayerStatus {
    println!("[Gear Dealer]");
    println!(
        "Yo {}, interested on buying some gears?",
        player_status.nickname
    );
    println!("\n[{}]", player_status.nickname);
    println!(
        "(You have {} zemy left). You decide to...",
        player_status.nickname
    );
    println!("\t1. Buy\t2. Not buy");
    println!("Action: ");

    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");

    match choice.trim() {
        "1" => println!("buy gears"),
        _ => {
            println!("[Gear Dealer]");
            println!("Thanks, be safe out there!");
        }
    }
    player_status
}

fn inn(mut player_status: PlayerStatus) -> PlayerStatus {
    println!("[Innkeeper]");
    println!("Hello {}, you must be tired. Do you want to rest here for 100 Z? (Restores both HP and SP)", &player_status.nickname);
    println!("\n [{}]", &player_status.nickname);
    println!(
        "(You have {} zemy left). You decide to...",
        &player_status.money
    );
    println!("\t1. Rest\t2. Not rest");
    println!("Action: ");

    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");

    match choice.trim() {
        "1" => {
            if player_status.money < 100 {
                println!("You do not have enough Zemy...");
            } else {
                player_status.money -= 100;
                player_status.current_hp = player_status.max_hp;
                player_status.current_sp = player_status.max_sp;
            }
        }
        _ => println!("You left without doing anything..."),
    }
    player_status
}

fn smith(mut player_status: PlayerStatus) -> PlayerStatus {
    println!("[Blacksmith]");
    println!("Helmet, Armor or Weapon. I can enhance them to their best form.");
    println!("It cost only 50 Z!");
    println!("\n[{}]", player_status.nickname);
    println!(
        "(You have {} Zemy left). You decide to enhance...",
        player_status.money
    );
    println!("\t1. Helmet[owned]\t2. Armor[owned]\t3. Weapon[owned]\t4. Back");
    println!("Action: ");

    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");

    match choice.trim() {
        "1" => {
            if player_status.money < 50 {
                println!("You do not have enough Zemy...");
            } else {
                player_status.money -= 50;
                // helmet upgrade
            }
        }
        "2" => {
            if player_status.money < 50 {
                println!("You do not have enough Zemy...");
            } else {
                player_status.money -= 50;
                // armor upgrade
            }
        }
        "3" => {
            if player_status.money < 50 {
                println!("You do not have enough Zemy...");
            } else {
                player_status.money -= 50;
                // weapon upgrade
            }
        }
        _ => {
            println!("[Blacksmith]");
            println!("Thank you, see you again!");
        }
    }

    player_status
}

fn portal_south(mut player_status: PlayerStatus) -> PlayerStatus {
    player_status
}

fn portal_west(mut player_status: PlayerStatus) -> PlayerStatus {
    let mut player_status = player_status;

    player_status
}
