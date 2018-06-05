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
    win_sequence: [i32; 25],
    current_round: usize,
    pub dealer_score: i32,
    pub player_score: i32,
    pub total_rounds: i32,
}

impl Game {
    pub fn new() -> Game {
        Game {
            deck: create_shuffled_deck(),
            deck_index: 0,
            win_sequence: generate_win_sequence(),
            current_round: 0,
            dealer_score: 0,
            player_score: 0,
            total_rounds: 25,
        }
    }

    pub fn show_next_card(&mut self) -> i32 {
        self.deck[self.deck_index]
    }

    pub fn guess_next_card(&mut self, guess: i32) -> bool {
        // Get the winner of the round
        let mut winner = self.win_sequence[self.current_round];
        self.current_round += 1;
        
        let previous_card = self.deck[self.deck_index];
        self.deck_index += 1;
        let next_card = self.deck[self.deck_index];

        // If card is either the highes or lowest the dealer must win
        if guess == UNDER && previous_card % 13 == 0 {
            winner = DEALER;
        } else if guess == OVER && previous_card % 13 == 12 {
            winner = DEALER;
        }

        // Evaluate the guess versus who should win and alter the deck accoridingly
        let mut swap_winner = false;
        if guess == OVER && winner == PLAYER && next_card % 13 <= previous_card % 13 {
            swap_winner = alter_deck(&mut self.deck, self.deck_index, SWAP_FOR_HIGHER, self.dealer_score);
        } else if guess == UNDER && winner == PLAYER && next_card % 13 >= previous_card % 13 {
            swap_winner = alter_deck(&mut self.deck, self.deck_index, SWAP_FOR_LOWER, self.dealer_score);
        } else if guess == OVER && winner == DEALER && next_card % 13 > previous_card % 13 {
            swap_winner = alter_deck(&mut self.deck, self.deck_index, SWAP_FOR_LOWER_OR_EQUAL, self.dealer_score); 
        } else if guess == UNDER && winner == DEALER && next_card % 13 < previous_card % 13 {
            swap_winner = alter_deck(&mut self.deck, self.deck_index, SWAP_FOR_HIGHER_OR_EQUAL, self.dealer_score);
        }

        // Deck alteration hit a problem and winner is swapped
        if swap_winner {
            winner = if winner == DEALER { PLAYER } else { DEALER };
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
    // Initialize deck without high and low cards
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
    // Add high and low cards to the back of the deck (reduces duplicates)
    for i in 0..4 {
        deck[44+i] = 0;
        deck[48+i] = 12;
    }
    deck
}

// Generate a win sequence where neither player will win more than 3 rounds in a row (reduces duplicates)
fn generate_win_sequence() -> [i32; 25] {
    let mut win_sequence: [i32; 25] = [0; 25];
    let mut dealer_score = 0;
    for i in 0..25 {
        win_sequence[i] = decide_winner(i as i32, 25, dealer_score);
        if win_sequence[i] == 0 {
            dealer_score += 1;
        }
    }
    //let mut concecutive_wins: [i32; 25] = [0; 25];
    let mut concecutive_wins_count = 1;
    for i in 1..25 {
        if win_sequence[i-1] == win_sequence[i] {
            concecutive_wins_count += 1;
        } else {
            concecutive_wins_count = 1;
        }

        if concecutive_wins_count > 3 {
            win_sequence = generate_win_sequence();
            break;
        }
    }
    win_sequence
}

// Decide who should win the round. This function calculates how much the dealer needs to 
// win the round based on the score and outputs a biased random winner.
fn decide_winner(current_round: i32, total_rounds: i32, dealer_score: i32) -> i32 {
    let need_to_win_ratio = calculate_need_to_win(current_round, total_rounds, dealer_score);
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

// Calculate how much the dealer needs to win this round
fn calculate_need_to_win(current_round: i32, total_rounds: i32, dealer_score: i32) -> f32 {
    let mut win_ratio = dealer_score as f32 / total_rounds as f32;
    let mut count = 0;
    for _i in current_round..total_rounds {
        win_ratio += 1.0 / total_rounds as f32;
        count += 1;
        if win_ratio > 0.51 {
            break;
        }
    }
    count as f32 / ((total_rounds) as f32 - current_round as f32)
}

// Search for the next preffered card in the deck and swap it for the top one
fn alter_deck(deck: &mut [i32; 52], deck_index: usize, alter_rule: i32, dealer_score: i32) -> bool {
    // Search for card in the remainder of the deck
    let mut swap_for = find_card_to_swap(*deck, alter_rule, deck_index + 1, 52, deck_index - 1);

    if swap_for > 51 {
        // No card found to swap for, check if we can change who wins
        let need_to_win = calculate_need_to_win(deck_index as i32, 25, dealer_score);
        if need_to_win < 1.0 {
            return true;
        }
        // Swap for a card that has allready been used and hope the player won't notice :)
        swap_for = find_card_to_swap(*deck, alter_rule, 0, 52, deck_index - 1);
    }

    let card_to_swap = deck[swap_for];
    deck[swap_for] = deck[deck_index];
    deck[deck_index] = card_to_swap;
    false
}

fn find_card_to_swap(deck: [i32; 52], alter_rule: i32, from: usize, to: usize, compare_to: usize) -> usize {
    let mut swap_for = 100;
    for i in from..to {
        if i == compare_to {
            continue;
        }
        if alter_rule == SWAP_FOR_HIGHER && deck[i as usize] % 13 > deck[compare_to] % 13 {
            swap_for = i;
            break;
        }
        if alter_rule == SWAP_FOR_LOWER && deck[i as usize] % 13 < deck[compare_to] % 13 {
            swap_for = i;
            break;
        }
        if alter_rule == SWAP_FOR_HIGHER_OR_EQUAL && deck[i as usize] % 13 >= deck[compare_to] % 13 {
            swap_for = i;
            break;
        }
        if alter_rule == SWAP_FOR_LOWER_OR_EQUAL && deck[i as usize] % 13 <= deck[compare_to] % 13 {
            swap_for = i;
            break;
        }
    }
    swap_for
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;

    #[test]
    fn play_10000_games_where_cards_in_deck_are_not_duplicated() {
        let mut duplicate_count = 0;
        for _i in 0..1000 { // play 10.000 games
            let mut game = Game::new();
            let mut used_cards: [i32; 52] = [0; 52];
            for _j in 0..game.total_rounds {
                used_cards[game.show_next_card() as usize] += 1;
                let card = game.show_next_card() % 13;
                let guess = if card < 7 { OVER } else { UNDER };
                game.guess_next_card(guess);
            }
            for i in 0..52 {
                if used_cards[i] > 1 {
                    duplicate_count += 1;
                }
            }
        }
        assert_eq!(duplicate_count, 0); // Currently, this wil fail. Duplicates occur ~0,1% of the time.
    }

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
        assert_eq!((ratio_sum / 1.0) >= 0.51, true);
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