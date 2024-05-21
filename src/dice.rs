use crate::results::{AllRollResults, DiceRolls, RollResult};
use rand::Rng;
use regex::Regex;

pub struct Dice {}

impl Dice {
    pub fn roll(num: usize, sides: usize, modifier: isize) -> Result<RollResult, String> {
        match (num, sides) {
            (n, s) if n < 1 && s < 2 => {
                return Err(
                    "There must be at least 1 die, and dice must have at least 2 sides".to_string(),
                )
            }
            (n, _) if n < 1 => return Err("There must be at least 1 die".to_string()),
            (_, s) if s < 2 => return Err("Dice must have at least 2 sides".to_string()),
            _ => {
                let mut rng = rand::thread_rng();
                let rolled = (0..num)
                    .map(|_| rng.gen_range(1..&sides + 1))
                    .collect::<Vec<usize>>();
                let total = rolled.iter().sum::<usize>() as isize + &modifier;
                let selected_rolls = rolled.clone();
                Ok(RollResult {
                    total,
                    rolled: DiceRolls { rolls: rolled },
                    selected_rolls: DiceRolls {
                        rolls: selected_rolls,
                    },
                    sides,
                    num_dice: num,
                    modifier,
                })
            }
        }
    }
}

pub fn roll(dice: &str) -> AllRollResults {
    let re = Regex::new(r"([0-9]+)d([0-9]+)([-+]?[0-9]*)").unwrap();
    let rolled = re
        .captures_iter(dice)
        .flat_map(|cap| {
            let num = cap[1].parse::<usize>().unwrap();
            let sides = cap[2].parse::<usize>().unwrap();
            let modifier = if cap[3].is_empty() {
                0
            } else {
                cap[3].parse::<isize>().unwrap()
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

#[test]
fn direct_roll_3_d_6_plus_6_produces_result_of_roll_result() {
    let res = Dice::roll(3, 6, 6);
    assert!(res.is_ok());
}

#[test]
fn bad_dice_all_roll_results() {
    let res = roll("3+2");
    assert_eq!(res.total, 0);
    assert_eq!(res.rolled.len(), 0);
}

#[test]
fn nothing_to_do_all_roll_results() {
    let res = roll("");
    assert_eq!(res.total, 0);
    assert_eq!(res.rolled.len(), 0);
}

#[test]
fn no_sided_dice_all_roll_results() {
    let res = roll("1d0");
    assert_eq!(res.total, 0);
    assert_eq!(res.rolled.len(), 0);
}

#[test]
fn no_dice_all_roll_results() {
    let res = roll("0d6");
    assert_eq!(res.total, 0);
    assert_eq!(res.rolled.len(), 0);
}

#[test]
fn roll_no_sided_dice() {
    let res = Dice::roll(3, 0, 0);
    assert!(res.is_err());
    assert_eq!(res.err().unwrap(), "Dice must have at least 2 sides");
}

#[test]
fn roll_one_sided_dice() {
    let res = Dice::roll(3, 1, 0);
    assert!(res.is_err());
    assert_eq!(res.err().unwrap(), "Dice must have at least 2 sides");
}

#[test]
fn roll_no_dice() {
    let res = Dice::roll(0, 6, 0);
    assert!(res.is_err());
    assert_eq!(res.err().unwrap(), "There must be at least 1 die");
}

#[test]
fn roll_no_one_sided_dice() {
    let res = Dice::roll(0, 1, 0);
    assert!(res.is_err());
    assert_eq!(
        res.err().unwrap(),
        "There must be at least 1 die, and dice must have at least 2 sides"
    );
}

#[test]
fn roll_no_zero_sided_dice() {
    let res = Dice::roll(0, 0, 0);
    assert!(res.is_err());
    assert_eq!(
        res.err().unwrap(),
        "There must be at least 1 die, and dice must have at least 2 sides"
    );
}
