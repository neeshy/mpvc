use std::fs;
use std::os::unix::fs::FileTypeExt;
use std::path::{Path, PathBuf};
use std::sync;

use notify::{RecursiveMode, Watcher, event::{CreateKind, Event, EventKind}};
use serde_json::Value;

use mpvc::Mpv;

fn watch() -> Result<(), notify::Error> {
    let (tx, rx) = sync::mpsc::channel();
    let mut watcher = notify::recommended_watcher(tx)?;
    watcher.watch(Path::new("/tmp"), RecursiveMode::NonRecursive)?;
    let path = PathBuf::from("/tmp/mpv.sock");
    for event in rx {
        if let Ok(Event { kind: EventKind::Create(CreateKind::File), paths, attrs: _ }) = event {
            if paths.contains(&path) {
                break;
            }
        }
    }
    Ok(())
}

fn print(idle: bool, pause: &Option<&str>, position: &Option<u64>, count: &Option<u64>, title: &Option<String>) {
    if let (false, Some(p), Some(o), Some(c), Some(t)) = (idle, pause, position, count, title) {
        println!("{} #{}/{} - {}", p, o, c, t);
    }
}

fn main() {
    let mut mpv = loop {
        if let Ok(metadata) = fs::metadata("/tmp/mpv.sock") {
            if metadata.file_type().is_socket() {
                if let Ok(instance) = Mpv::connect("/tmp/mpv.sock") {
                    break instance;
                } else {
                    let _ = fs::remove_file("/tmp/mpv.sock");
                }
            } else {
                println!("");
                let _ = watch();
            }
        } else {
            println!("");
            let _ = watch();
        }
    };

    let properties = ["idle-active", "pause", "playlist-pos-1", "playlist-count", "media-title"];
    for (i, property) in properties.into_iter().enumerate() {
        if let Err(_) = mpv.observe_property(i as isize + 1, property) {
            return;
        }
    }

    let mut idle = false;
    let mut pause = Option::<&str>::None;
    let mut position = Option::<u64>::None;
    let mut count = Option::<u64>::None;
    let mut title = Option::<String>::None;
    while let Ok(event) = mpv.listen() {
        if let Some(Value::String(e)) = event.get("event") {
            if e != "property-change" {
                continue;
            }
        } else {
            continue;
        }

        let Some(Value::String(prop)) = event.get("name") else { continue; };
        match prop.as_str() {
            "idle-active" => {
                if let Some(Value::Bool(b)) = event.get("data") {
                    idle = *b;
                    if idle {
                        println!("⏹ Stopped");
                    }
                }
            }
            "pause" => {
                if let Some(Value::Bool(b)) = event.get("data") {
                    pause = Some(if *b { "⏸" } else { "⏵" });
                    print(idle, &pause, &position, &count, &title);
                }
            }
            "playlist-pos-1" => {
                if let Some(Value::Number(n)) = event.get("data") {
                    if let Some(u) = n.as_u64() {
                        position = Some(u);
                        print(idle, &pause, &position, &count, &title);
                    }
                }
            }
            "playlist-count" => {
                if let Some(Value::Number(n)) = event.get("data") {
                    if let Some(u) = n.as_u64() {
                        count = Some(u);
                        print(idle, &pause, &position, &count, &title);
                    }
                }
            }
            "media-title" => {
                if let Some(Value::String(str)) = event.get("data") {
                    title = Some(str.clone());
                    print(idle, &pause, &position, &count, &title);
                }
            }
            _ => continue,
        }
    }
}
