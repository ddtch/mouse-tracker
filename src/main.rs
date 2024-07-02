mod db;
mod input;

use db::ActivityTracker;
use input::InputTracker;
use std::time::Duration;
use std::{
    sync::{Arc, Mutex},
    thread,
};
use winit::event_loop::EventLoop;

fn main() {
    let db_path = "activity.db";
    let tracker = ActivityTracker::new(db_path).expect("Failed to initialize database");
    let input_tracker = Arc::new(Mutex::new(InputTracker::new()));
    let write_interval = Duration::from_secs(60); // default to 1 minute

    let input_tracker_clone = Arc::clone(&input_tracker);
    thread::spawn(move || {
        let mut event_loop = EventLoop::new();
        loop {
            {
                let mut tracker = input_tracker_clone.lock().unwrap();
                tracker.track(&mut event_loop);
            }
            thread::sleep(Duration::from_millis(16)); // roughly 60 FPS
        }
    });

    loop {
        thread::sleep(write_interval);
        let mut input_tracker = input_tracker.lock().unwrap();
        tracker
            .log_activity(
                input_tracker.left_clicks,
                input_tracker.right_clicks,
                input_tracker.movement,
                input_tracker.scrolls,
            )
            .expect("Failed to log activity");
        input_tracker.left_clicks = 0;
        input_tracker.right_clicks = 0;
        input_tracker.movement = 0.0;
        input_tracker.scrolls = 0;
    }
}
