use std::error::Error;
use std::io::prelude::*;
use std::process::{Command, Stdio};
use std::collections::HashMap;
use serde_json::{self, Value};

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
                        output_map = output_map;
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

pub fn get_mpv_property<T: TypeHandler>(socket: &str, property: &str) -> Result<T, String> {
    let ipc_string = format!("{{ \"command\": [\"get_property\",\"{}\"] }}\n", property);

    match serde_json::from_str::<Value>(&send_command_wait(socket, &ipc_string)) {
        Ok(val) => T::get_value(val),
        Err(why) => error!("Error while getting property: {}", why),
    }
}

pub fn get_mpv_property_string(socket: &str, property: &str) -> Result<String, String> {
    let ipc_string = format!("{{ \"command\": [\"get_property\",\"{}\"] }}\n", property);
    match serde_json::from_str::<Value>(&send_command_wait(socket, &ipc_string)) {
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

pub fn set_mpv_property<T: TypeHandler>(socket: &str, property: &str, value: T) -> Option<String> {
    let ipc_string = format!("{{ \"command\": [\"set_property\", \"{}\", {}] }}\n",
                             property,
                             value.as_string());
    match serde_json::from_str::<Value>(&send_command_wait(socket, &ipc_string)) {
        Ok(_) => None,
        Err(why) => Some(why.description().to_string()),
    }
}

pub fn run_mpv_command(socket: &str, command: &str, args: &Vec<&str>) -> Option<String> {
    let mut ipc_string = format!("{{ \"command\": [\"{}\"", command);
    if args.len() > 0 {
        for arg in args.iter() {
            ipc_string.push_str(&format!(", \"{}\"", arg));
        }
    }
    ipc_string.push_str("] }\n");
    ipc_string = ipc_string;
    match serde_json::from_str::<Value>(&send_command_wait(socket, &ipc_string)) {
        Ok(feedback) => {
            if let Value::String(ref error) = feedback["error"] {
                if error == "success" {
                    None
                } else {
                    Some(error.to_string())
                }
            } else {
                error!("Error: Unexpected result received");
            }
        }
        Err(why) => Some(why.description().to_string()),
    }
}

fn send_command_wait(socket: &str, command: &str) -> String {
    // Spawn the `socat` command
    let process = match Command::new("socat")
              .arg("-")
              .arg(socket)
              .stdin(Stdio::piped())
              .stdout(Stdio::piped())
              .spawn() {
        Err(why) => panic!("couldn't spawn socat: {}", why.description()),
        Ok(process) => process,
    };

    // `stdin` has type `Option<ChildStdin>`, but since we know this instance
    // must have one, we can directly `unwrap` it.
    match process.stdin.unwrap().write_all(command.as_bytes()) {
        Err(why) => panic!("couldn't write to socat stdin: {}", why.description()),
        Ok(result) => result,
    }

    // Because `stdin` does not live after the above calls, it is `drop`ed,
    // and the pipe is closed.
    //
    // This is very important, otherwise `socat` wouldn't start processing the
    // input we just sent.

    // The `stdout` field also has type `Option<ChildStdout>` so must be unwrapped.
    let mut s = String::new();
    match process.stdout.unwrap().read_to_string(&mut s) {
        Err(why) => panic!("couldn't read socat stdout: {}", why.description()),
        Ok(_) => return s,
    }
}
