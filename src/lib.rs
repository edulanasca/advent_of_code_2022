pub mod days;
pub mod utils;

#[cfg(test)]
mod tests {
    use std::io::BufRead;
    use crate::days::*;
    use crate::utils::read_file;

    #[test]
    fn day_1() {
        assert_eq!(Some((69693, 200945)), calorie_counting());
    }

    #[test]
    fn day_2() {
        assert_eq!(Some((11386, 13600)), rock_paper_scissors());
    }

    #[test]
    #[ignore]
    fn day_2_fails() {
        assert_eq!(None, rock_paper_scissors());
    }

    #[test]
    fn day_3() {
        assert_eq!(Some(7903), rucksack_reorganization());
        assert_eq!(Some(2548), rucksack_reorganization_badges());
    }

    #[test]
    fn day_4() {
        assert_eq!(Some((496, 847)), camp_cleanup());
    }

    #[test]
    fn day_5() {
        assert_eq!(Some(String::from("TDCHVHJTG")), crate_mover_9000());
        assert_eq!(Some(String::from("NGCMPJLHV")), crate_mover_9001());
    }

    #[test]
    fn day_6() {
        assert_eq!(7, turning_rouble("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 4));
        assert_eq!(5, turning_rouble("bvwbjplbgvbhsrlpgdmjqwftvncz", 4));
        assert_eq!(10, turning_rouble("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4));
        assert_eq!(11, turning_rouble("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4));
        let mut input = read_file("6").unwrap().lines();
        assert_eq!(1892, turning_rouble(input.next().unwrap().unwrap().as_str(), 4));
    }

    #[test]
    fn day_6_p2() {
        assert_eq!(19, turning_rouble("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 14));
        assert_eq!(23, turning_rouble("bvwbjplbgvbhsrlpgdmjqwftvncz", 14));
        assert_eq!(29, turning_rouble("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 14));
        assert_eq!(26, turning_rouble("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 14));
        let mut input = read_file("6").unwrap().lines();
        assert_eq!(2313, turning_rouble(input.next().unwrap().unwrap().as_str(), 14));
    }

}