use crate::util::{ISolution, Solution, input};

// https://adventofcode.com/2025/day/3
pub struct Lobby;
impl ISolution for Lobby {
    fn solve(&self) -> Solution {
        let nums = input!("data/2025/3.txt")
            .iter()
            .map(|s| s.chars())
            .map(|ca| ca.into_iter().filter_map(|c| c.to_digit(10)).collect())
            .collect::<Vec<Vec<u32>>>();
        let mut max_2 = 0;
        let mut max_12 = 0;

        for num in nums {
            let this_max_2 = get_max_with_digits(2, &num);
            let this_max_12 = get_max_with_digits(12, &num);
            max_2 += this_max_2;
            max_12 += this_max_12;

            // println!("{:?} | {} {}", f_n(&num), this_max_2, this_max_12);
        }

        assert_eq!(17207, max_2);
        assert_eq!(170997883706617, max_12);
        Solution {
            part_one: max_2.to_string(),
            part_two: max_12.to_string(),
        }
    }
}

fn get_max_with_digits(num_digits: usize, num: &[u32]) -> u64 {
    let mut result = 0;
    let mut search_start = 0;
    let mut search_end = num.len() - num_digits;
    for _ in 1..=num_digits {
        let (max_idx, max) = max_in_slice(&num[search_start..=search_end]);
        search_start += max_idx + 1;
        search_end += 1;
        result = result * 10 + max as u64;
    }

    return result;
}

fn max_in_slice(slice: &[u32]) -> (usize, u32) {
    let mut max = 0;
    let mut max_idx = 0;
    for (idx, num) in slice.iter().enumerate() {
        if *num > max {
            max = *num;
            max_idx = idx;
        }
    }

    (max_idx, max)
}

#[allow(unused)]
fn f_n(slice: &[u32]) -> String {
    let digits: Vec<String> = slice.iter().map(|d| d.to_string()).collect();
    digits.join("")
}
