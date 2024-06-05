use notify::{Error, Event, RecommendedWatcher, RecursiveMode, Watcher};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use std::sync::{Arc, Mutex};

const CONFIG_PATH: &str = "config.toml";

fn main() -> anyhow::Result<()> {
    let config = load_config(CONFIG_PATH).expect("Unable to load config");

    // We wrap the data a mutex under an atomic reference counted pointer
    // to guarantee that the config won't be read and written to at the same time.
    let config = Arc::new(Mutex::new(config));
    let cloned_config = Arc::clone(&config);

    // We listen to file changes by giving Notify
    // a function that will get called when events happen
    let mut watcher =
        // To make sure that the config lives as long as the function
        // we need to move the ownership of the config inside the function
        RecommendedWatcher::new(move |result: Result<Event, Error>| {
            let event = result.expect("Unable to watch modified event");

            if event.kind.is_modify() {
                println!("something is modified");
                match load_config(CONFIG_PATH) {
                    Ok(new_config) => *cloned_config.lock().unwrap() = new_config, // Safety
                    Err(error) => eprint!("Error reloading config: {:?}", error),
                }
            }
        },notify::Config::default())?;

    watcher.watch(Path::new(CONFIG_PATH), RecursiveMode::Recursive)?;

    // We added thread sleep here to run this watcher until Duration what we have defined
    std::thread::sleep(std::time::Duration::from_secs(1000));

    Ok(())
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub audio_folder_path: String,
    pub messages: Messages,
}

/// The key is the audio file name
type Messages = HashMap<String, Message>;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    pub display_name: String,
    pub volume: f32,
}

pub fn load_config(path: &str) -> Result<Config, Box<dyn std::error::Error>> {
    let config: Config = toml_edit::easy::from_str(&std::fs::read_to_string(path)?)?;
    println!("new config: {:?}", config);
    Ok(config)
}
