use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub struct DiceRolls {
    pub rolls: Vec<usize>,
}

impl DiceRolls {
    pub fn sort(&self) -> Vec<usize> {
        let mut sorted = self.rolls.clone();
        sorted.sort();
        sorted.reverse();
        sorted
    }
}

impl Display for DiceRolls {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{}]",
            self.sort()
                .iter()
                .map(|r| r.to_string())
                .collect::<Vec<String>>()
                .join(",")
        )
    }
}

#[derive(Debug)]
pub struct RollResult {
    pub total: isize,
    pub rolled: DiceRolls,
    pub selected_rolls: DiceRolls,
    pub sides: usize,
    pub num_dice: usize,
    pub modifier: isize,
}

impl RollResult {
    fn signed_modifier(&self) -> String {
        match self.modifier {
            m if m < 0 => format!("{}", self.modifier),
            m if m > 0 => format!("+{}", self.modifier),
            _ => "".to_string(),
        }
    }

    fn dice_as_str(&self) -> String {
        format!("{}d{}{}", self.num_dice, self.sides, self.signed_modifier())
    }
}

impl Display for RollResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // let s = self.rolled.sort().iter().map(|r| r.to_string()).collect::<Vec<String>>().join(",");
        write!(
            f,
            "{}:{}:{}",
            self.dice_as_str(),
            format!("{}", self.rolled),
            self.total
        )
    }
}

#[derive(Debug)]
pub struct AllRollResults {
    pub total: isize,
    pub rolled: Vec<RollResult>,
}

impl Display for AllRollResults {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let dice_str = self
            .rolled
            .iter()
            .map(|rr| rr.dice_as_str())
            .collect::<Vec<String>>()
            .join(",");
        let rolls_str = self
            .rolled
            .iter()
            .map(|rr| format!("{}", rr.rolled))
            .collect::<Vec<String>>()
            .join(",");
        write!(f, "{}:{}:{}", dice_str, rolls_str, self.total)
    }
}

#[test]
fn dice_rolls_should_display_correctly() {
    assert_eq!(
        "[4,3,2]",
        format!(
            "{}",
            DiceRolls {
                rolls: vec![3, 4, 2]
            }
        )
    )
}

#[test]
fn should_format_roll_result_for_3d6_correctly() {
    let res = RollResult {
        total: 10,
        rolled: DiceRolls {
            rolls: vec![1, 6, 3],
        },
        selected_rolls: DiceRolls {
            rolls: vec![1, 6, 3],
        },
        sides: 6,
        num_dice: 3,
        modifier: 0,
    };
    assert_eq!("3d6:[6,3,1]:10", format!("{}", res));
}

#[test]
fn should_format_roll_result_for_3d6_plus_2_correctly() {
    let res = RollResult {
        total: 14,
        rolled: DiceRolls {
            rolls: vec![3, 6, 3],
        },
        selected_rolls: DiceRolls {
            rolls: vec![3, 6, 3],
        },
        sides: 6,
        num_dice: 3,
        modifier: 2,
    };
    assert_eq!("3d6+2:[6,3,3]:14", format!("{}", res));
}

#[test]
fn should_format_roll_result_for_3d6_minus_2_correctly() {
    let res = RollResult {
        total: 5,
        rolled: DiceRolls {
            rolls: vec![2, 1, 4],
        },
        selected_rolls: DiceRolls {
            rolls: vec![2, 1, 4],
        },
        sides: 6,
        num_dice: 3,
        modifier: -2,
    };
    assert_eq!("3d6-2:[4,2,1]:5", format!("{}", res));
}

#[test]
fn should_handle_negative_results() {
    let d6_res = RollResult {
        total: -2,
        rolled: DiceRolls {
            rolls: vec![1, 1, 2],
        },
        selected_rolls: DiceRolls {
            rolls: vec![1, 1, 2],
        },
        sides: 6,
        num_dice: 3,
        modifier: -6,
    };
    assert_eq!("3d6-6:[2,1,1]:-2", format!("{}", d6_res));
}

#[test]
fn should_format_all_roll_result_for_3d6_minus_2_and_2d4_plus_1_correctly() {
    let d6_res = RollResult {
        total: 5,
        rolled: DiceRolls {
            rolls: vec![2, 1, 4],
        },
        selected_rolls: DiceRolls {
            rolls: vec![2, 1, 4],
        },
        sides: 6,
        num_dice: 3,
        modifier: -2,
    };
    let d4_res = RollResult {
        total: 8,
        rolled: DiceRolls { rolls: vec![3, 4] },
        selected_rolls: DiceRolls { rolls: vec![3, 4] },
        sides: 4,
        num_dice: 2,
        modifier: 1,
    };
    let all_res = AllRollResults {
        total: 13,
        rolled: vec![d6_res, d4_res],
    };
    assert_eq!("3d6-2,2d4+1:[4,2,1],[4,3]:13", format!("{}", all_res));
}
