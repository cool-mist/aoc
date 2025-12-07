use crate::util::{ISolution, Solution, input_single};

// https://adventofcode.com/2025/day/2
pub struct GiftShop;
impl ISolution for GiftShop {
    fn solve(&self) -> Solution {
        let mut invalid_1 = 0;
        let mut invalid_2 = 0;
        for pair in pairs() {
            for i in pair.0..=pair.1 {
                let num_str = i.to_string();
                let num_length = num_str.len();
                let mut is_num_invalid = false;
                for part_count in 2..=num_length {
                    if num_length % part_count != 0 {
                        continue;
                    }

                    if is_num_invalid {
                        continue;
                    }

                    let part_size = num_length / part_count;
                    if (0..part_count)
                        .map(|i| &num_str[i * part_size..(i + 1) * part_size])
                        .collect::<Vec<&str>>()
                        .windows(2)
                        .all(|w| w[0] == w[1])
                    {
                        is_num_invalid = true;
                        if part_count == 2 {
                            invalid_1 += i;
                        }

                        invalid_2 += i;

                        println!(
                            "{} {} x {} | {}, {}",
                            num_str, part_size, part_count, invalid_1, invalid_2
                        );
                    }
                }
            }
        }

        assert_eq!(16793817782, invalid_1);
        assert_eq!(27469417404, invalid_2);
        Solution {
            part_one: invalid_1.to_string(),
            part_two: invalid_2.to_string(),
        }
    }
}

fn pairs() -> Vec<(u64, u64)> {
    input_single!("data/2025/2.txt")
        .into_iter()
        .filter_map(|s| s.split_once('-'))
        .map(|s| (s.0.parse().unwrap(), s.1.parse().unwrap()))
        .collect()
}
