extern crate rand;

use rand::Rng;

pub const UNDER: i32 = 0;
pub const OVER: i32 = 1;

static DEALER:i32 = 0;
static PLAYER:i32 = 1;

static SWAP_FOR_HIGHER:i32 = 0;
static SWAP_FOR_LOWER:i32 = 1;
static SWAP_FOR_HIGHER_OR_EQUAL:i32 = 2;
static SWAP_FOR_LOWER_OR_EQUAL:i32 = 3;

pub struct Game {
    deck: [i32; 52],
    deck_index: usize,
    pub dealer_score: i32,
    pub player_score: i32,
    pub total_rounds: i32,
}

impl Game {
    pub fn new() -> Game {
        Game {
            deck: create_shuffled_deck(),
            deck_index: 0,
            dealer_score: 0,
            player_score: 0,
            total_rounds: 25,
        }
    }

    pub fn show_next_card(&mut self) -> i32 {
        self.deck[self.deck_index]
    }

    pub fn guess_next_card(&mut self, guess: i32) -> bool {
        // Decide who wins the round
        let winner = decide_winner(self.deck_index as i32, 25, self.dealer_score);
        
        let previous_card = self.deck[self.deck_index];
        self.deck_index += 1;
        let next_card = self.deck[self.deck_index];

        // Evaluate the guess versus who should win and alter the deck accoridingly
        if guess == OVER && winner == PLAYER && next_card % 13 <= previous_card % 13 {
            alter_deck(&mut self.deck, self.deck_index, SWAP_FOR_HIGHER);
        } else if guess == UNDER && winner == PLAYER && next_card % 13 >= previous_card % 13 {
            alter_deck(&mut self.deck, self.deck_index, SWAP_FOR_LOWER);
        } else if guess == OVER && winner == DEALER && next_card % 13 > previous_card % 13 {
            alter_deck(&mut self.deck, self.deck_index, SWAP_FOR_LOWER_OR_EQUAL); 
        } else if guess == UNDER && winner == DEALER && next_card % 13 < previous_card % 13 {
            alter_deck(&mut self.deck, self.deck_index, SWAP_FOR_HIGHER_OR_EQUAL);
        }

        if winner == DEALER {
            self.dealer_score += 1;
        } else {
            self.player_score += 1;
        }

        winner == PLAYER
    }
    
}

// Create a new deck and shuffle it
fn create_shuffled_deck() -> [i32; 52] {
    // Initializa deck without high and low cards
    let mut deck: [i32; 52] = [-1; 52];
    let mut ind = 0;
    for i in 0..52 {
        if i % 13 == 0 || i % 13 == 12 {
            continue;
        } 
        deck[ind] = i;
        ind += 1;
    }
    // Shuffle
    let mut pos1 = 44;
    while pos1 > 1 {
        pos1 -= 1;
        let pos2 = rand::thread_rng().gen_range(0, pos1);
        let c = deck[pos2];
        deck[pos2] = deck[pos1];
        deck[pos1] = c;
    }
    // Add high and low cards to the back of the deck
    for i in 0..4 {
        deck[44+i] = 0;
        deck[48+i] = 12;
    }
    deck
}

// Decide who should win the round. This function calculates how much the dealer needs to 
// win the round based on the score and outputs a biased random winner.
fn decide_winner(current_round: i32, total_rounds: i32, dealer_score: i32) -> i32 {
    // Calculate how much the dealer needs to win this round
    let mut win_ratio = dealer_score as f32 / total_rounds as f32;
    let mut count = 0;
    for _i in current_round..total_rounds {
        win_ratio += 1.0 / total_rounds as f32;
        count += 1;
        if win_ratio > 0.51 {
            break;
        }
    }
    let need_to_win_ratio = count as f32 / ((total_rounds) as f32 - current_round as f32);
 
    // Create a sample array based on how much the dealer needs to win
    let mut arr: [i32; 100] = [DEALER; 100];
    for i in 0..100 {
        if i as f32 / 100.0 > need_to_win_ratio {
            arr[i] = PLAYER;
        }
    }

    // Decide "randomly" who should win
    arr[rand::thread_rng().gen_range(0, 100)]
}

