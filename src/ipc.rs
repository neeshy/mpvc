use std::error::Error;
use std::io::prelude::*;
use std::io::BufReader;
use std::os::unix::net::UnixStream;
use std::net::Shutdown;
use std::iter::Iterator;
use std::collections::HashMap;
use serde_json::{self, Value};

pub type Socket = String;

#[derive(Debug)]
pub struct PlaylistEntry {
    pub id: usize,
    pub filename: String,
    pub title: String,
    pub current: bool,
}

pub trait TypeHandler: Sized {
    fn get_value(value: Value) -> Result<Self, String>;
    fn as_string(&self) -> String;
}

impl TypeHandler for String {
    fn get_value(value: Value) -> Result<String, String> {
        if let Value::Object(map) = value {
            if let Value::String(ref error) = map["error"] {
                if error == "success" && map.contains_key("data") {
                    if let Value::String(ref s) = map["data"] {
                        Ok(s.to_string())
                    } else {
                        Err("Value did not contain a String".to_string())
                    }
                } else {
                    Err(error.to_string())
                }
            } else {
                Err("Unexpected value received".to_string())
            }
        } else {
            Err("Unexpected value received".to_string())
        }
    }

    fn as_string(&self) -> String {
        self.to_string()
    }
}

impl TypeHandler for bool {
    fn get_value(value: Value) -> Result<bool, String> {
        if let Value::Object(map) = value {
            if let Value::String(ref error) = map["error"] {
                if error == "success" && map.contains_key("data") {
                    if let Value::Bool(ref b) = map["data"] {
                        Ok(*b)
                    } else {
                        Err("Value did not contain a bool".to_string())
                    }
                } else {
                    Err(error.to_string())
                }
            } else {
                Err("Unexpected value received".to_string())
            }
        } else {
            Err("Unexpected value received".to_string())
        }
    }
    fn as_string(&self) -> String {
        if *self {
            "true".to_string()
        } else {
            "false".to_string()
        }
    }
}

impl TypeHandler for f64 {
    fn get_value(value: Value) -> Result<f64, String> {
        if let Value::Object(map) = value {
            if let Value::String(ref error) = map["error"] {
                if error == "success" && map.contains_key("data") {
                    if let Value::Number(ref num) = map["data"] {
                        Ok(num.as_f64().unwrap())
                    } else {
                        Err("Value did not contain a f64".to_string())
                    }
                } else {
                    Err(error.to_string())
                }
            } else {
                Err("Unexpected value received".to_string())
            }
        } else {
            Err("Unexpected value received".to_string())
        }
    }

    fn as_string(&self) -> String {
        self.to_string()
    }
}

impl TypeHandler for usize {
    fn get_value(value: Value) -> Result<usize, String> {
        if let Value::Object(map) = value {
            if let Value::String(ref error) = map["error"] {
                if error == "success" && map.contains_key("data") {
                    if let Value::Number(ref num) = map["data"] {
                        Ok(num.as_u64().unwrap() as usize)
                    } else {
                        Err("Value did not contain an usize".to_string())
                    }
                } else {
                    Err(error.to_string())
                }
            } else {
                Err("Unexpected value received".to_string())
            }
        } else {
            Err("Unexpected value received".to_string())
        }
    }

    fn as_string(&self) -> String {
        self.to_string()
    }
}

impl TypeHandler for HashMap<String, String> {
    fn get_value(value: Value) -> Result<HashMap<String, String>, String> {
        if let Value::Object(map) = value {
            if let Value::String(ref error) = map["error"] {
                if error == "success" && map.contains_key("data") {
                    if let Value::Object(ref inner_map) = map["data"] {
                        let mut output_map: HashMap<String, String> = HashMap::new();
                        for (ref key, ref value) in inner_map.iter() {
                            if let Value::String(ref val) = **value {
                                output_map.insert(key.to_string(), val.to_string());
                            }
                        }
                        let output_map = output_map;
                        Ok(output_map)
                    } else {
                        Err("Value did not contain a HashMap".to_string())
                    }
                } else {
                    Err(error.to_string())
                }
            } else {
                Err("Unexpected value received".to_string())
            }
        } else {
            Err("Unexpected value received".to_string())
        }
    }

    fn as_string(&self) -> String {
        format!("{:?}", self)
    }
}

