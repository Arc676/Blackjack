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

use std::vec;
use rand::Rng;

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
		for _ in 0..=deck_count {
			for suit in vec![DIAMONDS, HEARTS, CLUBS, SPADES] {
				for card in 1..=13 {
					cards.push(card | suit);
				}
			}
		}
		Deck { cards, deck_count, card_index: 0 }
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
		self.card_index += 1;
		card
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
