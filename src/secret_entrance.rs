use crate::util::{ISolution, Solution, input};
use std::fmt::Display;

// https://adventofcode.com/2025/day/1
pub struct SecretEntrance;
impl ISolution for SecretEntrance {
    fn solve(&self) -> Solution {
        let rots = rotations();
        let mut zeroes = 0;
        let mut crosses_zero = 0;
        let mut dial: i64 = 50;
        for rot in rots {
            let prev_dial = dial;
            let normalized = (rot.by % 100) as i64;
            crosses_zero += rot.by / 100;
            match rot.dir {
                RotationDir::Right => dial = dial + normalized,
                RotationDir::Left => dial = dial - normalized,
            }

            if dial < 0 {
                dial += 100;
                if prev_dial != 0 {
                    crosses_zero += 1;
                }
            }

            if dial >= 100 {
                dial -= 100;
                if dial != 0 {
                    crosses_zero += 1;
                }
            }

            if dial == 0 {
                zeroes += 1;
                if prev_dial != 0 {
                    crosses_zero += 1;
                }
            }
            // println!(
            //     "{:>4} {} {:>4} | {:>3}, {:>5}",
            //     prev_dial, rot, dial, zeroes, crosses_zero
            // );
        }

        assert_eq!(995, zeroes);
        assert_eq!(5847, crosses_zero);
        Solution {
            part_one: zeroes.to_string(),
            part_two: crosses_zero.to_string(),
        }
    }
}

fn rotations() -> Vec<Rotation> {
    input!("data/2025/1.txt")
        .iter_mut()
        .map(|l| {
            let remaining = l.split_off(1);
            let dir = match l.as_str() {
                "L" => RotationDir::Left,
                _ => RotationDir::Right,
            };

            let r = Rotation {
                dir,
                by: remaining.parse().unwrap(),
            };
            r
        })
        .collect()
}

struct Rotation {
    dir: RotationDir,
    by: u64,
}

enum RotationDir {
    Left,
    Right,
}

impl Display for Rotation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let d = match self.dir {
            RotationDir::Left => '-',
            RotationDir::Right => '+',
        };

        write!(f, "{} {:>3}", d, self.by)
    }
}
