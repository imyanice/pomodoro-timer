use std::path::PathBuf;
use std::thread;
use std::time::Duration;
use serde::{Deserialize, Serialize};
slint::include_modules!();

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Config {
    pub enabled: bool,
}

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;
    ui.on_start_timer({
        let ui_handle = ui.as_weak();
        move || {
            let mut config = get_config();
            config.enabled = !config.enabled;
            set_config(config);
            let app = ui_handle.unwrap();
            if get_config().enabled {
                app.set_state("Stop".into())
            } else {
                app.set_state("Start".into())
            }
        }
    });


    let ui_handle = ui.as_weak();
    std::thread::spawn(move || {
        let mut session_time = 60 * 25;
        let mut break_time = 60 * 5;
        loop {

            if get_config().enabled {
                println!("gi3");
                let ui = ui_handle.clone();

                if session_time > 0 {
                    let time_minutes = session_time / 60;
                    let time_seconds = session_time - time_minutes * 60;
                    println!("{:?}", time_seconds);
                    slint::invoke_from_event_loop(move || ui.unwrap().set_time((time_minutes.to_string() + ":" + &time_seconds.to_string()).into())).expect("TODO: panic message");
                    session_time -= 1;
                } else if break_time > 0 {
                } else if break_time == 0 {
                    session_time = 60 * 25;
                    break_time = 60 * 5;
                }
                thread::sleep(Duration::from_millis(1000));
            }
        }

    });
    ui.run()
}

pub fn get_config() -> Config {
    let path = data_dir().join("config.toml");

    confy::load_path(path).expect("Could not load config")
}

pub fn set_config(config: Config) {
    let path = data_dir().join("config.toml");

    confy::store_path(path, config).expect("Could not save config");
}

pub fn data_dir() -> PathBuf {
    let home_dir = std::env::var_os("HOME").map(PathBuf::from).unwrap();
    home_dir
        .join("Library/Application Support")
        .join("me.yanice.pomodoro-timer")
}