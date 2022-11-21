extern crate rand;
use std::io;
use rand::seq::SliceRandom;

const NUM_PLAYERS: usize = 4;

fn main() {
    let mut players: Vec<String> = vec![String::from("");NUM_PLAYERS];

    for (i,x) in players.iter_mut().enumerate() {
        println!("Ingresar jugador nº {}:",i+1);
        io::stdin().read_line(x).unwrap();
        *x=String::from(x.trim());
    }

    let mut rng = rand::thread_rng();
    players.shuffle(&mut rng);

    println!("Team 1: {:?}",&players[..NUM_PLAYERS/2]);
    println!("Team 2: {:?}",&players[NUM_PLAYERS/2..]);
}
