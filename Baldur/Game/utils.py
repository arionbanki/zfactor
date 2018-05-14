import os

def clear_screen():
        os.system('cls' if os.name == 'nt' else 'clear')

def get_percentage(total, actual):
    if total == actual:
        return 100.0
    try:
       return (actual/total)*100.0
    except ZeroDivisionError:
        return 0