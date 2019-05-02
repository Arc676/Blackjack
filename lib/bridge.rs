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

pub mod bridge {
	use crate::player::player::*;
	use crate::card::card::*;
	use std::ffi::CString;
	use std::os::raw::c_char;
	use std::ptr;

	macro_rules! wrap {
		($obj:expr) => {
			Box::into_raw(Box::new($obj))
		};
	}

	macro_rules! unwrap {
		($ptr:expr) => {
			unsafe {
				assert!(!$ptr.is_null());
				&*$ptr
			}
		};
	}

	macro_rules! unwrap_mut {
		($ptr:expr) => {
			unsafe {
				assert!(!$ptr.is_null());
				&mut *$ptr
			}
		};
	}

	macro_rules! free_ptr {
		($type:ident, $ptr:expr) => {
			unsafe {
				let _ = $type::from_raw($ptr);
			}
		};
	}

	#[no_mangle]
	pub extern "C" fn rust_freestr(ptr: *mut c_char) {
		free_ptr!(CString, ptr);
	}

	#[no_mangle]
	pub extern "C" fn rust_freeplayer(ptr: *mut Player) {
		free_ptr!(Box, ptr);
	}

	#[no_mangle]
	pub extern "C" fn rust_freehand(ptr: *mut Hand) {
		free_ptr!(Box, ptr);
	}

	#[no_mangle]
	pub extern "C" fn rust_freecard(ptr: *mut Card) {
		free_ptr!(Box, ptr);
	}

	#[no_mangle]
	pub extern "C" fn rust_freedeck(ptr: *mut Deck) {
		free_ptr!(Box, ptr);
	}

	#[no_mangle]
	pub extern "C" fn player_new(name: *mut c_char, is_dealer: bool, balance: i32) -> *mut Player {
		unsafe {
			let rname = match CString::from_raw(name).into_string() {
				Ok(converted) => converted,
				Err(_) => "Unnamed player".to_string()
			};
			wrap!(Player::new(rname.to_string(), is_dealer, balance))
		}
	}

	#[no_mangle]
	pub extern "C" fn player_getName(ptr: *const Player) -> *mut c_char {
		let player = unwrap!(ptr);
		match CString::new(player.get_name()) {
			Ok(s) => s.into_raw(),
			Err(_) => ptr::null_mut()
		}
	}

	#[no_mangle]
	pub extern "C" fn player_getBalance(ptr: *const Player) -> i32 {
		let player = unwrap!(ptr);
		player.get_balance()
	}

	#[no_mangle]
	pub extern "C" fn player_getStanding(ptr: *const Player) -> i32 {
		let player = unwrap!(ptr);
		player.get_standing()
	}

	#[no_mangle]
	pub extern "C" fn player_isPlaying(ptr: *const Player) -> bool {
		let player = unwrap!(ptr);
		player.is_playing()
	}

	#[no_mangle]
	pub extern "C" fn player_hit(ptr: *mut Player, pdeck: *mut Deck) -> bool {
		let player = unwrap_mut!(ptr);
		let mut deck = unwrap_mut!(pdeck);
		player.hit(&mut deck)
	}

	#[no_mangle]
	pub extern "C" fn player_stand(ptr: *mut Player) {
		let player = unwrap_mut!(ptr);
		player.stand()
	}

	#[no_mangle]
	pub extern "C" fn player_surrender(ptr: *mut Player) {
		let player = unwrap_mut!(ptr);
		player.surrender()
	}

	#[no_mangle]
	pub extern "C" fn player_split(ptr: *mut Player, pdeck: *mut Deck) -> bool {
		let player = unwrap_mut!(ptr);
		let mut deck = unwrap_mut!(pdeck);
		player.split(&mut deck)
	}

	#[no_mangle]
	pub extern "C" fn player_double(ptr: *mut Player, pdeck: *mut Deck) {
		let player = unwrap_mut!(ptr);
		let mut deck = unwrap_mut!(pdeck);
		player.double(&mut deck);
	}

	#[no_mangle]
	pub extern "C" fn player_bet(ptr: *mut Player, bet: i32, pdeck: *mut Deck) {
		let player = unwrap_mut!(ptr);
		let mut deck = unwrap_mut!(pdeck);
		player.bet(bet, &mut deck)
	}

	#[no_mangle]
	pub extern "C" fn player_hasLost(ptr: *const Player) -> bool {
		let player = unwrap!(ptr);
		player.has_lost()
	}

	#[no_mangle]
	pub extern "C" fn player_gameOver(ptr: *mut Player, dealer_value: u32) {
		let player = unwrap_mut!(ptr);
		player.game_over(dealer_value)
	}

	#[no_mangle]
	pub extern "C" fn player_playAsDealer(ptr: *mut Player, pdeck: *mut Deck) -> u32 {
		let dealer = unwrap_mut!(ptr);
		let mut deck = unwrap_mut!(pdeck);
		dealer.play_as_dealer(&mut deck)
	}

	#[no_mangle]
	pub extern "C" fn player_handCount(ptr: *const Player) -> usize {
		let player = unwrap!(ptr);
		player.get_hand_count()
	}

	#[no_mangle]
	pub extern "C" fn player_getHandWithIndex(ptr: *const Player, idx: usize) -> *const Hand {
		let player = unwrap!(ptr);
		wrap!(player.get_hand_at(idx))
	}

	#[no_mangle]
	pub extern "C" fn hand_cardCount(ptr: *const Hand) -> usize {
		let hand = unwrap!(ptr);
		hand.get_card_count()
	}

	#[no_mangle]
	pub extern "C" fn hand_getCardWithIndex(ptr: *const Hand, idx: usize) -> *const Card {
		let hand = unwrap!(ptr);
		wrap!(hand.get_card_at(idx))
	}

	#[no_mangle]
	pub extern "C" fn hand_isSet(ptr: *const Hand) -> bool {
		let hand = unwrap!(ptr);
		hand.get_is_set()
	}

	#[no_mangle]
	pub extern "C" fn hand_value(ptr: *const Hand) -> u32 {
		let hand = unwrap!(ptr);
		hand.value(false)
	}

	#[no_mangle]
	pub extern "C" fn card_toString(ptr: *const Card) -> *mut c_char {
		let card = unwrap!(ptr);
		match CString::new(card.to_string()) {
			Ok(s) => s.into_raw(),
			Err(_) => ptr::null_mut()
		}
	}

	#[no_mangle]
	pub extern "C" fn card_toU32(ptr: *const Card) -> u32 {
		let card = unwrap!(ptr);
		card.to_u32()
	}

	#[no_mangle]
	pub extern "C" fn deck_new(deck_count: usize) -> *mut Deck {
		wrap!(Deck::new(deck_count))
	}

	#[no_mangle]
	pub extern "C" fn deck_shuffle(ptr: *mut Deck) {
		let deck = unwrap_mut!(ptr);
		deck.shuffle();
	}

	#[no_mangle]
	pub extern "C" fn deck_reset(ptr: *mut Deck) {
		let deck = unwrap_mut!(ptr);
		deck.reset()
	}

	#[no_mangle]
	pub extern "C" fn deck_getNextCard(ptr: *mut Deck) -> u32 {
		let deck = unwrap_mut!(ptr);
		deck.next_card().to_u32()
	}
}
