use crate::day_1::calorie_counting;
use crate::day_2::rock_paper_scissors;

mod day_1;
mod day_2;
mod utils;

fn main() {
   let args: Vec<String> = std::env::args().collect();

   match args[1].as_str() {
      "1" => calorie_counting(),
      "2" => rock_paper_scissors(),
      _ => println!("I haven't done it yet UwU")
   }
}
