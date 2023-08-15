use std::fs;
use std::os::unix::fs::FileTypeExt;
use std::path::PathBuf;
use std::sync;

use notify::{event, RecursiveMode, Watcher};
use serde_json::Value;

use mpvc::Mpv;

fn watch() -> Result<(), notify::Error> {
    let (tx, rx) = sync::mpsc::channel();
    let mut watcher = notify::recommended_watcher(tx)?;
    watcher.watch("/tmp".as_ref(), RecursiveMode::NonRecursive)?;
    for event in rx {
        if let Ok(e) = event {
            if e.kind == event::EventKind::Create(event::CreateKind::File) &&
                    e.paths.contains(&PathBuf::from("/tmp/mpv.sock")) {
                break;
            }
        }
    }
    Ok(())
}

fn print(idle: &bool, pause: &Option<String>, position: &Option<u64>, count: &Option<u64>, title: &Option<String>) {
    match (idle, pause, position, count, title) {
        (false, Some(p), Some(o), Some(c), Some(t)) => println!("{} #{}/{} - {}", p, o, c, t),
        _ => (),
    }
}

fn main() {
    let mut mpv = loop {
        match fs::metadata("/tmp/mpv.sock") {
            Ok(metadata) => {
                if metadata.file_type().is_socket() {
                    match Mpv::connect("/tmp/mpv.sock") {
                        Ok(instance) => break instance,
                        Err(_) => {
                            let _ = fs::remove_file("/tmp/mpv.sock");
                        }
                    }
                } else {
                    println!("N/A");
                    let _ = watch();
                }
            }
            Err(_) => {
                println!("N/A");
                let _ = watch();
            }
        }
    };

    for (i, property) in ["idle-active", "pause", "playlist-pos-1", "playlist-count", "media-title"].into_iter().enumerate() {
        if let Err(_) = mpv.observe_property(i as isize + 1, property) {
            return;
        }
    }

    let mut idle = false;
    let mut pause = Option::<String>::None;
    let mut position = Option::<u64>::None;
    let mut count = Option::<u64>::None;
    let mut title = Option::<String>::None;
    while let Ok(event) = mpv.listen() {
        if event.get("event") == Some(&Value::String("property-change".to_string())) {
            if let Some(Value::String(ref prop)) = event.get("name") {
                match prop.as_str() {
                    "idle-active" => {
                        if let Some(Value::Bool(b)) = event.get("data") {
                            idle = *b;
                            if idle {
                                println!("N/A");
                            }
                        }
                    }
                    "pause" => {
                        if let Some(Value::Bool(b)) = event.get("data") {
                            pause = if *b {
                                Some("⏸".to_string())
                            } else {
                                Some("⏵".to_string())
                            };
                            print(&idle, &pause, &position, &count, &title);
                        }
                    }
                    "playlist-pos-1" => {
                        if let Some(Value::Number(ref n)) = event.get("data") {
                            if let Some(u) = n.as_u64() {
                                position = Some(u);
                                print(&idle, &pause, &position, &count, &title);
                            }
                        }
                    }
                    "playlist-count" => {
                        if let Some(Value::Number(ref n)) = event.get("data") {
                            if let Some(u) = n.as_u64() {
                                count = Some(u);
                                print(&idle, &pause, &position, &count, &title);
                            }
                        }
                    }
                    "media-title" => {
                        if let Some(Value::String(ref str)) = event.get("data") {
                            title = Some(str.clone());
                            print(&idle, &pause, &position, &count, &title);
                        }
                    }
                    _ => continue,
                }
            }
        }
    }
}
