use std::{time::Duration, thread};

pub fn sleep_1_sec() {
    thread::sleep(Duration::from_secs(1));
}
