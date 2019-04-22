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

#ifndef BLACKJACK_H
#define BLACKJACK_H

extern const int DIAMONDS, HEARTS, CLUBS, SPADES, SUIT, VALUE;

typedef struct Player Player;
typedef struct Hand Hand;
typedef struct Deck Deck;
typedef struct Card Card;

extern void rust_freestr(char*);
extern void rust_freeplayer(Player*);
extern void rust_freehand(Hand*);
extern void rust_freecard(Card*);
extern void rust_freedeck(Deck*);

extern Player* player_new(char*, int, int);

extern char* player_getName(Player*);

extern int player_getBalance(Player*);

extern int player_getStanding(Player*);

extern int player_isPlaying(Player*);

extern int player_hit(Player*, Deck*);

extern void player_stand(Player*);

extern void player_surrender(Player*);

extern int player_split(Player*, Deck*);

extern void player_double(Player*, Deck*);

extern void player_bet(Player*, int, Deck*);

extern int player_hasLost(Player*);

extern void player_gameOver(Player*, unsigned int);

extern unsigned int player_playAsDealer(Player*, Deck*);

extern unsigned int player_handCount(Player*);

extern Hand* player_getHandWithIndex(Player*, unsigned int);

extern unsigned int hand_cardCount(Hand*);

extern Card* hand_getCardWithIndex(Hand*, unsigned int);

extern int hand_isSet(Hand*);

extern unsigned int hand_value(Hand*);

extern char* card_toString(Card*);

extern Deck* deck_new(unsigned int);

extern void deck_shuffle(Deck*);

extern void deck_reset(Deck*);

extern int deck_getNextCard(Deck*);

#endif
