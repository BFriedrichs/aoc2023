use std::collections::HashMap;

fn load_input() -> String {
    let input = include_str!("input.txt");
    return input.to_string();
}

fn main() {
    println!("{}", solve(&load_input()));
}

fn to_value (card: char) -> i32 {
    match card {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 11,
        'T' => 10,
        _ => card.to_digit(10).unwrap() as i32,
    }
}

fn solve(input: &str) -> i32 {
    let lines = input.lines();

    let mut game_vec = Vec::new();

    for game in lines {
        let mut game = game.trim().split(" ");
        let hand = game.next().unwrap().chars();

        let bid = game.next().unwrap().parse::<i32>().unwrap();

        let mut counts = HashMap::new();
        let mut highest: i32 = 0;
        let mut second_highest = 0;

        let mut order_value = 0;

        for (i, card) in hand.clone().enumerate() {
            let card = to_value(card);
            let count = counts.get(&card).unwrap_or(&0) + 1;

            let mut count_highest = *counts.get(&highest).unwrap_or(&0);
            let mut count_second = *counts.get(&second_highest).unwrap_or(&0);
            if count > count_highest || count == count_highest && card > highest {
                // if card has changed
                if card != highest {
                    second_highest = highest;
                    highest = card;

                    count_second = count_highest;
                }

                count_highest = count;
            } else if count > count_second || count == count_second && card > second_highest {
                second_highest = card;
                count_second = count;
            }

            // println!("{}: {} -> {} -> {}", i, card, count, order_value);
            // println!("highest: {} -> {}", highest, count_highest);
            // println!("second_highest: {} -> {}\n", second_highest, count_second);

            counts.insert(card, count);
            order_value += card * 100_i32.pow(4-i as u32);
        }

        let count_highest = *counts.get(&highest).unwrap_or(&0);
        let count_second = *counts.get(&second_highest).unwrap_or(&0);

        // let mut value = count_highest * highest;
        let mut value = count_highest * 10;
        if count_highest == 2 && count_second == 2 {
            value = 10 * 2 + 3 * 2;
        } else if count_highest == 3 && count_second == 2 {
            value = 10 * 3 + 3 * 2;
        }

        game_vec.push((value, order_value, bid, hand.clone().collect::<String>().to_string()));
    }

    game_vec.sort_by_key(|(value, order_value, _, _)| (*value, *order_value));

    // println!("{:?}", game_vec);

    for (i, (value, order_value, bid, hand)) in game_vec.iter().enumerate() {
        println!("{}:\t{}\t{}\t{:?}", i, value, order_value, hand);
    }

    let total = game_vec.iter().enumerate().map(|(i, (_, _, bid, _))| (i + 1) as i32 * bid).sum::<i32>();
    return total;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn works() {
        let test_input = "32T3K 765
        T55J5 684
        KK677 28
        KTJJT 220
        QQQJA 483";

        let desired = 6440;
        let result = solve(test_input);
        assert_eq!(result, desired);
    }
}
