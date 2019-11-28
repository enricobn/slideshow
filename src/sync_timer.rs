use std::ops::Sub;
use std::time::Instant;
use std::time::Duration;

pub struct SyncTimer {
    start: Instant,
    events: Vec<SyncEvent>,
}

impl SyncTimer {

    pub fn new() -> SyncTimer {
        SyncTimer{start: Instant::now(), events: Vec::new()}
    }

    pub fn add(&mut self, event: SyncEvent) {
        self.events.push(event);
    }

    pub fn fired(&mut self) -> Vec<&'static str> {
        let mut result = Vec::new();

        let now = Instant::now();
        for event in self.events.iter_mut() {
            if !event.done {
                let elapsed = now.sub(event.start);
                
                if elapsed > event.after {
                    if event.recurring {
                        event.start = now;
                    } else {
                        event.done = true;
                    }
                    result.push(event.id);
                }
            }
        }

        result
    }

}

pub struct SyncEvent {
    id: &'static str,
    start: Instant,
    after: Duration,
    recurring: bool,
    done: bool,
}

impl SyncEvent {

    pub fn new(id: &'static str, after: Duration, recurring: bool) -> SyncEvent {
        SyncEvent{ id, start: Instant::now(), after, done: false, recurring }
    }

}