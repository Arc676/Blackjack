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

pub mod player {
	use std::vec;
	use crate::card::card::*;

	#[no_mangle]
	pub struct Player {
		name: String,
		is_dealer: bool,
		hands: Vec<Hand>,
		surrendered: bool,
		balance: i32,
		standing: i32
	}

	#[no_mangle]
	pub struct Hand {
		cards: Vec<i32>,
		wager: i32
	}

	impl Player {
		pub fn new(name: String, is_dealer: bool, balance: i32) -> Player {
			Player {
				name, is_dealer, hands: Vec::with_capacity(2), surrendered: false, balance, standing: 0
			}
		}

		pub fn surrender(&mut self) {
			self.surrendered = true;
		}

		pub fn has_surrendered(&self) -> bool {
			self.surrendered
		}

		pub fn bet(&mut self, wager: i32, deck: &mut Deck) {
			self.hands.push(Hand::new(wager, deck));
			self.surrendered = false;
		}

		pub fn split(&mut self, deck: &mut Deck) -> bool {
			for mut hand in &mut self.hands {
				if let Some(newhand) = hand.split(deck) {
					self.hands.push(newhand);
					return true;
				}
			}
			false
		}

		pub fn hit(&mut self, deck: &mut Deck) -> bool {
			self.hands[0].hit(deck)
		}

		pub fn has_busted(&self) -> bool {
			for hand in &self.hands {
				if hand.busted() {
					return true;
				}
			}
			false
		}

		pub fn play_as_dealer(&mut self, deck: &mut Deck) {
			let mut hand = self.hands[0];
			while hand.value() < 17 {
				hand.hit(&mut deck);
			}
		}

		pub fn game_over(&mut self, dealer_value: u32) {
			for hand in self.hands {
				let value = hand.value();
				if value > dealer_value {
					self.standing += hand.get_wager();
					self.balance += hand.get_wager();
				} else if value < dealer_value {
					self.standing -= hand.get_wager();
					self.balance -= hand.get_wager();
				}
			}
			self.hands.clear();
		}
	}

	impl Hand {
		pub fn new(wager: i32, deck: &mut Deck) -> Hand {
			let mut hand = Hand { cards: Vec::with_capacity(11), wager };
			for _ in 0..2 {
				hand.cards.push(deck.next_card())
			}
			hand
		}

		pub fn split(&mut self, deck: &mut Deck) -> Option<Hand> {
			if self.cards.len() == 2 {
				if (self.cards[0] & VALUE) == (self.cards[1] & VALUE) {
					let card = self.cards[1];
					self.cards[1] = deck.next_card();
					return Some(Hand {
						cards: vec![card, deck.next_card()], wager: self.wager
					});
				}
			}
			None
		}

		pub fn hit(&mut self, deck: &mut Deck) -> bool {
			self.cards.push(deck.next_card());
			self.busted()
		}

		pub fn busted(&self) -> bool {
			let mut total = 0;
			for card in &self.cards {
				total += card & VALUE;
			}
			total > 21
		}

		pub fn get_wager(&self) -> i32 {
			self.wager
		}

		pub fn value(&self) -> u32 {
			let mut total = 0;
			let mut aces = 0;
			for card in self.cards {
				let value = card.get_value();
				if value == 1 {
					aces += 1
				}
				total += value;
			}
			while total > 21 && aces > 0 {
				total -= 10;
				aces -= 1;
			}
			total
		}
	}

	#[no_mangle]
	pub extern "C" fn player_new(name: String, isDealer: bool, balance: i32) -> *mut Player {
		Box::into_raw(Box::new(Player::new(name, isDealer, balance)))
	}

	#[no_mangle]
	pub extern "C" fn player_isDealer(player: *mut Player) -> bool {
		false
	}

	#[no_mangle]
	pub extern "C" fn player_getBalance(player: *const Player) -> i32 { 0 }

	#[no_mangle]
	pub extern "C" fn player_getStanding(player: *const Player) -> i32 { 0 }
}
