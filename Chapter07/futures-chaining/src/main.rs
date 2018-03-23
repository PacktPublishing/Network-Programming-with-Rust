extern crate futures;
extern crate futures_cpupool;
extern crate tokio_timer;
extern crate rand;

use futures::future::select_ok;
use std::time::Duration;

use futures::Future;
use futures_cpupool::CpuPool;
use tokio_timer::Timer;
use std::thread;
use rand::{thread_rng, Rng};

fn player_one() -> &'static str {
    let d = thread_rng().gen_range::<u64>(1, 5);
    thread::sleep(Duration::from_secs(d));
    "player_one"
}

fn player_two() -> &'static str {
    let d = thread_rng().gen_range::<u64>(1, 5);
    thread::sleep(Duration::from_secs(d));
    "player_two"
}

fn main() {
    let pool = CpuPool::new_num_cpus();
    let timer = Timer::default();

    let timeout = timer.sleep(Duration::from_secs(3))
        .then(|_| Err(()));

    let one = pool.spawn_fn(|| {
        Ok(player_one())
    });

    let two = pool.spawn_fn(|| {
        Ok(player_two())
    });
    
    let tasks = vec![one, two];
    let winner = select_ok(tasks).select(timeout).map(|(result, _)| result);
    let result = winner.wait().ok();
    match result {
        Some(("player_one", _)) => println!("Player one won"),
        Some(("player_two", _)) => println!("Player two won"),
        Some((_, _)) | None => println!("Timed out"),
    }
}