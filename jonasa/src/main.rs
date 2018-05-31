extern crate over_under;
extern crate rand;

use over_under::Game;
use std::io;
use std::process;

fn main() {
    println!("Velkominn í yfir undir");
    println!("Stokka spil...");
    println!("Skipti stokk í tvennt..");
    println!("Veldu viðmót leikstjóra: Elskulegur (e) eða algjör asni (a)");
    let mut dealer_attitute = String::new();
    io::stdin().read_line(&mut dealer_attitute).expect("Failed to read line");
    dealer_attitute = dealer_attitute.trim().to_lowercase();
    if dealer_attitute == "e" {
        println!("Yndislegt val hjá þér, gangi þér vel í leiknum. Ég held með þér.");
    } else if dealer_attitute == "a" {
        println!("Þú ert hugaður auminginn þinn, ég á eftir að slátra þér.");
    } else {
        println!("Þér tókst að klúðra þessu eins og öllu í lífinu, ég ætla að vera asni við þig.");
    }

    let mut game = Game::new();
    println!("Fyrsta spilið er {}", get_card_name(game.show_next_card()));

    for _i in 0..game.total_rounds {
        println!("Giskaðu hvort að næsta spil er yfir eða undir (y/u)");
        let mut guess = String::new();
        let mut wrong_count = 0;
        loop {
            io::stdin().read_line(&mut guess).expect("Failed to read line");
            guess = guess.trim().to_lowercase();
            if guess == "u" || guess == "y" {
                break;
            } else {
                guess = String::new();
                wrong_count += 1;
                if wrong_count > 3 {
                    if dealer_attitute == "e" {
                        println!("Ég held að við reynum þetta aftur þegar betur liggur á þér. Bless í bili.");
                    } else {
                        println!("Þú ert vonlaus. Bless grimma veröld...");
                    }
                    process::exit(1);
                }
                if dealer_attitute == "e" {
                    println!("Vinsamlegast ýttu á y eða u, þú getur þetta.");
                } else {
                    println!("Er þetta of erfitt fyrir þig hálfviti, ýttu á Y eða U!!!");
                }
            }
        }

        let correct_guess = game.guess_next_card(if guess == "u" { over_under::UNDER } else { over_under::OVER });

        if dealer_attitute == "e" {
            if correct_guess {
                println!("Rétt. Frábært hjá þér, ég vissi að þú gætir þetta. Næsta spil er {}. Staðan er {}-{}", get_card_name(game.show_next_card()), game.dealer_score, game.player_score);
            } else {
                println!("Rangt. Næstum því, þetta var samt flott ágiskun. Næsta spil er {}. Staðan er {}-{}", get_card_name(game.show_next_card()), game.dealer_score, game.player_score);
            }
        } else {
            if correct_guess {
                println!("Rétt. Djöfulsins heppni hjá þér. Næsta spil er {}. Staðan er {}-{}", get_card_name(game.show_next_card()), game.dealer_score, game.player_score);
            } else {
                println!("Rangt. Þetta var glatað hjá þér, þú ert svo ömurlegur. Næsta spil er {}. Staðan er {}-{}", get_card_name(game.show_next_card()), game.dealer_score, game.player_score);
            }
        }
    }

    if dealer_attitute == "e" {
        println!("Ohh, það mundaði svo litlu að þú myndir vinna. Þú tekur þetta örugglega næst :)")
    } else {
        println!("Ha ha ha ha, djöfulsins lúser ertu. Reyndu aftur ef þú þorir aumingi!")
    }
}

fn get_card_name(card_number: i32) -> String {
    let card_sort = (card_number as f32 / 13.0) as i32;
    let card_type = card_number % 13;
    let card_sort_name: String;
    match card_sort {
        0 => card_sort_name = String::from("hjarta"),
        1 => card_sort_name = String::from("spaða"),
        2 => card_sort_name = String::from("tígul"),
        3 => card_sort_name = String::from("laufa"),
        _ => card_sort_name = String::from("jóker")
    }
    let card_type_name: String;
    match card_type {
        0 => card_type_name = String::from("ás"),
        1 => card_type_name = String::from("tvistur"),
        2 => card_type_name = String::from("þristur"),
        3 => card_type_name = String::from("fjarki"),
        4 => card_type_name = String::from("fimma"),
        5 => card_type_name = String::from("sexa"),
        6 => card_type_name = String::from("sjöa"),
        7 => card_type_name = String::from("átta"),
        8 => card_type_name = String::from("nía"),
        9 => card_type_name = String::from("tía"),
        10 => card_type_name = String::from("gosi"),
        11 => card_type_name = String::from("drottning"),
        12 => card_type_name = String::from("kóngur"),
        _ => card_type_name = String::from("")
    }

    format!("{} {}", card_sort_name, card_type_name)
}