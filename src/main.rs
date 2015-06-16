mod sbsqsort;
mod engine;

use engine::Recommender;

fn main() {
  let rec = Recommender::load_knowlege(String::from("test_file.txt"));
  let target_categories = vec![String::from("Shopping.Publications.University_Presses")];
  print!("{:?}", rec.recommend(target_categories));
}

