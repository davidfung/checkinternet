use chrono::{DateTime, Local};
use online::check;
use std::{thread, time};

const CHECK_INTERVAL: u64 = 5;

fn main() {
    loop {
        if check(None).is_ok() {
            if false {
                let now: DateTime<Local> = Local::now();
                println!("Online @ {}", now);
            }
        } else {
            let now: DateTime<Local> = Local::now();
            println!("Offline @ {}", now);
        }
        let interval = time::Duration::from_secs(CHECK_INTERVAL);
        thread::sleep(interval);
    }
}