impl TypeHandler for Vec<PlaylistEntry> {
    fn get_value(value: Value) -> Result<Vec<PlaylistEntry>, String> {
        if let Value::Object(map) = value {
            if let Value::String(ref error) = map["error"] {
                if error == "success" && map.contains_key("data") {
                    if let Value::Array(ref playlist_vec) = map["data"] {
                        let mut output: Vec<PlaylistEntry> = Vec::new();
                        for (id, entry) in playlist_vec.iter().enumerate() {
                            let mut filename: String = String::new();
                            let mut title: String = String::new();
                            let mut current: bool = false;
                            if let Value::String(ref f) = entry["filename"] {
                                filename = f.to_string();
                            }
                            if let Value::String(ref t) = entry["title"] {
                                title = t.to_string();
                            }
                            if let Value::Bool(ref b) = entry["current"] {
                                current = *b;
                            }
                            output.push(PlaylistEntry {
                                            id: id,
                                            filename: filename,
                                            title: title,
                                            current: current,
                                        });
                        }
                        let output = output;
                        Ok(output)
                    } else {
                        Err("Value did not contain a playlist".to_string())
                    }
                } else {
                    Err(error.to_string())
                }
            } else {
                Err("Unexpected value received".to_string())
            }
        } else {
            Err("Unexpected value received".to_string())
        }
    }

    fn as_string(&self) -> String {
        format!("{:?}", self)
    }
}

/// #Description
///
/// Retrieves the property value from mpv.
///
/// ##Supported types
/// - String
/// - bool
/// - HashMap<String, String> (e.g. for the 'metadata' property)
/// - Vec<PlaylistEntry> (for the 'playlist' property)
///
/// ##Input arguments
///
/// - **socket** defines the socket that ipc connects to
/// - **property** defines the mpv property that should be retrieved
///
/// #Example
/// ```
/// let paused: bool = get_mpv_property("/tmp/mpvsocket", "pause").unwrap();
/// let title: String = get_mpv_property("/tmp/mpvsocket", "media-title").unwrap();
/// ```
pub fn get_mpv_property<T: TypeHandler>(socket: &str, property: &str) -> Result<T, String> {
    let ipc_string = format!("{{ \"command\": [\"get_property\",\"{}\"] }}\n", property);

    match serde_json::from_str::<Value>(&send_command_sync(socket, &ipc_string)) {
        Ok(val) => T::get_value(val),
        Err(why) => error!("Error while getting property: {}", why),
    }
}

/// #Description
///
/// Retrieves the property value from mpv. Implemented for the following types:
/// The result is always of type String, regardless of the type of the value of the mpv property
///
/// ##Input arguments
///
/// - **socket** defines the socket that ipc connects to
/// - **property** defines the mpv property that should be retrieved
///
/// #Example
///
/// ```
/// let title = get_mpv_property("/tmp/mpvsocket", "media-title").unwrap();
/// ```
pub fn get_mpv_property_string(socket: &str, property: &str) -> Result<String, String> {
    let ipc_string = format!("{{ \"command\": [\"get_property\",\"{}\"] }}\n", property);
    match serde_json::from_str::<Value>(&send_command_sync(socket, &ipc_string)) {
        Ok(val) => {
            if let Value::Object(map) = val {
                if let Value::String(ref error) = map["error"] {
                    if error == "success" && map.contains_key("data") {
                        match map["data"] {
                            Value::Bool(b) => Ok(b.to_string()),
                            Value::Number(ref n) => Ok(n.to_string()),
                            Value::String(ref s) => Ok(s.to_string()),
                            Value::Array(ref array) => Ok(format!("{:?}", array)),
                            Value::Object(ref map) => Ok(format!("{:?}", map)),
                            _ => Err("Value contains an unsupported type".to_string()),
                        }
                    } else {
                        Err(error.to_string())
                    }
                } else {
                    Err("Unexpected value received".to_string())
                }
            } else {
                Err("Unexpected value received".to_string())
            }
        }
        Err(why) => error!("Error while getting property: {}", why),
    }
}

/// #Description
///
/// Sets the mpv property _<property>_ to _<value>_.
///
/// ##Supported types
/// - String
/// - bool
/// - HashMap<String, String> (no use-cases known)
/// - Vec<PlaylistEntry> (no use-cases known)
///
/// ##Input arguments
///
/// - **socket** defines the socket that ipc connects to
/// - **property** defines the mpv property that should be retrieved
/// - **value** defines the value of the given mpv property _<property>_
///
/// #Example
/// ```
/// set_mpv_property("/tmp/mpvsocket", "pause", true);
/// ```
pub fn set_mpv_property<T: TypeHandler>(socket: &str,
                                        property: &str,
                                        value: T)
                                        -> Result<(), String> {
    let ipc_string = format!("{{ \"command\": [\"set_property\", \"{}\", {}] }}\n",
                             property,
                             value.as_string());
    match serde_json::from_str::<Value>(&send_command_sync(socket, &ipc_string)) {
        Ok(_) => Ok(()),
        Err(why) => Err(why.description().to_string()),
    }
}

