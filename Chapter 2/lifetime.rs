fn main() {
    let v1 = vec![1, 2, 3, 4, 5];
    let v2 = vec![1, 2];

    println!("{:?}", longer_vector(&v1, &v2));
}

fn longer_vector<'a>(x: &'a[i32], y: &'a[i32]) -> &'a[i32] {
    if x.len() > y.len() { x } else {y }
}
