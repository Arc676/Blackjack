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
use std::process;

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

fn print_player_hand(player: &Player) {
	for (ih, hand) in player.hand_iter().enumerate() {
		println!("{}'s hand #{} ({}): {} points",
			player.get_name(),
			ih + 1,
			match hand.get_is_set() {
				true => "set",
				false => "playing"
			},
			hand.value(false));
		for (ic, card) in hand.card_iter().enumerate() {
			print!("{}{}", match ic { 0 => "", _ => ", " }, card.to_string());
		}
		println!("");
	}
}

fn main() {
	println!("Blackjack!");

	let deck_count = get_int("How many decks? ") as usize;
	let player_count = get_int("How many players? ") as usize;

	if (player_count + 1) * 5 >= deck_count * 52 {
		println!("You need more decks to be able to play with this many players or the card supply might be exhausted.");
		process::exit(1);
	}

	let mut deck = Deck::new(deck_count);
	let mut players: Vec<Player> = Vec::with_capacity(player_count);
	for _ in 0..player_count {
		let mut name = String::new();
		print!("Enter your name: ");
		io::stdout().flush().expect("Failed to flush");
		io::stdin().read_line(&mut name).expect("Failed to read");
		let mut initial_balance = get_int("Enter player's initial balance: ");
		if initial_balance <= 0 {
			println!("Can't be negative. Defaulting to 1000.");
			initial_balance = 1000;
		}
		let player = Player::new(name.trim_end().to_string(), false, initial_balance);
		players.push(player);
	}

	let mut dealer = Player::new(String::from("Dealer"), true, -1);

	loop {
		deck.reset();
		for player in players.iter_mut() {
			let bet = loop {
				let input = get_int(&format!("{}: Enter wager for this hand: ", player.get_name()));
				if input <= 0 || input > player.get_balance() {
					println!("Bet must be between 1 and your balance, {}", player.get_balance());
				} else {
					break input;
				}
			};
			player.bet(bet, &mut deck);
		}
		dealer.bet(0, &mut deck);

		for player in players.iter_mut() {
			print_player_hand(player);
			while player.is_playing() {
				let mut input = String::new();
				print!("> ");
				io::stdout().flush().expect("Failed to flush");
				match io::stdin().read_line(&mut input) {
					Ok(_) => match input.trim() {
						"hit" => {
							if !player.hit(&mut deck) {
								print_player_hand(player);
							}
						},
						"stand" => {
							player.stand();
							if player.is_playing() {
								print_player_hand(player);
							}
						},
						"surrender" => {
							if !player.surrender() {
								println!("Can't surrender this hand now.");
							}
						},
						"split" => {
							if player.split(&mut deck) {
								print_player_hand(player);
							} else {
								println!("Can't split this hand");
							}
						},
						"double" => {
							player.double(&mut deck);
						},
						"help" => println!("Commands: hit, stand, surrender, split, double"),
						_ => println!("Unknown command. Type 'help' for a list of available choices.")
					},
					Err(_) => println!("Failed to read")
				}
			}
			print_player_hand(player);
		}
		let mut dealer_plays = false;
		for player in players.iter() {
			if !player.has_lost() {
				dealer_plays = true;
				break;
			}
		}
		let dealer_value = match dealer_plays {
			true => {
				println!("Dealer's turn");
				let value = dealer.play_as_dealer(&mut deck);
				print_player_hand(&dealer);
				value
			},
			false => 0
		};
		for player in players.iter_mut() {
			player.game_over(dealer_value);
			println!("{}'s balance, standing: {}/{}", player.get_name(), player.get_balance(), player.get_standing());
		}
		dealer.game_over(0);
		let mut input = String::new();
		print!("Play again? [Y/n]: ");
		io::stdout().flush().expect("Failed to flush");
		match io::stdin().read_line(&mut input) {
			Ok(_) => match input.trim() {
				"n" | "N" => break,
				_ => ()
			},
			Err(_) => println!("Failed to read")
		};
	}
}
