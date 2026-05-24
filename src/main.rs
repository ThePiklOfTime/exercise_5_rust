use rand::Rng;
use std::io;
fn receive_player_attack_dmg() -> f32{
    rand::thread_rng().gen_range(12.5..20.0)
}
fn receive_defense_multiplier() -> f32{
    1.0 / rand::thread_rng().gen_range(2.0..4.0)
}
fn receive_boss_attack_dmg() -> f32{
    rand::thread_rng().gen_range(5.0..25.0)
}


fn main() {
    let mut potions = 3;
    let mut player_health: f32 = 100.0;
    let mut boss_health: f32 = 150.0;

    loop {
        println!("| Your HP - {player_health} | Boss HP - {boss_health}");
        println!("| 1) Attack | 2) Defend | 3) Heal |");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let choice: u8 = input.trim().parse().expect("Please enter a valid number");
        if choice == 1 {
            let player_attack_dmg = receive_player_attack_dmg();
            let boss_attack_dmg = receive_boss_attack_dmg();
            boss_health -= player_attack_dmg;
            player_health -= boss_attack_dmg;
            println!("Your attack deals {player_attack_dmg} amount of damage.");
            println!("You take {boss_attack_dmg} damage.");
            
        }else if choice == 2 {
            let defense_multiplier = receive_defense_multiplier();
            let boss_attack_dmg = receive_boss_attack_dmg() * defense_multiplier;
            player_health -= boss_attack_dmg;
            println!("Defense activated!");
            println!("You take {boss_attack_dmg} damage.");
        }
        else if choice == 3 {
            let boss_attack_dmg = receive_boss_attack_dmg();
            player_health -= boss_attack_dmg;
            println!("You take {boss_attack_dmg} damage.");
            if potions > 0 {
                player_health += 25.0;
                potions -= 1;
                println!("You consume a potion.");
            } else {
                println!("You have no potions left!");
            }
        }
         else {
            println!("Invalid choice, please try again.");
        }
        if boss_health <= 0.0 {
                println!("You win!");
                break;
            }else if player_health <= 0.0 {
                println!("You have been defeated!");
                break;
            }

    }
}
