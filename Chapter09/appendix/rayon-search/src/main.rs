extern crate rayon;

use std::fmt::Debug;
use rayon::scope;

fn binary_search_rayon<T: Ord + Send + Copy + Sync + Debug>(src: &mut [T], el: T) -> bool {
    src.sort();
    let mid = src.len() / 2;
    let srcmid = src[mid];
    if src.len() == 1 && src[0] != el {
        return false;
    }
    if el == srcmid {
        true
    } else {
        let mut left_result = false;
        let mut right_result = false;
        let (left, right) = src.split_at_mut(mid);
        scope(|s| if el < srcmid {
            s.spawn(|_| left_result = binary_search_rayon(left, el))
        } else {
            s.spawn(|_| right_result = binary_search_rayon(right, el))
        });
        left_result || right_result
    }
}

fn binary_search_recursive<T: Ord + Send + Copy>(src: &mut [T], el: T) -> bool {
    src.sort();
    let mid = src.len() / 2;
    let srcmid = src[mid];
    if src.len() == 1 && src[0] != el {
        return false;
    }
    if el == srcmid {
        true
    } else {
        let (left, right) = src.split_at_mut(mid);
        if el < srcmid {
            binary_search_recursive(left, el)
        } else {
            binary_search_recursive(right, el)
        }
    }
}

fn main() {
    let mut v = vec![100, 12, 121, 1, 23, 35];
    println!("{}", binary_search_recursive(&mut v, 5));
    println!("{}", binary_search_rayon(&mut v, 5));
    println!("{}", binary_search_rayon(&mut v, 100));
}