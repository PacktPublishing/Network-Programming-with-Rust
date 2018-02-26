use std::thread;

fn main() {
    for i in 1..10 {
        let handle = thread::spawn(move || {
            println!("Hello from thread number {}", i);
        });
        let _ = handle.join();
    }
}
