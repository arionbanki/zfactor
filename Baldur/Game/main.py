from game import OverUnder
from human import Human
from robot import Robot
from utils import get_percentage, clear_screen


print("Welcome to the game Over-Under\n")
print("""\
Dealer deals two cards, one face up and the other facedown from
a deck of 26 cards. It's the goal for the player to correctly 
guess if the facedown card is higher or lower than the faceup one.

A single gamle is played until the 26 card is emptied giving a 
total of 13 points to go around. In the case of equal cards the
dealer receives a point!

Good luck!

Modes: 
        1. Play as the human being you are
        2. Sit idle while 2 robots play the game 10000 times,
           no output whill be shown bu you will receive
           stats at the end for how well the dealer did.
           Super fun...""")

user_choice = input("Select option 1 or 2: ")
while user_choice not in ['1', '2']:
    user_choice = input("Only option 1 or 2 allowed: ")

if user_choice == '1':
    clear_screen()
    game = OverUnder(Human())
    game.play()
else:
    game = OverUnder(Robot())
    game.play()
    winner_list = []
    total_rounds = 10000
    for _ in range(total_rounds):
        winner_list.append(game.play())
        game.reset()

    dealer_wins = winner_list.count("Dealer")
    print("Dealer wins: {0}".format(dealer_wins))
    print("Player wins: {0}".format(total_rounds - dealer_wins))
    print("\nDealer won {0} games from {1} with a {2:.2f}% win percentage!"
        .format(dealer_wins, total_rounds, get_percentage(total_rounds, dealer_wins)))