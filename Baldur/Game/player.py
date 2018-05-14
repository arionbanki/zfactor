class Player:

    points = 0
    answers = ['l', 'h']
    name = "AS400"

    def __init__(self, name="AS400", isRobot=True):
        self.name = name
        self.isRobot = isRobot

    def __str__(self):
        return "{0} has {1} points".format(self.name, self.points)
    
    def __lt__(self, other):
        return self.points < other.points

    def add_point(self):
        self.points = self.points + 1