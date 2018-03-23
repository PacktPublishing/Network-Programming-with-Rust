
#![feature(test)]

extern crate rayon;
extern crate test;

use rayon::prelude::*;

fn filter_parallel(src: Vec<u64>) -> Vec<u64> {
    src.par_iter()
        .filter(|x| *x % 2 != 0)
        .map(|x| x * x)
        .collect()
}


fn filter_sequential(src: Vec<u64>) -> Vec<u64> {
    src.iter().filter(|x| *x % 2 != 0).map(|x| x * x).collect()
}

fn main() {
    let nums_one = (1..10).collect();
    println!("{:?}", filter_sequential(nums_one));

    let nums_two = (1..10).collect();
    println!("{:?}", filter_parallel(nums_two));
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_filter_sequential(b: &mut Bencher) {
        b.iter(|| filter_sequential((1..1000).collect::<Vec<u64>>()));
    }

    #[bench]
    fn bench_filter_parallel(b: &mut Bencher) {
        b.iter(|| filter_parallel((1..1000).collect::<Vec<u64>>()));
    }
}
