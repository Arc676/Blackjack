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
	#[derive(Debug)]
	pub struct Card  {
		value: i32,
		symbol: Symbol
	}

	#[no_mangle]
	#[repr(C)]
	pub struct Deck {
		cards: Vec<i32>,
		deck_count: usize,
		card_index: usize
	}

	#[no_mangle]
	pub static DIAMONDS: i32 = 0b0001_0000;

	#[no_mangle]
	pub static HEARTS: i32   = 0b0010_0000;

	#[no_mangle]
	pub static CLUBS: i32    = 0b0100_0000;

	#[no_mangle]
	pub static SPADES: i32   = 0b1000_0000;

	#[no_mangle]
	pub static SUIT: i32     = 0b1111_0000;

	#[no_mangle]
	pub static VALUE: i32    = 0b0000_1111;

	impl Deck {
		pub fn new(deck_count: usize) -> Deck {
			let mut cards: Vec<i32> = Vec::with_capacity(52 * deck_count);
			for _ in 0..deck_count {
				cards.append(&mut Deck::create_valid_deck().iter().map(|x| x.to_i32()).collect::<Vec<i32>>());
			}
			Deck { cards, deck_count, card_index: 0 }
		}

		pub fn create_valid_deck() -> Vec<Card>{
			let sym = vec![Symbol::DIAMONDS, Symbol::HEARTS, Symbol::CLUBS, Symbol::SPADES];
			(1..14).cycle().take(52).zip(sym.iter().cycle())
				.fold(Vec::new(), |mut acc, x| {
					acc.push(Card::new(x.0, x.1.clone()));
					acc
				})
		}

		pub fn shuffle(&mut self) {
			for i in (1..=self.deck_count * 52).rev() {
				let idx: usize = rand::thread_rng().gen_range(0, i as usize);
				let curr = i as usize;
				let tmp = self.cards[curr];
				self.cards[curr] = self.cards[idx];
				self.cards[idx] = tmp;
			}
			self.card_index = 0;
		}

		pub fn next_card(&mut self) -> i32 {
			let card = self.cards[self.card_index];
			self.card_index+=1;
			card

		}
	}

	impl Symbol {
		pub fn val(&self) -> i32 {
			match &self {
				Symbol::DIAMONDS => 0b0001_0000,
				Symbol::HEARTS => 0b0010_0000,
				Symbol::CLUBS => 0b0100_0000,
				Symbol::SPADES => 0b1000_0000
			}
		}
	}

	impl Card {
		pub fn new(value: i32, symbol: Symbol) -> Card {
			Card{value, symbol}
		}

		pub fn to_i32(&self) -> i32 {
			self.symbol.val() | self.value
		}
	}

	#[no_mangle]
	pub extern "C" fn deck_new(deck_count: usize) -> *mut Deck {
		Box::into_raw(Box::new(Deck::new(deck_count)))
	}

	#[no_mangle]
	pub extern "C" fn deck_shuffle(ptr: *mut Deck) {
		let deck = unsafe {
			assert!(!ptr.is_null());
			&mut *ptr
		};
		deck.shuffle();
	}

	#[no_mangle]
	pub extern "C" fn deck_getNextCard(ptr: *mut Deck) -> i32 {
		let deck = unsafe {
			assert!(!ptr.is_null());
			&mut *ptr
		};
		deck.next_card()
	}
}

#[cfg(test)]
mod tests {
	use crate::card::card::Card;
	use crate::card::card::Symbol;
	use crate::card::card::Deck;


	#[test]
	fn card_to_int_works() {
		let card = Card::new(2, Symbol::DIAMONDS);
		assert_eq!(0b0001_0010, card.to_i32());
		let card1 = Card::new(10, Symbol::SPADES);
		assert_eq!(0b1000_1010, card1.to_i32());
		let card2 = Card::new(5, Symbol::HEARTS);
		assert_eq!(0b0010_0101, card2.to_i32());
		let card3 = Card::new(7, Symbol::CLUBS);
		assert_eq!(0b0100_0111, card3.to_i32());
	}

	#[test]
	fn create_deck() {
		let mut x = Deck::create_valid_deck();
		x.sort_by(|a,b| a.to_i32().cmp(&b.to_i32()));
		println!("{:?}", x);
		assert_eq!(52,x.len())
	}

	#[test]
	fn create_new_deck() {
		let mut deck = Deck::new(4);
		let mut counter = 0;
		while counter < (4*52) {
			println!("{:08b}", deck.next_card());
			counter += 1;assert_eq!(1,2);
		}
		dbg!(counter);
		//Did not panic, yay!
	}
}