/// #Description
///
/// Runs mpv commands. The arguments are passed as a String-Vector reference:
///
/// #Example
/// ```
/// //Run command 'playlist-shuffle' which takes no arguments
/// run_mpv_command(&self.socket, "playlist-shuffle", &vec![]);
///
/// //Run command 'seek' which in this case takes two arguments
/// run_mpv_command(self, "seek", &vec!["0", "absolute"]);
/// ```
pub fn run_mpv_command(socket: &str, command: &str, args: &Vec<&str>) -> Result<(), String> {
    let mut ipc_string = format!("{{ \"command\": [\"{}\"", command);
    if args.len() > 0 {
        for arg in args.iter() {
            ipc_string.push_str(&format!(", \"{}\"", arg));
        }
    }
    ipc_string.push_str("] }\n");
    ipc_string = ipc_string;
    match serde_json::from_str::<Value>(&send_command_sync(socket, &ipc_string)) {
        Ok(feedback) => {
            if let Value::String(ref error) = feedback["error"] {
                if error == "success" {
                    Ok(())
                } else {
                    Err(error.to_string())
                }
            } else {
                error!("Error: Unexpected result received");
            }
        }
        Err(why) => Err(why.description().to_string()),
    }
}

/// #Description
///
/// Listens on socket <socket> for events and prints them in real-time to stdout.
/// This function contains an infinite-loop which keeps the application open indefinitely.
///
/// #Example
/// ```
/// listen("/tmp/mpvsocket");
/// ```
pub fn listen(socket: &str) {
    match UnixStream::connect(socket) {
        Ok(stream) => {
            let mut response = String::new();
            let mut reader = BufReader::new(&stream);
            loop {
                reader.read_line(&mut response).unwrap();
                match serde_json::from_str::<Value>(&response) {
                    Ok(e) => {
                        if let Value::String(ref name) = e["event"] {
                            println!("{}", name);
                        }
                    }
                    Err(why) => error!("{}", why.description().to_string()),
                }
                response.clear();
            }
        }
        Err(why) => error!("Error: Could not connect to socket: {}", why.description()),
    }
}

/// #Description
///
/// Listens on socket <socket> for events quits as soon as event <event> occurs.
///
/// #Example
/// ```
/// wait_for_event("/tmp/mpvsocket", "pause");
/// ```
pub fn wait_for_event(socket: &str, event: &str) {
    match UnixStream::connect(socket) {
        Ok(stream) => {
            let mut response = String::new();
            let mut reader = BufReader::new(&stream);
            loop {
                reader.read_line(&mut response).unwrap();
                match serde_json::from_str::<Value>(&response) {
                    Ok(e) => {
                        if let Value::String(ref name) = e["event"] {
                            if name.as_str() == event {
                                break;
                            }
                        }
                    }
                    Err(why) => error!("{}", why.description().to_string()),
                }
                response.clear();
            }
            stream.shutdown(Shutdown::Both).expect("socket shutdown");
        }
        Err(why) => error!("Error: Could not connect to socket: {}", why.description()),
    }
}

// pub fn observe_property(socket: &str, property: &str) -> String {
//     match UnixStream::connect(socket) {
//         Ok(mut stream) => {
//             let command = format!("{{ \"command\": [\"observe_property\", 1, \"{}\"] }}\n",
//                                   property);
//             match stream.write_all(command.as_bytes()) {
//                 Err(why) => error!("Error: Could not write to socket: {}", why.description()),
//                 Ok(_) => {
//                     let mut response = String::new();
//                     let mut reader = BufReader::new(&stream);
//                     loop {
//                         reader.read_line(&mut response).unwrap();
//                         println!("{}", response);
//                         response.clear();
//                     }
//                 }
//             }
//         }
//         Err(why) => error!("Error: Could not connect to socket: {}", why.description()),
//     }
// }

fn send_command_sync(socket: &str, command: &str) -> String {
    match UnixStream::connect(socket) {
        Ok(mut stream) => {
            match stream.write_all(command.as_bytes()) {
                Err(why) => error!("Error: Could not write to socket: {}", why.description()),
                Ok(_) => {
                    let mut response = String::new();
                    {
                        let mut reader = BufReader::new(&stream);
                        reader.read_line(&mut response).unwrap();
                    }
                    stream
                        .shutdown(Shutdown::Both)
                        .expect("shutdown function failed");
                    response
                }
            }
        }
        Err(why) => error!("Error: Could not connect to socket: {}", why.description()),
    }
}
