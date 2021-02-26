use crate::game;
use rand::Rng;
use std::collections::HashSet;

pub fn convert_to_u32(num: &str) -> u32 {
    num.to_string().parse::<u32>().unwrap()
}

pub fn randomize_state(rows: u32, cols: u32) -> HashSet<game::Coordinate> {
    let mut new_set: HashSet<game::Coordinate> = HashSet::new();
    let mut rng = rand::thread_rng();

    for i in 0..rows {
        for j in 0..cols {
            let state = rng.gen_range(0..=1);
            if state == 1 {
                new_set.insert(game::Coordinate(i, j));
            }
        }
    }

    new_set
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_to_u32_with_positive_number() {
        let converted_num = convert_to_u32("09");
        assert_eq!(converted_num, 9);
    }

    #[test]
    #[should_panic]
    fn test_convert_to_u32_with_negative_number() {
        convert_to_u32("-15");
    }

    #[test]
    // NOTE: test if the same rows and cols never produce
    // the same result. This test may cause a failure
    // but the probablity of that is quite low,
    // with that being said, not sure if this is useful at all
    fn test_randomize_state_fn() {
        let rows = 12;
        let cols = 14;

        let set1 = randomize_state(rows, cols);
        let set2 = randomize_state(rows, cols);

        let mut all_different_coords = false;
        for set_item in set1.iter() {
            if !set2.contains(set_item) {
                all_different_coords = true;
                break;
            }
        }

        assert_eq!(all_different_coords, true);
    }
}
