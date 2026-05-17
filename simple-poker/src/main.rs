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

    let mut rng = rand::rng();
    deck.shuffle(&mut rng);
    // println!("{:?}", deck);

    let mut hand: Vec<Card> = Vec::new();
    for _ in 0..5 {
        hand.push(deck.pop().unwrap());
    }
    hand.sort_by(|a, b| a.rank.cmp(&b.rank)); // TODO: この構文何?

    println!("----Hand----");

    for card in hand {
        println!("{:?} {:}", card.suit, card.rank);
    }
}
