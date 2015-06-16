use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::io::Write;
use std::collections::btree_map::BTreeMap;
use std::collections::btree_set::BTreeSet;
use std::iter::Iterator;
use std::cmp::Ordering;

// borrowed from https://github.com/servo/rust-quicksort/blob/master/lib.rs
fn quicksort_helper<T, S, F>(arr: &mut Vec<T>, keys: &mut Vec<S>, left: isize, right: isize, compare: &F)
where F: Fn(&T, &T) -> Ordering {
    if right <= left {
        return
    }

    let mut i: isize = left - 1;
    let mut j: isize = right;
    let mut p: isize = i;
    let mut q: isize = j;
    unsafe {
        let v: *mut T = &mut arr[right as usize];
        loop {
            i += 1;
            while compare(&arr[i as usize], &*v) == Ordering::Less {
                i += 1
            }
            j -= 1;
            while compare(&*v, &arr[j as usize]) == Ordering::Less {
                if j == left {
                    break
                }
                j -= 1;
            }
            if i >= j {
                break
            }
            arr.swap(i as usize, j as usize);
            if compare(&arr[i as usize], &*v) == Ordering::Equal {
                p += 1;
                arr.swap(p as usize, i as usize);
                keys.swap(p as usize, i as usize);
            }
            if compare(&*v, &arr[j as usize]) == Ordering::Equal {
                q -= 1;
                arr.swap(j as usize, q as usize);
                keys.swap(j as usize, q as usize);
            }
        }
    }

    arr.swap(i as usize, right as usize);
    keys.swap(i as usize, right as usize);
    j = i - 1;
    i += 1;
    let mut k: isize = left;
    while k < p {
        arr.swap(k as usize, j as usize);
        keys.swap(k as usize, j as usize);
        k += 1;
        j -= 1;
        assert!(k < arr.len() as isize);
    }
    k = right - 1;
    while k > q {
        arr.swap(i as usize, k as usize);
        keys.swap(i as usize, k as usize);
        k -= 1;
        i += 1;
        assert!(k != 0);
    }

    quicksort_helper(arr, keys, left, j, compare);
    quicksort_helper(arr, keys, i, right, compare);
}


/// An in-place quicksort.
///
/// The algorithm is from Sedgewick and Bentley, "Quicksort is Optimal":
///     http://www.cs.princeton.edu/~rs/talks/QuicksortIsOptimal.pdf
pub fn quicksort_by<T, S, F>(arr: &mut Vec<T>, keys: &mut Vec<S>, compare: F) where F: Fn(&T, &T) -> Ordering {
    if arr.len() <= 1 {
        return
    }

    let len = arr.len();
    quicksort_helper(arr, keys, 0, (len - 1) as isize, &compare);
}

/// An in-place quicksort for ordered items.
#[inline]
pub fn quicksort<T, S>(arr: &mut Vec<T>, keys: &mut Vec<S>) where T: Ord {
    quicksort_by(arr, keys, |a, b| a.cmp(b))
}


macro_rules! errorln(
    ($($arg:tt)*) => (
        match writeln!(&mut ::std::io::stderr(), $($arg)* ) {
            Ok(_) => {},
            Err(x) => panic!("Unable to write to stderr: {}", x),
        }
    )
);

struct Recommender {
  categories : BTreeMap<String, BTreeSet<String>>,
  pages : BTreeMap<String, BTreeSet<String>>,
}

fn load_knowlege(path : String) -> Recommender  {
  let file = match File::open(path) {
      Ok(file) => file,
      Err(..)  => panic!("room"),
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

fn recommend(rec : Recommender, categories : Vec<String>) -> Vec<String> {
  let mut ranked_pages = BTreeMap::new();

  for category in categories.iter() {
    match rec.categories.get(category) {
      Some(rec_cat) => {
        for page in rec_cat.iter() {
          match rec.pages.get(page) {
            Some(pages) => {
              ranked_pages.entry(page.clone())
                          .or_insert(
                              similarity(categories.iter(), pages.iter()));
            },
            _ => {}
          }
        }
      },
      _ => {}
    }
  }

  let mut keys : Vec<String> = ranked_pages.keys().cloned().collect();
  let mut values : Vec<u64> = ranked_pages.values().cloned().collect();
  quicksort(&mut values, &mut keys);

  return keys;
}

fn similarity<'a, S: Iterator<Item = &'a String>, T: Clone + Iterator<Item = &'a String>>(a : S, b : T) -> u64 {
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

   return 1000 * ((2 * found_in_both) as u64 / (cnt_a + cnt_b)) as u64;
}

fn main() {
  let rec = load_knowlege(String::from("test_file.txt"));
  print!("{:?}", recommend(rec, vec![String::from("Shopping.Publications.University_Presses")]));
}

