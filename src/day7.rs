use std::cmp::Ordering;
use std::collections::HashMap;
use std::str::FromStr;
use anyhow::Result;
use crate::util::read_inputs;

#[derive(Clone, Copy, Eq, PartialEq, PartialOrd, Ord, Hash)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Card {
    Ace,
    King,
    Queen,
    Jack,
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
}

impl FromStr for Card {
    type Err = ();

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "A" => Ok(Card::Ace),
            "K" => Ok(Card::King),
            "Q" => Ok(Card::Queen),
            "J" => Ok(Card::Jack),
            "T" => Ok(Card::Ten),
            "9" => Ok(Card::Nine),
            "8" => Ok(Card::Eight),
            "7" => Ok(Card::Seven),
            "6" => Ok(Card::Six),
            "5" => Ok(Card::Five),
            "4" => Ok(Card::Four),
            "3" => Ok(Card::Three),
            "2" => Ok(Card::Two),
            _ => Err(()),
        }
    }
}

#[derive(Eq, PartialEq, Hash)]
struct Hand {
    hand_type: HandType,
    cards: Vec<Card>,
}

fn sort(a: &Hand, b: &Hand, ordering: [Card; 13]) -> Ordering {
        if a.hand_type != b.hand_type {
            return a.hand_type.cmp(&b.hand_type);
        } else {
            for i in 0..a.cards.len() {
                if a.cards[i] != b.cards[i] {
                    return ordering.iter().position(|&x| x == a.cards[i]).unwrap().cmp(&ordering.iter().position(|&x| x == b.cards[i]).unwrap());
                }
            }
        }
        return Ordering::Equal;
}

pub(crate) fn run() -> Result<()> {
    let inputs = read_inputs(7)?;
    let mut result: i32;

    let mut hands: Vec<(Hand, u32)> = inputs.iter().filter(|x| !x.is_empty()).filter_map(|x| x.split_once(' ')).map(|split| {
        let cards = split.0.chars().map(|x| x.to_string().parse::<Card>().unwrap()).collect::<Vec<Card>>();
        let bid = split.1.parse::<u32>().unwrap();

        let mut map: HashMap<Card, u8> = HashMap::new();
        cards.iter().for_each(|&card| {
            *map.entry(card).or_insert(0) += 1;
        });

        let max = map.values().max().unwrap();

        let hand_type = match max {
            5 => HandType::FiveOfAKind,
            4 => HandType::FourOfAKind,
            3 => {
                if map.len() == 2 {
                    HandType::FullHouse
                } else {
                    HandType::ThreeOfAKind
                }
            }
            2 => {
                if map.len() == 3 {
                    HandType::TwoPair
                } else {
                    HandType::OnePair
                }
            }
            1 => HandType::HighCard,
            _ => unreachable!(),
        };

        (Hand {
            hand_type,
            cards,
        }, bid)
    }).collect();

    let mut ordering = [
        Card::Ace,
        Card::King,
        Card::Queen,
        Card::Jack,
        Card::Ten,
        Card::Nine,
        Card::Eight,
        Card::Seven,
        Card::Six,
        Card::Five,
        Card::Four,
        Card::Three,
        Card::Two,
    ];

    hands.sort_unstable_by(|(a, _), (b, _)| sort(a, b, ordering));

    result = hands.iter().rev().enumerate().map(|(i, (_, bid))| (i as u32 + 1) * bid).sum::<u32>() as i32;

    println!("Day 7 Part 1: {result}");

    // ---------------------------------------

    hands = inputs.iter().filter(|x| !x.is_empty()).filter_map(|x| x.split_once(' ')).map(|split| {
        let cards = split.0.chars().filter_map(|x| x.to_string().parse::<Card>().ok()).collect::<Vec<Card>>();
        let bid = split.1.parse::<u32>().unwrap();

        let mut jokers: u8 = 0;
        let mut map: HashMap<Card, u8> = HashMap::new();
        cards.iter().for_each(|&card| {
            if card == Card::Jack {
                jokers += 1;
            }

            *map.entry(card).or_insert(0) += 1;
        });
        let no_jokers = map.iter().filter(|e| e.0 != &Card::Jack);

        let max = *(&no_jokers.clone().map(|x| x.1).max().unwrap_or(&(0u8))) + jokers;

        let hand_type = match max {
            5 => HandType::FiveOfAKind,
            4 => HandType::FourOfAKind,
            3 => {
                if no_jokers.count() == 2 {
                    HandType::FullHouse
                } else {
                    HandType::ThreeOfAKind
                }
            }
            2 => {
                if no_jokers.count() == 3 {
                    HandType::TwoPair
                } else {
                    HandType::OnePair
                }
            }
            1 => HandType::HighCard,
            _ => unreachable!(),
        };

        (Hand {
            hand_type,
            cards,
        }, bid)
    }).collect();

    ordering = [
        Card::Ace,
        Card::King,
        Card::Queen,
        Card::Ten,
        Card::Nine,
        Card::Eight,
        Card::Seven,
        Card::Six,
        Card::Five,
        Card::Four,
        Card::Three,
        Card::Two,
        Card::Jack,
    ];

    hands.sort_unstable_by(|(a, _), (b, _)| sort(a, b, ordering));

    result = hands.iter().rev().enumerate().map(|(i, (_, bid))| (i as u32 + 1) * bid).sum::<u32>() as i32;

    println!("Day 7 Part 2: {result}");

    Ok(())
}