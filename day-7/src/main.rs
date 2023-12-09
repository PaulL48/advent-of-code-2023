use std::{collections::{HashMap, HashSet}, ops::RangeBounds};
use lazy_static::lazy_static;

lazy_static! {
    static ref FIVE: HashMap<u8, u8> = {
        let mut m = HashMap::new();
        m.insert(5, 1);
        m
    };
    static ref FOUR: HashMap<u8, u8> = {
        let mut m = HashMap::new();
        m.insert(4, 1);
        m.insert(1, 1);
        m
    };
    static ref FULL_HOUSE: HashMap<u8, u8> = {
        let mut m = HashMap::new();
        m.insert(3, 1);
        m.insert(2, 1);
        m
    };
    static ref THREE: HashMap<u8, u8> = {
        let mut m = HashMap::new();
        m.insert(3, 1);
        m.insert(1, 2);
        m
    };
    static ref TWO_PAIR: HashMap<u8, u8> = {
        let mut m = HashMap::new();
        m.insert(2, 2);
        m.insert(1, 1);
        m
    };
    static ref PAIR: HashMap<u8, u8> = {
        let mut m = HashMap::new();
        m.insert(2, 1);
        m.insert(1, 3);
        m
    };
    static ref HIGH_CARD: HashMap<u8, u8> = {
        let mut m = HashMap::new();
        m.insert(1, 5);
        m
    };
}

const INPUT: &str = include_str!("./input.txt");
const HAND_SIZE: usize = 5;

#[derive(Debug, PartialEq, Eq, Copy, Clone, PartialOrd, Ord, Hash)]
enum Card {
    Ace = 14,
    King = 13,
    Queen = 12,
    Jack = 11,
    Ten = 10,
    Nine = 9,
    Eight = 8,
    Seven = 7,
    Six = 6,
    Five = 5,
    Four = 4,
    Three = 3,
    Two = 2,
    One = 1,
}

impl From<char> for Card {
    fn from(value: char) -> Self {
        match value {
            'A' => Card::Ace,
            'K' => Card::King,
            'Q' => Card::Queen,
            'J' => Card::Jack,
            'T' => Card::Ten,
            '9' => Card::Nine,
            '8' => Card::Eight,
            '7' => Card::Seven,
            '6' => Card::Six,
            '5' => Card::Five,
            '4' => Card::Four,
            '3' => Card::Three,
            '2' => Card::Two,
            '1' => Card::One,
            c => panic!("Bad input: {}", c)
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Ord, Eq)]
enum HandType {
    FiveOfAKind = 6,
    FourOfAKind = 5,
    FullHouse = 4,
    ThreeOfAKind = 3,
    TwoPair = 2,
    OnePair = 1,
    HighCard = 0,
}

#[derive(Debug, PartialEq, Eq)]
struct Hand {
    hand: Vec<Card>,
    card_map: HashMap<Card, u8>,
    hand_type: HandType,
    bet: u64,
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.hand_type, &self.hand).cmp(&(other.hand_type, &other.hand))
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Hand {
    pub fn new(cards: &[Card; HAND_SIZE], bet: u64) -> Self {
        let mut card_map = HashMap::new();
        for card in cards {
            *card_map.entry(*card).or_insert(0) += 1;
        }
        let hand_type = Hand::compute_type(&card_map);

        Self {
            hand: cards.iter().copied().collect(),
            card_map: card_map,
            hand_type,
            bet,
        }
    }
    
    pub fn from_str(s: &str) -> Self {
        // Convert s into card slice
        let mut line_iter = s.split_whitespace();
        let hand = line_iter.next().unwrap().chars().map(|c| c.into()).collect::<Vec<_>>();
        let bet = line_iter.next().unwrap().parse().unwrap();
        Hand::new(&[hand[0], hand[1], hand[2], hand[3], hand[4]], bet)
    }

    pub fn compute_type(card_map: &HashMap<Card, u8>) -> HandType {
        // Each hand is uniquely defined by the count of counts
        // ie. the number of times a particular number of identical cards appear
        let mut count_count: HashMap<u8, u8> = HashMap::new();
        for count in card_map.values() {
            *count_count.entry(*count).or_insert(0) += 1;
        }

        if count_count == *FIVE {
            return HandType::FiveOfAKind;
        } else if count_count == *FOUR {
            return HandType::FourOfAKind;
        } else if count_count == *FULL_HOUSE {
            return HandType::FullHouse;
        } else if count_count == *THREE {
            return HandType::ThreeOfAKind;
        } else if count_count == *TWO_PAIR {
            return HandType::TwoPair;
        } else if count_count == *PAIR {
            return HandType::OnePair;
        } else if count_count == *HIGH_CARD {
            return HandType::HighCard;
        } else {
            panic!("Unrecognized hand type {:?}", card_map);
        }
    }
}

fn main() {
    let mut hands = INPUT.lines().map(|l| Hand::from_str(l)).collect::<Vec<_>>();
    hands.sort();
    let total_winnings: u64 = hands.iter().enumerate().map(|(rank, h)| h.bet * (rank as u64 + 1)).sum();
    println!("{}", total_winnings);
}
