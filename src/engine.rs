use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::io::Write;
use std::collections::btree_map::BTreeMap;
use std::collections::btree_set::BTreeSet;
use std::iter::Iterator;

use sbsqsort;

macro_rules! errorln(
    ($($arg:tt)*) => (
        match writeln!(&mut ::std::io::stderr(), $($arg)* ) {
            Ok(_) => {},
            Err(e) => panic!("Unable to write to stderr: {}", e),
        }
    )
);

/// Loads things and catgories those things belong the thing
/// and then let you figure out which thing best matches a set
/// of categories
pub struct Recommender {
  categories : BTreeMap<String, BTreeSet<String>>,
  pages : BTreeMap<String, BTreeSet<String>>,
}

impl Recommender {
  /// Loads the knowlege file and returns a filled-in Recommended
  /// File should be in the following format. Each value should
  /// contain no whitespace
  ///
  ///    <thing> <category> <category> <category> ...
  ///
  /// Example:
  ///
  ///     trains transportation ground things-people-model
  ///     cars transportation ground
  ///     plane transportation air
  pub fn load_knowlege(path : String) -> Recommender  {
    let file = match File::open(path) {
        Ok(file) => file,
        Err(e)  => panic!("Error occured opening knowlege file: {} ", e),
    };

    let reader = BufReader::new(&file);

    let mut categories = BTreeMap::new();
    let mut pages      = BTreeMap::new();

    for line in reader.lines().filter_map(|result| result.ok()) {
      let data : Vec<&str> = line.split(" ").collect();
      match data.len()  {
        0 => {
          errorln!("empty line found");
          continue;
        },
        1 => {
          errorln!("{} contains no categories", data[0]);
          continue;
        }
        _ => {}
      }

      let key = data[0];

      let page = pages.entry(String::from(key))
                      .or_insert(BTreeSet::new());

      for &category in data[1..].iter() {
        let category = String::from(category);

        page.insert(category.clone());
        categories.entry(category.clone())
                  .or_insert(BTreeSet::new())
                  .insert(String::from(key));
      }
    }

    let ret = Recommender {
      categories: categories,
      pages: pages
    };
    
    return ret;
  }

  /// Finds the most closely related values in the set
  pub fn recommend(self, categories : Vec<String>) -> Vec<String> {
    let mut ranked_pages = BTreeMap::new();

    // Gather all the pages belonging to each category
    // and see store its similarity against the list.
    //
    // This is only done once per page.
    for category in categories.iter() {
      match self.categories.get(category) {
        Some(self_cat) => {
          for page in self_cat.iter() {
            match self.pages.get(page) {
              Some(pages) => {
                ranked_pages.entry(page.clone())
                            .or_insert(
                                Recommender::similarity(categories.iter(), pages.iter()));
              },
              _ => {}
            }
          }
        },
        _ => {}
      }
    }

    // Sort the pages by their similarity
    let mut keys : Vec<String> = ranked_pages.keys().cloned().collect();
    let mut values : Vec<u64> = ranked_pages.values().cloned().collect();
    sbsqsort::quicksort(&mut values, &mut keys);

    return keys;
  }

  /// Uses a Jaccard Index to measure similarity between sets of categories
  fn similarity<'a,
                 S: Iterator<Item = &'a String>,
                 T: Clone + Iterator<Item = &'a String>>
               (a : S, b : T) -> u64 {
     let mut found_in_both = 0;
     let mut cnt_a = 0;
     let mut cnt_b = 0;

     for _ in b.clone() { cnt_b += 1; }
     for c in a {
      cnt_a += 1;
      for d in b.clone() {
        if c == d {
          found_in_both += 1;
          break;
        }
      }
     }

     return 1000 * ((2 * found_in_both) as u64 / (cnt_a + cnt_b - found_in_both)) as u64;
  }
}

