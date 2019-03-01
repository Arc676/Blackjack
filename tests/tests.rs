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

#[cfg(test)]
mod tests {
        use blackjack::card::card::Card;
        use blackjack::card::card::Symbol;
        use blackjack::card::card::Deck;


        #[test]
        fn card_to_int_works() {
                let card = Card::new(2, Symbol::DIAMONDS);
                assert_eq!(0b0001_0010, card.to_u32());
                let card1 = Card::new(10, Symbol::SPADES);
                assert_eq!(0b1000_1010, card1.to_u32());
                let card2 = Card::new(5, Symbol::HEARTS);
                assert_eq!(0b0010_0101, card2.to_u32());
                let card3 = Card::new(7, Symbol::CLUBS);
                assert_eq!(0b0100_0111, card3.to_u32());
        }

        #[test]
        fn create_deck() {
                let mut x = Deck::create_valid_deck();
                x.sort_by(|a,b| a.to_u32().cmp(&b.to_u32()));
                println!("{:?}", x);
                assert_eq!(52,x.len())
        }

        #[test]
        fn create_new_deck() {
                let mut deck = Deck::new(4);
                let mut counter = 0;
                while counter < (4*52) {
                        println!("{:08b}", deck.next_card().to_u32());
                        counter += 1;assert_eq!(1,2);
                }
                dbg!(counter);
                //Did not panic, yay!
        }
}
