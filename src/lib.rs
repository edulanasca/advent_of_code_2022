pub mod days;
pub mod utils;

#[cfg(test)]
mod tests {
    use crate::days::*;

    #[test]
    fn calorie_counting() {
        assert_eq!(Some((69693, 200945)), day_1::calorie_counting());
    }

    #[test]
    fn rock_paper_scissors() {
        assert_eq!(Some((11386, 13600)), day_2::rock_paper_scissors());
    }
}