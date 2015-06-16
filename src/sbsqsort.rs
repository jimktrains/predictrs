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
