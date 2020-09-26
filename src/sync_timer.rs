use std::ops::Sub;
use std::time::Duration;
use std::time::Instant;

pub struct SyncTimer {
    start: Instant,
    events: Vec<SyncEvent>,
}

impl SyncTimer {
    pub fn new() -> SyncTimer {
        SyncTimer { start: Instant::now(), events: Vec::new() }
    }

    pub fn add(&mut self, event: SyncEvent) {
        self.events.push(event);
    }

    pub fn fired(&mut self) -> Vec<&'static str> {
        let mut result = Vec::new();
        let mut to_remove = Vec::new();

        let now = Instant::now();
        let mut index : usize = 0;
        for event in self.events.iter_mut() {
            let elapsed = now.sub(event.start);

            if elapsed >= event.after {
                if event.recurring {
                    event.start = now;
                } else {
                    to_remove.push(index);
                }
                result.push(event.id);
            }
            index = index + 1;
        }

        for i in to_remove.iter() {
            self.events.remove(*i);
        }

        result
    }
}

pub struct SyncEvent {
    id: &'static str,
    start: Instant,
    after: Duration,
    recurring: bool,
}

impl SyncEvent {
    pub fn new(id: &'static str, after: Duration, recurring: bool) -> SyncEvent {
        SyncEvent { id, start: Instant::now(), after, recurring }
    }
}