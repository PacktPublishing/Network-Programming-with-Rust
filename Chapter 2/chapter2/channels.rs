use std::thread;
use std::sync::mpsc;

fn main() {
    let rhs = vec![10, 20, 30, 40, 50, 60, 70];
    let lhs = vec![1, 2, 3, 4, 5, 6, 7];
    let (tx, rx) = mpsc::channel();

    assert_eq!(rhs.len(), lhs.len());
    for i in 1..rhs.len() {
        let rhs = rhs.clone();
        let lhs = lhs.clone();
        let tx = tx.clone();
        let handle = thread::spawn(move || {
            let s = format!("Thread {} added {} and {}, result {}", i, rhs[i], lhs[i], rhs[i] + lhs[i]);
            tx.clone().send(s).unwrap();
        });
        let _ = handle.join().unwrap();
    }

    drop(tx);
    for result in rx {
        println!("{}", result);
    }
}
