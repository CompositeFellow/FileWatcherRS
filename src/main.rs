extern crate notify;

use notify::{RecommendedWatcher, RecursiveMode, Watcher, Config, event, Event};
use std::sync::mpsc::channel;
use std::time::Duration;
use std::path::Path;
// use std::env;

fn main() {
    let (tx, rx) = channel();
    let config = Config::default().with_poll_interval(Duration::from_secs(100))
    ;


    let mut watcher:RecommendedWatcher = Watcher::new(tx, config).unwrap();
    let path_to_watch = Path::new("../");

    if path_to_watch.exists() {
        watcher.watch(path_to_watch, RecursiveMode::Recursive).unwrap();
    } else {
        println!("The specified path does not exist");
        return;
    }
    
    loop {
        match rx.recv(){
            Ok(Ok(event)) => {
                let kind = event.kind;
                let paths = event.paths;
                println!("Event: {:?}", kind);
                for path in paths {
                    let path_obj = Path::new(&path);
                    if let Some(filename) = path_obj.file_name() {
                        if let Some(filename_str) = filename.to_str() {
                            println!("Affected filename: {}", filename_str);
                        }
                    }
                }
            }
            Ok(Err(e)) => {
                println!("Event Error: {:?}", e);
            }
            Err(e) => println!("Error: {:?}", e)
        }
    }
}
