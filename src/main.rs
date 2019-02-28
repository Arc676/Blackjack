// Copyright (C) 2019 Arc676/Alessandro Vinciguerra <alesvinciguerra@gmail.com>

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation (version 3).

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <http://www.gnu.org/licenses/>.

use std::io;
use std::io::Write;

extern crate blackjack;
use blackjack::card::card::*;
use blackjack::player::player::*;

fn get_int(prompt: &str) -> i32 {
        let mut input = String::new();
        print!("{}", prompt);
        io::stdout().flush().expect("Failed to flush");
        match io::stdin().read_line(&mut input) {
                Ok(_) => {
                        let val: i32 = match input.trim().parse() {
                                Ok(val) => val,
                                Err(_) => {
                                        println!("Expected integer input");
                                        get_int(prompt)
                                }
                        };
                        val
                },
                Err(_) => {
                        println!("Failed to read");
                        get_int(prompt)
                }
        }
}

fn main() {
	println!("Blackjack!");

	let deck_count = get_int("How many decks? ") as usize;
	let mut deck = Deck::new(deck_count);

	let player_count = get_int("How many players? ") as usize;
	let mut players: Vec<Player> = Vec::with_capacity(player_count);
	for i in 0..player_count {
		players[i] = Player::new(false, -1);
	}

	for mut player in players {
		loop {
			let mut input = String::new();
			print!("> ");
			io::stdout().flush().expect("Failed to flush");
			match io::stdin().read_line(&mut input) {
				Ok(_) => match input.trim() {
					"hit" => {
						player.hit(&mut deck);
					},
					"stand" => break,
					"surrender" => break,
					"split" => {
						player.split(&mut deck);
					},
					"double" => {
						println!("unavailable");
					},
					"help" => println!("Commands: hit, stand, surrender, split, double"),
					_ => println!("Unknown command. Type 'help' for a list of available choices.")
				},
				Err(_) => println!("Failed to read")
			}
		}
	}
}
