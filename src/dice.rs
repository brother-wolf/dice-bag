use crate::results::{AllRollResults, DiceRolls, RollResult};
use rand::Rng;
use regex::Regex;

pub struct Dice {}

impl Dice {
    pub fn roll(num: usize, sides: usize, modifier: isize) -> RollResult {
        let mut rng = rand::thread_rng();
        let rolled = (0..num)
            .map(|_| rng.gen_range(1..sides + 1))
            .collect::<Vec<usize>>();
        let total = rolled.iter().sum::<usize>() as isize + modifier;
        let selected_rolls = rolled.clone();
        RollResult {
            total,
            rolled: DiceRolls { rolls: rolled },
            selected_rolls: DiceRolls {
                rolls: selected_rolls,
            },
            sides,
            num_dice: num,
            modifier,
        }
    }
}

pub fn roll(dice: &str) -> AllRollResults {
    let re = Regex::new(r"([0-9]+)d([0-9]+)([-+]?[0-9]*)").unwrap();
    let rolled = re
        .captures_iter(dice)
        .map(|cap| {
            // println!("one: {} two: {} three: {}", &cap[1], &cap[2], &cap[3]);
            let num = *&cap[1].parse::<usize>().unwrap();
            let sides = *&cap[2].parse::<usize>().unwrap();
            let modifier = if *&cap[3].is_empty() {
                0
            } else {
                *&cap[3].parse::<isize>().unwrap()
            };
            Dice::roll(num, sides, modifier)
        })
        .collect::<Vec<RollResult>>();
    let total = rolled.iter().map(|rr| rr.total).sum();
    AllRollResults { total, rolled }
}

// TODO: This is not ideal as it could introduce random test
// failures and does not explicitly catch edge cases
#[cfg(test)]
fn should_roll(dice: &str, min: isize, max: isize) {
    (0..10 * max).for_each(|_| {
        let roll = roll(dice).total;
        assert!(
            min <= roll && roll <= max,
            "{} from {} is not in the range {}/{}",
            roll,
            dice,
            min,
            max
        );
    });
}

#[test]
fn should_roll_1d6() {
    should_roll("1d6", 1, 6);
}

#[test]
fn should_roll_2d8() {
    should_roll("2d8", 2, 16);
}

#[test]
fn should_roll_1d6_plus_1() {
    should_roll("1d6+1", 2, 7);
}

#[test]
fn should_roll_1d2_minus_3() {
    should_roll("1d2-3", -2, 5);
}

#[test]
fn should_roll_4d20() {
    should_roll("4d20", 4, 80);
}

#[test]
fn should_roll_3d6_and_2d4() {
    should_roll("3d6 2d4", 5, 26);
}

#[test]
fn should_roll_3d6_plus_50_and_2d4() {
    should_roll("3d6+50 2d4", 55, 76);
}

#[test]
fn should_roll_3d6_minus_6() {
    should_roll("3d6-6", -3, 12);
}