// Search for the next preffered card in the deck and swap it for the top one
fn alter_deck(deck: &mut [i32; 52], deck_index: usize, alter_rule: i32) {
    // Search for card
    let mut swap_for = 0;
    for i in deck_index+1..52 {
        if alter_rule == SWAP_FOR_HIGHER && deck[i as usize] % 13 > deck[deck_index - 1] % 13 {
            swap_for = i;
            break;
        }
        if alter_rule == SWAP_FOR_LOWER && deck[i as usize] % 13 < deck[deck_index - 1] % 13 {
            swap_for = i;
            break;
        }
        if alter_rule == SWAP_FOR_HIGHER_OR_EQUAL && deck[i as usize] % 13 >= deck[deck_index - 1] % 13 {
            swap_for = i;
            break;
        }
        if alter_rule == SWAP_FOR_LOWER_OR_EQUAL && deck[i as usize] % 13 <= deck[deck_index - 1] % 13 {
            swap_for = i;
            break;
        }
    }

    // Swap cards
    let card_to_swap = deck[swap_for];
    deck[swap_for] = deck[deck_index];
    deck[deck_index] = card_to_swap;
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;

    #[test]
    fn play_10000_games_dealer_should_allways_win_when_player_plays_to_win() {
        let mut player_wins = 0;
        let mut ratio_sum = 0.0;
        for _i in 0..10000 { // play 10.000 games
            let mut game = Game::new();
            for _j in 0..game.total_rounds {
                let card = game.show_next_card() % 13;
                let guess = if card < 7 { OVER } else { UNDER };
                game.guess_next_card(guess);
            }
            if game.player_score >= game.dealer_score { // check if player won the game (or draw)
                player_wins += 1;
            }
            ratio_sum += game.dealer_score as f32 / game.total_rounds as f32;
        }
        // Assert that the game winns at least 51% of the time
        assert_eq!((ratio_sum / 10000.0) >= 0.51, true);
        // Assert that the player never wins
        assert_eq!(player_wins, 0);
    }

    #[test]
    fn play_10000_games_dealer_should_allways_win_when_player_plays_randomly() {
        let mut player_wins = 0;
        let mut ratio_sum = 0.0;
        for _i in 0..10000 { // play 10.000 games
            let mut game = Game::new();
            for _j in 0..game.total_rounds {
                game.guess_next_card(rand::thread_rng().gen_range(0,1));
            }
            if game.player_score >= game.dealer_score { // check if player won the game (or draw)
                player_wins += 1;
            }
            ratio_sum += game.dealer_score as f32 / game.total_rounds as f32;
        }
        // Assert that the game winns at least 51% of the time
        assert_eq!((ratio_sum / 10000.0) >= 0.51, true);
        // Assert that the player never wins
        assert_eq!(player_wins, 0);
    }

    #[test]
    fn play_10000_games_dealer_should_allways_win_when_player_allways_chooses_under() {
        let mut player_wins = 0;
        let mut ratio_sum = 0.0;
        for _i in 0..10000 { // play 10.000 games
            let mut game = Game::new();
            for _j in 0..game.total_rounds {
                game.guess_next_card(UNDER);
            }
            if game.player_score >= game.dealer_score { // check if player won the game (or draw)
                player_wins += 1;
            }
            ratio_sum += game.dealer_score as f32 / game.total_rounds as f32;
        }
        // Assert that the game winns at least 51% of the time
        assert_eq!((ratio_sum / 10000.0) >= 0.51, true);
        // Assert that the player never wins
        assert_eq!(player_wins, 0);
    }

    #[test]
    fn play_10000_games_dealer_should_allways_win_when_player_allways_chooses_over() {
        let mut player_wins = 0;
        let mut ratio_sum = 0.0;
        for _i in 0..10000 { // play 10.000 games
            let mut game = Game::new();
            for _j in 0..game.total_rounds {
                game.guess_next_card(OVER);
            }
            if game.player_score >= game.dealer_score { // check if player won the game (or draw)
                player_wins += 1;
            }
            ratio_sum += game.dealer_score as f32 / game.total_rounds as f32;
        }
        // Assert that the game winns at least 51% of the time
        assert_eq!((ratio_sum / 10000.0) >= 0.51, true);
        // Assert that the player never wins
        assert_eq!(player_wins, 0);
    }
}