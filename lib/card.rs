// Copyright (C) 2019 Arc676/Alessandro Vinciguerra <alesvinciguerra@gmail.com>
// Copyright (C) 2019 Fatcat560/Mario Spies

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation (version 3).

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <http://www.gnu.org/licenses/>.

pub mod card {
	use std::vec;
	use rand::Rng;

	///This enum represents the 4 card types found in black jack.
	#[derive(Copy, Clone, Debug)]
	pub enum Symbol {
		DIAMONDS,
		HEARTS,
		CLUBS,
		SPADES
	}

	///This struct represents a card found in a black jack game. It contains a numeric value and
	/// a symbol
	#[derive(Debug, Copy, Clone)]
	pub struct Card  {
		pub value: u32,
		pub symbol: Symbol
	}

	#[no_mangle]
	#[repr(C)]
	pub struct Deck {
		cards: Vec<Card>,
		deck_count: usize,
		card_index: usize
	}

	#[no_mangle]
	pub static DIAMONDS: u32 = 0b0001_0000;

	#[no_mangle]
	pub static HEARTS: u32   = 0b0010_0000;

	#[no_mangle]
	pub static CLUBS: u32    = 0b0100_0000;

	#[no_mangle]
	pub static SPADES: u32   = 0b1000_0000;

	#[no_mangle]
	pub static SUIT: u32     = 0b1111_0000;

	#[no_mangle]
	pub static VALUE: u32    = 0b0000_1111;

	impl Deck {
		pub fn new(deck_count: usize) -> Deck {
			let mut cards: Vec<Card> = Vec::with_capacity(52 * deck_count);
			for _ in 0..deck_count {
				cards.append(&mut Deck::create_valid_deck());
			}
			Deck { cards, deck_count, card_index: 0 }
		}

		pub fn create_valid_deck() -> Vec<Card> {
			let sym = vec![Symbol::DIAMONDS, Symbol::HEARTS, Symbol::CLUBS, Symbol::SPADES];
			(1..14).cycle().take(52).zip(sym.iter().cycle())
				.fold(Vec::new(), |mut acc, x| {
					acc.push(Card::new(x.0, x.1.clone()));
					acc
				})
		}

		pub fn shuffle(&mut self) {
			for i in (1..self.deck_count * 52).rev() {
				let idx: usize = rand::thread_rng().gen_range(0, i as usize);
				let curr = i as usize;
				let tmp = self.cards[curr];
				self.cards[curr] = self.cards[idx];
				self.cards[idx] = tmp;
			}
			self.card_index = 0;
		}

		pub fn next_card(&mut self) -> Card {
			let card = self.cards[self.card_index];
			self.card_index += 1;
			card

		}

		pub fn reset(&mut self) {
			self.card_index = 0;
			self.shuffle();
		}
	}

	impl Symbol {
		pub fn val(&self) -> u32 {
			match &self {
				Symbol::DIAMONDS => DIAMONDS,
				Symbol::HEARTS => HEARTS,
				Symbol::CLUBS => CLUBS,
				Symbol::SPADES => SPADES
			}
		}

		pub fn name(&self) -> &str {
			match &self {
				Symbol::DIAMONDS => "Diamonds",
				Symbol::HEARTS => "Hearts",
				Symbol::CLUBS => "Clubs",
				Symbol::SPADES => "Spades"
			}
		}
	}

	impl Card {
		pub fn new(value: u32, symbol: Symbol) -> Card {
			Card{value, symbol}
		}

		pub fn to_u32(&self) -> u32 {
			self.symbol.val() | self.value
		}

		pub fn to_string(&self) -> String {
			format!("{} of {}", self.value_to_string(), self.symbol.name())
		}

		pub fn score(&self) -> u32 {
			match self.value {
				1 => 11,
				11..=13 => 10,
				value => value
			}
		}

		fn value_to_string(&self) -> &str {
			match self.value {
				1 => "Ace",
				2 => "Two",
				3 => "Three",
				4 => "Four",
				5 => "Five",
				6 => "Six",
				7 => "Seven",
				8 => "Eight",
				9 => "Nine",
				10 => "Ten",
				11 => "Jack",
				12 => "Queen",
				13 => "King",
				_ => "Joker"
			}
		}
	}
}
