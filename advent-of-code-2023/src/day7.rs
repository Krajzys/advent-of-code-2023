use std::{time::Instant, collections::{HashSet, HashMap}, cmp::Ordering, iter::zip};

use crate::util::read_file;

pub fn day7_1(filename: String) {
    let mut filename_l = filename;
    if filename_l.is_empty() {
        filename_l = String::from("inputs/day7.txt");
    }   
    let file = read_file(&filename_l);
    
    // Create hands vector which holds all the hands in the game
    let mut hands: Vec<(&str, i32)> = Vec::new();
    for line in file.lines() {
        let hand_bet: Vec<&str> = line.split(" ").collect();
        let hand = hand_bet[0];
        let bet: i32 = hand_bet[1].parse().unwrap();
        hands.push((hand, bet));
    }

    // sort the hands vector, first by hand strength and if they are equal by characters
    let time = Instant::now();
    hands.sort_by(|(hand, _), (hand2, _)| {
        let val1 = get_hand_strength(hand);
        let val2 = get_hand_strength(hand2);

        match val1.cmp(&val2) {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => {
                for (card1, card2) in zip(hand.chars(), hand2.chars()) {
                    let order = compare_cards(card1, card2);
                    if order != Ordering::Equal {
                        return order;
                    }
                }
                Ordering::Equal
            }
        } 
    });

    // sum the bets multiplied by bet factor (index + 1)
    let mut total_winnings = 0;
    for (i, hand) in hands.iter().enumerate() {
        total_winnings += hand.1*((i+1) as i32);
    }

    print!("the result is {} [time taken {}ms]", total_winnings, time.elapsed().as_millis());
}

fn get_hand_strength(hand: &str) -> i32 {
    // hands:
    // 7 - five of a kind - set has one distinct element
    // 6 - four of a kind - set has two distinct elements
    // 5 - full house - set has two distinct elements
    // 4 - three of a kind - set has three distinct elements
    // 3 - two pair - set has three distinct elements
    // 2 - one pair - set has four distinct elements
    // 1 - high card  - set has five distinct elements

    let mut set: HashSet<char> = HashSet::new();
    set.extend(hand.chars());
    let strength = match set.len() {
        1 => 7,
        2 => {
            // if the set has two elements it can be full house or four of a kind
            // we check occurances of the first element of the vector, if it occurs in the vector 1 or 4 times it has to be four of a kind, otherwise it's full house
            let element = hand.chars().nth(0).unwrap();
            let element_count = hand.chars().filter(|el| el == &element).count();
            let strength = if element_count == 1 || element_count == 4 {
                6
            } else {
                5
            };
            return strength;
        },
        3 => {
            // if the set has three elements it can be two pairs or three of a kind
            // we check how many times does each element occur in the set, if there is an element which occurs 3 times, it's three of a kind otherwise it's two pairs
            let mut max_elemnt_count = 0;
            for element in hand.chars() {
                let element_count = hand.chars().filter(|el| el == &element).count();
                if element_count > max_elemnt_count {
                    max_elemnt_count = element_count;
                }
            }
            if max_elemnt_count == 3 {
                return 4
            } else {
                return 3
            }
        },
        4 => 2,
        5 => 1,
        _ => 0
    };
    strength
}

fn compare_cards(card1: char, card2: char) -> Ordering {
    let card1_power = get_card_value(card1);
    let card2_power = get_card_value(card2);

    return card1_power.cmp(&card2_power)
}

fn get_card_value(card: char) -> i32 {
    let cards = HashMap::from([('A', 13), ('K', 12), ('Q', 11), ('J', 10), ('T', 9), ('9', 8), ('8', 7), ('7', 6), ('6', 5), ('5', 4), ('4', 3), ('3', 2), ('2', 1)]);
    return *cards.get(&card).unwrap();
}

pub fn day7_2(filename: String) {
    let mut filename_l = filename;
    if filename_l.is_empty() {
        filename_l = String::from("inputs/day7.txt");
    }   
    let file = read_file(&filename_l);
    
    // Create hands vector which holds all the hands in the game
    let mut hands: Vec<(&str, i32)> = Vec::new();
    for line in file.lines() {
        let hand_bet: Vec<&str> = line.split(" ").collect();
        let hand = hand_bet[0];
        let bet: i32 = hand_bet[1].parse().unwrap();
        hands.push((hand, bet));
    }

    // sort the best possible hands vector, first by hand strength and if they are equal by characters of the original hand
    let time = Instant::now();
    hands.sort_by(|(hand, _), (hand2, _)| {
        let val1 = get_hand_strength(get_best_hand(hand).as_str());
        let val2 = get_hand_strength(get_best_hand(hand2).as_str());

        match val1.cmp(&val2) {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => {
                for (card1, card2) in zip(hand.chars(), hand2.chars()) {
                    let order = compare_cards2(card1, card2);
                    if order != Ordering::Equal {
                        return order;
                    }
                }
                Ordering::Equal
            }
        }
    });

    // sum the bets multiplied by bet factor (index + 1)
    let mut total_winnings = 0;
    for (i, hand) in hands.iter().enumerate() {
        total_winnings += hand.1*((i+1) as i32);
    }

    print!("the result is {} [time taken {}ms]", total_winnings, time.elapsed().as_millis());
}

fn compare_cards2(card1: char, card2: char) -> Ordering {
    let card1_power = get_card_value2(card1);
    let card2_power = get_card_value2(card2);

    return card1_power.cmp(&card2_power)
}

fn get_card_value2(card: char) -> i32 {
    let cards = HashMap::from([('A', 13), ('K', 12), ('Q', 11), ('T', 10), ('9', 9), ('8', 8), ('7', 7), ('6', 6), ('5', 5), ('4', 4), ('3', 3), ('2', 2), ('J', 1)]);
    return *cards.get(&card).unwrap();
}

fn get_best_hand(hand: &str) -> String {
    const JOKER: char = 'J';

    // count the number of occurances of each card in the hand
    let mut cards: HashMap<char, i32> = HashMap::new();
    for card in hand.chars() {
        if card == JOKER {
            continue;
        }
        let prev = match cards.insert(card, 1) {
            None => 0,
            Some(x) => x
        };
        cards.insert(card, prev + 1);
    };

    // best card to convert jokers into is always the one which occurs the most times, if it's a tie we convert to the card that have higher value
    let mut max_count = 0;
    let mut best_card = JOKER;
    for (card, count) in cards {
        if (count > max_count) ||
           (count == max_count && (get_card_value2(card) > get_card_value2(best_card))) {
            best_card = card;
            max_count = count;
        }
    }
    hand.replace(JOKER, best_card.to_string().as_str())
}