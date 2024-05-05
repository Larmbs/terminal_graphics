use std::time::{Duration, SystemTime};


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

        while let Ok(elapsed) = self.last_time.elapsed() {
            if time > elapsed {
                break;
            }
        }

        self.last_time = SystemTime::now();
    }
}

