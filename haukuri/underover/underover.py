import argparse
import random
import sys

from abc import ABC, abstractmethod
from collections import namedtuple
from itertools import count, product

Rank = namedtuple('Rank', 'name value')
Suit = namedtuple('Suit', 'name')
Card = namedtuple('Card', 'rank suit')
Score = namedtuple('Score', 'player house')

def iter_ranks():
    rank_names = (
        'ace', 'two', 'three', 'four', 'five', 'six', 'seven',
        'eight', 'nine', 'ten', 'jack', 'queen', 'king'
    )
    for name, value in zip(rank_names, count(1)):
        yield Rank(name, value)

def iter_suits():
    suit_names = ('hearts', 'spades', 'diamonds', 'clubs')
    for name in suit_names:
        yield Suit(name)

def iter_deck():
    ranks = iter_ranks()
    suits = iter_suits()
    for rank, suit in product(ranks, suits):
        yield Card(rank, suit)

def make_deck():
    deck = list(iter_deck())
    return deck

class GameControls(ABC):
    @abstractmethod
    def down_is_higher(self):
        pass
    
    @abstractmethod
    def down_is_lower(self):
        pass

class PlayerCallbacks:
    @abstractmethod
    def on_round(self, game: GameControls, card: Card):
        pass
    
    @abstractmethod
    def on_game_over(self, game: GameControls, score: Score):
        pass

class Game(GameControls):
    def __init__(self, player: PlayerCallbacks):
        self.deck = make_deck()
        random.shuffle(self.deck)
        half = len(self.deck) // 2
        self.deck = self.deck[:half]
        self.player = player
        self.player_points = 0
        self.house_points = 0
    
    def start(self):
        self.advance()
    
    def advance(self):
        if self.deck:
            self.deal()
        else:
            self.end()
    
    def deal(self):
        self.up = self.deck.pop()
        self.down = self.deck.pop()
        self.player.on_round(self, self.up)
    
    def down_is_higher(self):
        if self.down > self.up:
            self.player_points += 1
        else:
            self.house_points += 1
        self.advance()
    
    def down_is_lower(self):
        if self.down < self.up:
            self.player_points += 1
        else:
            self.house_points += 1
        self.advance()
    
    def end(self):
        score = Score(self.player_points, self.house_points)
        self.player.on_game_over(self, score)

class RandomTestPlayer(PlayerCallbacks):
    def __init__(self):
        self.player_victories = 0
        self.house_victories = 0

    def on_round(self, game: GameControls, card: Card):
        answers = (game.down_is_higher, game.down_is_lower)
        answer = random.choice(answers)
        answer()

    def on_game_over(self, game: GameControls, score: Score):
        if score.player > score.house:
            self.player_victories += 1
        else:
            self.house_victories += 1

class ConsolePlayer(PlayerCallbacks):
    def on_round(self, game: GameControls, card: Card):
        print(f'The upwards facing card is a {card.rank.name} of {card.suit.name}')
        user_answer = self.prompt_for_answer()
        if user_answer == 'h':
            game.down_is_higher()
        elif user_answer == 'l':
            game.down_is_lower()
        else:
            raise Exception('invalid answer')
    
    def on_game_over(self, game: GameControls, score: Score):
        if score.player > score.house:
            print(f'You won with {score.player} points over {score.house}')
        else:
            print(f'The house won with {score.house} points over {score.player}')
    
    def print_instructions(self):
        print('The following commands are available')
        print('h', 'the hidden card is higher')
        print('l', 'the hidden card is lower')
        print('q', 'quit the game')

    def prompt_for_answer(self):
        answer = ''
        while answer not in ('h', 'l', 'q'):
            answer = input('>')
            answer = answer.strip()
            if answer == 'q':
                self.quit()
        return answer
    
    def quit(self):
        print('Quitters never win')
        sys.exit(0)

def test_house_mostly_wins():
    iterations = 1000
    player = RandomTestPlayer()
    for _ in range(iterations):
        game = Game(player)
        game.start()
    house_victory_ratio = player.house_victories / iterations
    assert house_victory_ratio > 0.51, r'the house should win 51% of the time or more'

def run_tests():
    tests = [
        thing for name, thing in globals().items() 
        if callable(thing) and name.startswith('test_')]
    print('Running tests...')
    for test in tests:
        test()
        print('ok', test.__name__)
    print('All passed')

def play_on_console():
    player = ConsolePlayer()
    game = Game(player)
    player.print_instructions()
    game.start()

def main():
    parser = argparse.ArgumentParser(description='Play a game of under/over')
    commands = ('play', 'test')
    parser.add_argument('command', type=str, help='a command to run', choices=commands)
    args = parser.parse_args()
    if args.command == 'play':
        play_on_console()
    if args.command == 'test':
        run_tests()


if __name__ == '__main__':
    main()

