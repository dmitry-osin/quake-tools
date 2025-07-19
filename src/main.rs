pub mod config;
pub mod hotkeys;

use std::{
    sync::{Arc, Mutex},
    thread,
    time::{Duration, Instant},
};

use device_query::{DeviceQuery, DeviceState};
use slint::{SharedString, Timer};

use crate::{config::load_config, hotkeys::map_hotkey_to_keycode};

slint::include_modules!();

#[derive(Clone)]
struct TimerState {
    start_time: Option<Instant>,
    total_duration: u32,
}

impl TimerState {
    fn new(duration: u32) -> Self {
        Self {
            start_time: None,
            total_duration: duration,
        }
    }

    /// Starts the timer if it is not already running.
    ///
    /// If the timer is already running, it does nothing.
    fn start(&mut self) {
        if self.start_time.is_some() {
            return; // Timer is already running
        }

        self.start_time = Some(Instant::now());
    }

    /// Stops the timer if it is currently running.
    ///
    /// If the timer is not running, it does nothing.
    fn reset(&mut self) {
        if self.start_time.is_none() {
            return; // Timer is not runnig
        }

        self.start_time = None;
    }

    /// Returns the time left in seconds.
    ///
    /// If the timer has not been started, it returns the total duration.
    /// If the timer has finished, it returns 0.
    fn time_left(&self) -> u32 {
        if self.start_time.is_none() {
            return self.total_duration;
        }

        let elapsed = self.start_time.unwrap().elapsed().as_secs() as u32;

        if elapsed >= self.total_duration {
            0
        } else {
            self.total_duration - elapsed
        }
    }

    fn is_running(&self) -> bool {
        self.start_time.is_some()
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = AppWindow::new()?;
    app.set_window_title(SharedString::from("QuakeLive Tools"));

    let config = load_config().unwrap();

    // Initialize Quake Items timers
    let megahealth_timer = Arc::new(Mutex::new(TimerState::new(35)));
    let red_armor_timer = Arc::new(Mutex::new(TimerState::new(25)));

    // Callbacks for MegaHealth timer
    {
        let timer = megahealth_timer.clone();
        app.on_start_megahealth_timer(move || {
            let mut timer = timer.lock().unwrap();
            timer.start();
        });
    }

    {
        let timer = megahealth_timer.clone();
        app.on_reset_megahealth_timer(move || {
            let mut timer = timer.lock().unwrap();
            timer.reset();
        });
    }

    // Callbacks for Red Armor timer
    {
        let timer = red_armor_timer.clone();
        app.on_start_red_armor_timer(move || {
            let mut timer = timer.lock().unwrap();
            timer.start();
        });
    }
    {
        let timer = red_armor_timer.clone();
        app.on_reset_red_armor_timer(move || {
            let mut timer = timer.lock().unwrap();
            timer.reset();
        });
    }

    // Set update UI timer
    let app_weak = app.as_weak();
    let megahealth_timer_ui = megahealth_timer.clone();
    let red_armor_timer_ui = red_armor_timer.clone();

    let timer = Timer::default();
    timer.start(
        slint::TimerMode::Repeated,
        Duration::from_millis(100),
        move || {
            let upgrade: Option<AppWindow> = app_weak.upgrade();
            if let Some(app) = upgrade {
                // Update MegaHealth
                {
                    let mut timer = megahealth_timer_ui.lock().unwrap();
                    let time_left = timer.time_left();

                    if timer.is_running() && time_left == 0 {
                        timer.reset();
                    }

                    app.set_megahealth_timer(time_left as i32);
                    app.set_is_megahelth_active(timer.is_running() && time_left > 0);
                    app.set_is_megahealth_warning(time_left <= 10);
                    app.set_is_megahealth_critical(time_left <= 5);
                }

                // Update red armor
                {
                    let mut timer = red_armor_timer_ui.lock().unwrap();
                    let time_left = timer.time_left();

                    if timer.is_running() && time_left == 0 {
                        timer.reset();
                    }

                    app.set_red_armor_timer(time_left as i32);
                    app.set_is_red_armor_active(timer.is_running() && time_left > 0);
                    app.set_is_red_armor_warning(time_left <= 10);
                    app.set_is_red_armor_critical(time_left <= 5);
                }
            }
        },
    );

    let megahealth_timer_hotkey = megahealth_timer.clone();
    let red_armor_timer_hotkey = red_armor_timer.clone();

    let megahealth_hotkey = map_hotkey_to_keycode(config.megahealth_hotkey.as_str()).unwrap();
    let red_armor_hotkey = map_hotkey_to_keycode(config.red_armor_hotkey.as_str()).unwrap();

    thread::spawn(move || {
        let device_state = DeviceState::new();
        let mut last_keys = Vec::new();

        // separate thread for hotkeys
        loop {
            thread::sleep(Duration::from_millis(50));
            let keys = device_state.get_keys();

            if keys.contains(&megahealth_hotkey) && !last_keys.contains(&megahealth_hotkey) {
                let mut timer = megahealth_timer_hotkey.lock().unwrap();
                timer.start();
            }

            if keys.contains(&red_armor_hotkey) && !last_keys.contains(&red_armor_hotkey) {
                let mut timer = red_armor_timer_hotkey.lock().unwrap();
                timer.start();
            }

            // Update last keys
            last_keys = keys;
        }
    });

    app.run()?;
    Ok(())
}
