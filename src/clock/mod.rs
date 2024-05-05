use std::time::{Duration, SystemTime};
use std::thread;

pub struct Clock {
    last_time: SystemTime,
}

impl Clock {
    pub fn new() -> Self {
        Clock {
            last_time: SystemTime::now(),
        }
    }

    pub fn tick(&mut self, time: Duration) {
        loop {
            if let Ok(elapsed) = self.last_time.elapsed() {
                if time >= elapsed {
                    break;
                }
            } else {
                break;
            }

            thread::sleep(Duration::from_millis(1));
        }

        self.last_time = SystemTime::now();
    }
}

