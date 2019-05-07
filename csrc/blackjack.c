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

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "blackjack.h"

void printPlayerHand(Player* player) {
	unsigned int handCount = player_handCount(player);
	char* name = player_getName(player);
	for (unsigned int ih = 0; ih < handCount; ih++) {
		Hand* hand = player_getHandWithIndex(player, ih);
		printf("%s's hand #%d (%s): %d points\n",
			name,
			ih + 1,
			hand_isSet(hand) ? "set" : "playing",
			hand_value(hand));
		unsigned int cardCount = hand_cardCount(hand);
		for (unsigned int ic = 0; ic < cardCount; ic++) {
			Card* card = hand_getCardWithIndex(hand, ic);
			char* c = card_toString(card);
			printf("%s%s", ic ? ", " : "", c);
			rust_freestr(c);
			rust_freecard(card);
		}
		rust_freehand(hand);
		printf("\n");
	}
	rust_freestr(name);
}

int main(int argc, char* argv[]) {
	char input[500];
	printf("Blackjack!\n");

	printf("How many decks? ");
	fgets(input, sizeof(input), stdin);
	const int deckCount = (int)strtol(input, (char**)NULL, 0);
	Deck* deck = deck_new(deckCount);

	printf("How many players? ");
	fgets(input, sizeof(input), stdin);
	const int playerCount = (int)strtol(input, (char**)NULL, 0);

	Player** players = malloc(playerCount * sizeof(Player*));
	for (int i = 0; i < playerCount; i++) {
		char* name = malloc(100);
		printf("Enter name for player %d: ", i + 1);
		fgets(name, 100, stdin);
		name[strlen(name) - 1] = 0;
		printf("Enter initial balance: ");
		fgets(input, sizeof(input), stdin);
		const int initialBal = (int)strtol(input, (char**)NULL, 0);
		players[i] = player_new(name, 0, initialBal);
	}

	char* dealerName = malloc(7);
	sprintf(dealerName, "Dealer");
	Player* dealer = player_new(dealerName, 1, -1);

	while (1) {
		deck_reset(deck);
		for (Player** pplayer = players; pplayer < players + playerCount; pplayer++) {
			Player* player = *pplayer;
			char* name = player_getName(player);
			printf("%s: Enter wager for this hand: ", name);
			rust_freestr(name);
			fgets(input, sizeof(input), stdin);
			int bet = (int)strtol(input, (char**)NULL, 0);
			player_bet(player, bet, deck);
		}
		player_bet(dealer, 0, deck);

		for (Player** pplayer = players; pplayer < players + playerCount; pplayer++) {
			Player* player = *pplayer;
			printPlayerHand(player);
			while (player_isPlaying(player)) {
				printf("> ");
				fgets(input, sizeof(input), stdin);
				input[strlen(input) - 1] = 0;
				if (!strcmp(input, "hit")) {
					player_hit(player, deck);
					if (player_isPlaying(player)) {
						printPlayerHand(player);
					}
				} else if (!strcmp(input, "stand")) {
					player_stand(player);
					if (player_isPlaying(player)) {
						printPlayerHand(player);
					}
				} else if (!strcmp(input, "surrender")) {
					player_surrender(player);
				} else if (!strcmp(input, "split")) {
					if (player_split(player, deck)) {
						printPlayerHand(player);
					} else {
						printf("Can't split this hand.\n");
					}
				} else if (!strcmp(input, "double")) {
					player_double(player, deck);
				} else if (!strcmp(input, "help")) {
					printf("Commands: hit, stand, surrender, split, double\n");
				} else {
					printf("Unknown command. Type 'help' for a list of available choices.\n");
				}
			}
			printPlayerHand(player);
		}
		int dealerPlays = 0;
		for (Player** pplayer = players; pplayer < players + playerCount; pplayer++) {
			if (!player_hasLost(*pplayer)) {
				dealerPlays = 1;
				break;
			}
		}
		unsigned int dealerValue = 0;
		if (dealerPlays) {
			printf("Dealer's turn\n");
			dealerValue = player_playAsDealer(dealer, deck);
			printPlayerHand(dealer);
		}
		for (Player** pplayer = players; pplayer < players + playerCount; pplayer++) {
			Player* player = *pplayer;
			player_gameOver(player, dealerValue);
			char* name = player_getName(player);
			printf("%s's balance, standing: %d/%d\n", name,  player_getBalance(player), player_getStanding(player));
			rust_freestr(name);
		}
		player_gameOver(dealer, 0);
		printf("Play again? [Y/n]: ");
		fgets(input, sizeof(input), stdin);
		if (input[0] == 'n' || input[0] == 'N') {
			break;
		}
	}

	for (Player** pplayer = players; pplayer < players + playerCount; pplayer++) {
		rust_freeplayer(*pplayer);
	}
	free(players);
	rust_freeplayer(dealer);
	rust_freedeck(deck);
	return 0;
}
