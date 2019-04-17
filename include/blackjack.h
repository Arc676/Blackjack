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

extern void rust_free(char*);

typedef struct Player Player;
typedef struct Deck Deck;

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

extern Deck* deck_new(unsigned int);

extern void deck_shuffle(Deck*);

extern void deck_reset(Deck*);

extern int deck_getNextCard(Deck*);

#endif
