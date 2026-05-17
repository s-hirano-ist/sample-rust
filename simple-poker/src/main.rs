use rand::prelude::SliceRandom;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Suit {
    Club,
    Diamond,
    Heart,
    Spade,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Card {
    suit: Suit,
    rank: i32,
}

fn main() {
    let suit = Suit::Club;
    let rank = 1;
    let card = Card { suit, rank };
    println!("{:?}", card);

    let mut deck: Vec<Card> = Vec::new();
    let suits = [Suit::Club, Suit::Diamond, Suit::Heart, Suit::Spade];

    for suit in suits {
        for rank in 1..=13 {
            deck.push(Card { suit, rank });
        }
    }
    println!("{:?}", deck);

    let mut rng = rand::thread_rng();
    deck.shuffle(&mut rng);
    println!("{:?}", deck);
}
