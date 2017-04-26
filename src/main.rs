extern crate serde;
extern crate serde_json;
extern crate clap;

use std::error::Error;
use std::io::prelude::*;
use std::process::{Command, Stdio};
use serde_json::Value;
use clap::{App, Arg};

#[macro_use]
mod macros;

fn main() {

    let matches = App::new("mpvipc")
        .version("0.1")
        .author("Jonas Frei <freijon@gmail.com>")
        .about("A mpc-like cli tool for mpv which connects to existing mpv instances through sockets")
        .arg(Arg::with_name("socket")
                    .short("S")
                    .long("socket")
                    .value_name("PATH")
                    .help("Specifies the path to the socket")
                    .default_value("/tmp/mpvsocket")
                    .takes_value(true)
                    .require_equals(true))
        .arg(Arg::with_name("get_property")
                    .short("g")
                    .long("get-property")
                    .value_name("Property")
                    .help("Property that should be retrieved")
                    .takes_value(true)
                    .require_equals(true))
        .get_matches();

    // if let Value::Bool(b) = is_paused_property["data"] {
    //     set_mpv_property("pause", &format!("{}", !b));
    // }
    if let Some(property) = matches.value_of("get_property") {
        match get_mpv_property::<bool>(property) {
            Ok(data) => println!("{}", data),
            Err(msg) => error!("Error: {}", msg),
        }
    }
}

trait ValueInterpretor: Sized {
    fn get_value(value: Value) -> Result<Self, String>;
}

impl ValueInterpretor for String {
    fn get_value(value: Value) -> Result<String, String> {
        if let Value::Object(map) = value {
            if let Value::String(ref error) = map["error"] {
                if error == "success" {
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
}

impl ValueInterpretor for bool {
    fn get_value(value: Value) -> Result<bool, String> {
        if let Value::Object(map) = value {
            if let Value::String(ref error) = map["error"] {
                if error == "success" {
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
}

impl ValueInterpretor for f64 {
    fn get_value(value: Value) -> Result<f64, String> {
        if let Value::Object(map) = value {
            if let Value::String(ref error) = map["error"] {
                if error == "success" {
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
}

fn get_mpv_property<T: ValueInterpretor>(property: &str) -> Result<T, String> {
    let ipc_string = format!("{{ \"command\": [\"get_property\",\"{}\"] }}\n", property);

    match serde_json::from_str::<Value>(&send_command_wait(&ipc_string)) {
        Ok(val) => T::get_value(val),
        Err(why) => error!("Error while getting property: {}", why),
    }
}

fn set_mpv_property(property: &str, value: &str) -> Value {
    let ipc_string = format!("{{ \"command\": [\"set_property\", \"{}\", {}] }}\n",
                             property,
                             value);
    serde_json::from_str(&send_command_wait(&ipc_string)).unwrap()
}

fn send_command_wait(command: &str) -> String {
    // Spawn the `socat` command
    let process = match Command::new("socat")
              .arg("-")
              .arg("/tmp/mpvsocket")
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
