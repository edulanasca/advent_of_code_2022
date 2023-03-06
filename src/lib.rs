pub mod days;
pub mod utils;

#[cfg(test)]
mod tests {
    use crate::days::*;

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

}