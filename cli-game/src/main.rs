use std::io; 
use rand::thread_rng;
use rand::seq::SliceRandom;

fn main() {
    println!("Rock-Paper-Scissor Game");
    println!("Enter (r)ock (p)aper or (s)cissor or (q)uit");

    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");

    println!("You played: {choice}");    

    let choices = [String::from("r"), String::from("p"), String::from("s")];
    let mut rng = thread_rng();
    let comp_choice = choices.choose(&mut rng).expect("Empty Range");

    println!("Computer played: {}", comp_choice);

    if &choice == comp_choice {
        println!("You won, hehe");
    }
    else {
        println!("You lost, sad");
    }

}