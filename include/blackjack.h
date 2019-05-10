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

/**
 * Integer representations of card suits
 */
static const unsigned int DIAMONDS = 0b00010000;
static const unsigned int HEARTS   = 0b00100000;
static const unsigned int CLUBS    = 0b01000000;
static const unsigned int SPADES   = 0b10000000;

/**
 * Bitwise filters for suit and value from integer representation of cards
 */
static const unsigned int SUIT     = 0b11110000;
static const unsigned int VALUE    = 0b00001111;

typedef struct Player Player;
typedef struct Hand Hand;
typedef struct Deck Deck;
typedef struct Card Card;

extern void rust_freestr(char*);
extern void rust_freeplayer(Player*);
extern void rust_freehand(Hand*);
extern void rust_freecard(Card*);
extern void rust_freedeck(Deck*);

/**
 * Create a new player
 * @param name Player name
 * @param isDealer Whether the player is the dealer
 * @param balance Player's initial balance
 * @return Newly constructed player
 */
extern Player* player_new(char* name, int isDealer, int balance);

/**
 * Get a player's name
 * @param player The player
 * @return The player's name (must be freed by Rust)
 */
extern char* player_getName(Player* player);

/**
 * Get a player's balance
 * @param player The player
 * @return The player's current balance
 */
extern int player_getBalance(Player* player);

/**
 * Get a player's standing
 * @param player The player
 * @return The player's current standing
 */
extern int player_getStanding(Player* player);

/**
 * Determine whether a player is still playing
 * @param player The player
 * @return Whether the player's turn is still in progress
 */
extern int player_isPlaying(Player* player);

/**
 * Hit
 * @param player The active player
 * @param deck The deck from which to deal
 * @return Whether the player busted
 */
extern int player_hit(Player* player, Deck* deck);

/**
 * Stand
 * @param player The active player
 */
extern void player_stand(Player* player);

/**
 * Surrender the current hand
 * @param player The active player
 * @return Whether the current hand can be surrendered
 */
extern int player_surrender(Player* player);

/**
 * Split the player's hand
 * @param player The active player
 * @param deck The deck from which to deal
 * @return Whether the hand can be split
 */
extern int player_split(Player* player, Deck* deck);

/**
 * Double down on the player's hand
 * @param player The active player
 * @param deck The deck from which to deal
 * @return Whether the player busted
 */
extern int player_double(Player* player, Deck* deck);

/**
 * Place a bet
 * @param player The player placing the bet
 * @param deck The deck from which to deal
 */
extern void player_bet(Player* player, int, Deck* deck);

/**
 * Determine whether a player has already lost
 * @param player The player to check
 * @return Whether the player has surrendered or busted all their hands
 */
extern int player_hasLost(Player* player);

/**
 * Settle all bets and discard all cards drawn this round
 * @param player The player whose round is ending
 * @param dealerValue The value of the dealer's hand
 */
extern void player_gameOver(Player* player, unsigned int dealerValue);

/**
 * Play as the dealer
 * @param player The dealer
 * @param deck The deck from which to deal
 * @return The value of the dealer's hand, or 0 if the dealer busted
 */
extern unsigned int player_playAsDealer(Player* player, Deck* deck);

/**
 * Get the number of hands a player has
 * @param player The player to check
 * @return Number of hands being played by the player
 */
extern unsigned int player_handCount(Player* player);

/**
 * Get a player's hand
 * @param player The player with the desired hand
 * @param idx The index of the desired hand
 * @return The desired hand
 */
extern Hand* player_getHandWithIndex(Player* player, unsigned int idx);

/**
 * Determine whether a player can surrender their currently active hand
 * @param player The player to check
 * @return Whether the currently active hand can be surrendered
 */
extern int player_canSurrenderCurrentHand(Player* player);

/**
 * Determine whether a player can split their currently active hand
 * @param player The player to check
 * @return Whether the currently active hand can be split
 */
extern int player_canSplitHand(Player* player);

/**
 * Get the number of cards in a hand
 * @param hand Hand to check
 * @return Number of cards in the hand
 */
extern unsigned int hand_cardCount(Hand* hand);

/**
 * Get a card in a player hand
 * @param hand Hand containing the desired card
 * @param idx Index of the desired card
 * @return The desired card
 */
extern Card* hand_getCardWithIndex(Hand* hand, unsigned int idx);

/**
 * Check whether a hand is set
 * @param hand Hand to check
 * @return Whether the hand is set
 */
extern int hand_isSet(Hand* hand);

/**
 * Get the value of a hand
 * @param hand Hand whose value to check
 * @return Points the hand is worth
 */
extern unsigned int hand_value(Hand* hand);

/**
 * Obtain the string representation of a card
 * @param card Card to convert to string
 * @return String representation of the card (must be freed by Rust)
 */
extern char* card_toString(Card* card);

/**
 * Obtain the integer representation of a card
 * @param card Card to convert to unsigned integer
 * @return Integer representation of the card
 */
extern unsigned int card_toU32(Card* card);

/**
 * Create a new set of decks
 * @param count Number of decks
 * @return Newly constructed Deck struct
 */
extern Deck* deck_new(unsigned int count);

/**
 * Shuffle a deck
 * @param deck Deck to shuffle
 */
extern void deck_shuffle(Deck* deck);

/**
 * Reset deck status
 * @param deck Deck to reset
 */
extern void deck_reset(Deck* deck);

#endif
