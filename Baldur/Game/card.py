class Card:

    card_values = { 'Ace': 1, '2': 2, '3': 3, '4': 4, '5': 5, '6': 6, '7': 7, 
        '8': 8, '9': 9, '10': 10, 'Jack': 11, 'Queen': 12, 'King': 13 }

    card_names_by_rank = { v: k for k, v in card_values.items() }

    def __init__(self, suit, rank):
        """
        :param suit: The face of the card, e.g. Spade or Diamond
        :param rank: The value of the card, e.g 3 or King
        """
        self.suit = suit.capitalize()
        self.rank = rank
        self.points = self.card_values[rank]

    def __lt__(self, other):
        if isinstance(other, type(self)):
            return self.points < other.points
        return NotImplemented

    def __le__(self, other):
        if isinstance(other, type(self)):
            return self.points <= other.points
        return NotImplemented

    def __gt__(self, other):
        if isinstance(other, type(self)):
            return self.points > other.points
        return NotImplemented

    def __ge__(self, other):
        if isinstance(other, type(self)):
            return self.points >= other.points
        return NotImplemented

    def __eq__(self, other):
        if isinstance(other, type(self)):
            return self.rank == other.rank
        return NotImplemented

    def __str__(self):
        return "{0} of {1}".format(self.rank, self.suit)