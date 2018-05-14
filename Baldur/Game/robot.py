from player import Player
from card import Card
import random

class Robot(Player):

    def __init__(self):
        super().__init__()

    def get_guess(self, card:Card=None):
        if card:
            if card.points >= 8:
                return 'l'
            elif card.points <= 6:
                return 'h'
        return random.choice(self.answers)