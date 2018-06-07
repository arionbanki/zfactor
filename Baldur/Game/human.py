from player import Player

class Human(Player):

    def __init__(self):
        super().__init__("Human", False)

    def get_guess(self, card=None):
        guess = input("Is the face down card higher(h) or lower(l)?: ")
        while guess not in self.answers:
            guess = input("Please select either 'h' for 'higher' or 'l' for 'lower': ")
        return guess