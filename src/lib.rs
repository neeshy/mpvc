extern crate log;
extern crate serde;
extern crate serde_json;

use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::io::prelude::*;
use std::io::BufReader;
use std::os::unix::net::UnixStream;

use log::debug;
use serde::ser::Serialize;
use serde_json::{json, Map, Value};

pub struct Mpv {
    stream: UnixStream,
    reader: BufReader<UnixStream>,
    name: String,
    responses: Vec<Map<String, Value>>,
    counter: i64,
}

impl Debug for Mpv {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> FmtResult {
        fmt.debug_tuple("Mpv").field(&self.name).finish()
    }
}

impl Drop for Mpv {
    fn drop(&mut self) {
        self._disconnect();
    }
}

impl Clone for Mpv {
    fn clone(&self) -> Self {
        let stream = self.stream.try_clone().expect("cloning UnixStream");
        let cloned_stream = stream.try_clone().expect("cloning UnixStream");
        Mpv {
            stream,
            reader: BufReader::new(cloned_stream),
            name: self.name.clone(),
            responses: self.responses.clone(),
            counter: self.counter,
        }
    }

    fn clone_from(&mut self, source: &Self) {
        let stream = source.stream.try_clone().expect("cloning UnixStream");
        let cloned_stream = stream.try_clone().expect("cloning UnixStream");
        *self = Mpv {
            stream,
            reader: BufReader::new(cloned_stream),
            name: source.name.clone(),
            responses: self.responses.clone(),
            counter: self.counter,
        }
    }
}

#[derive(Debug)]
pub enum Error {
    MpvError(String),
    JsonParseError(String),
    ConnectError(String),
    ReadError(String),
    WriteError(String),
    UnexpectedValue,
    MissingValue,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            Error::ConnectError(ref msg) => f.write_str(&format!("ConnectError: {}", msg)),
            Error::ReadError(ref msg) => f.write_str(&format!("ReadError: {}", msg)),
            Error::WriteError(ref msg) => f.write_str(&format!("WriteError: {}", msg)),
            Error::JsonParseError(ref msg) => f.write_str(&format!("JsonParseError: {}", msg)),
            Error::MpvError(ref msg) => f.write_str(&format!("MpvError: {}", msg)),
            Error::UnexpectedValue => f.write_str("Unexpected value received"),
            Error::MissingValue => f.write_str("Missing value"),
        }
    }
}

impl Mpv {
    pub fn connect(socket: &str) -> Result<Mpv, Error> {
        match UnixStream::connect(socket) {
            Ok(stream) => {
                let cloned_stream = stream.try_clone().expect("cloning UnixStream");
                Ok(Mpv {
                    stream,
                    reader: BufReader::new(cloned_stream),
                    name: String::from(socket),
                    responses: Vec::new(),
                    counter: -1,
                })
            }
            Err(why) => Err(Error::ConnectError(why.to_string())),
        }
    }

    fn _disconnect(&mut self) {
        self.stream.shutdown(std::net::Shutdown::Both).expect("socket disconnect");
    }

    pub fn disconnect(mut self) {
        self._disconnect();
    }

    fn _command(&mut self, command: &Vec<Value>) -> Result<Value, Error> {
        self.counter += 1;
        let v = json!({ "command": command, "request_id": self.counter });
        let c = &(v.to_string() + "\n");
        debug!("Command: {}", c.trim_end());
        self.stream.write_all(c.as_bytes())
            .map_err(|why| Error::WriteError(why.to_string()))?;
        loop {
            let mut response = String::new();
            let n = self.reader.read_line(&mut response)
                .map_err(|why| Error::ReadError(why.to_string()))?;
            if n == 0 {
                return Err(Error::ReadError("EOF reached".to_string()));
            }
            response = response.trim_end().to_string();
            debug!("Response: {}", response);

            let r = serde_json::from_str::<Value>(&response)
                .map_err(|why| Error::JsonParseError(why.to_string()))?;

            let mut map = if let Value::Object(map) = r {
                Ok(map)
            } else {
                Err(Error::UnexpectedValue)
            }?;

            if let Some(Value::Number(ref request_id)) = map.get("request_id") {
                if request_id.as_i64() != Some(self.counter) {
                    continue;
                }
            } else {
                if let Some(Value::String(_)) = map.get("event") {
                    self.responses.push(map);
                }
                continue;
            }

            let error = if let Some(Value::String(ref error)) = map.get("error") {
                Ok(error)
            } else {
                Err(Error::UnexpectedValue)
            }?;

            return if error == "success" {
                if map.contains_key("data") {
                    Ok(map["data"].take())
                } else {
                    Ok(Value::Null)
                }
            } else {
                Err(Error::MpvError(error.to_string()))
            };
        }
    }

    /// # Description
    ///
    /// Runs mpv commands. The arguments are passed as a String-Vector reference:
    ///
    /// ## Input arguments
    ///
    /// - **command**   defines the mpv command that should be executed
    /// - **args**      a slice of &str's which define the arguments
    ///
    /// # Example
    /// ```
    /// use mpvipc::{Mpv, Error};
    /// fn main() -> Result<(), Error> {
    ///     let mpv = Mpv::connect("/tmp/mpvsocket")?;
    ///
    ///     // Run command 'seek' which takes two arguments
    ///     mpv.command_arg("seek", &["0", "absolute"])?;
    ///     Ok(())
    /// }
    /// ```
    pub fn command_arg(&mut self, command: &str, args: &[&str]) -> Result<(), Error> {
        let mut a = Vec::with_capacity(args.len() + 1);
        a.push(Value::String(command.to_string()));
        a.extend(args.iter().map(|v| Value::String(v.to_string())));
        // XXX: Drop return value for now, change interface if needed
        self._command(&a).map(|_| ())
    }

    /// # Description
    ///
    /// Runs mpv commands.
    ///
    /// ## Input arguments
    ///
    /// - **command**   defines the mpv command that should be executed
    ///
    /// # Example
    /// ```
    /// use mpvipc::{Mpv, Error};
    /// fn main() -> Result<(), Error> {
    ///     let mpv = Mpv::connect("/tmp/mpvsocket")?;
    ///
    ///     // Run command 'playlist-shuffle' which takes no arguments
    ///     mpv.command("playlist-shuffle")?;
    ///     Ok(())
    /// }
    /// ```
    pub fn command(&mut self, command: &str) -> Result<(), Error> {
        self._command(&vec![Value::String(command.to_string())]).map(|_| ())
    }

    /// # Description
    ///
    /// Retrieves the property value from mpv.
    ///
    /// ## Supported types
    /// - String
    /// - bool
    /// - Map<String, String>
    /// - usize
    /// - f64
    ///
    /// ## Input arguments
    ///
    /// - **property** defines the mpv property that should be retrieved
    ///
    /// # Example
    /// ```
    /// use mpvipc::{Mpv, Error};
    /// fn main() -> Result<(), Error> {
    ///     let mpv = Mpv::connect("/tmp/mpvsocket")?;
    ///     let paused: bool = mpv.get_property("pause")?;
    ///     let title: String = mpv.get_property("media-title")?;
    ///     Ok(())
    /// }
    /// ```
    pub fn get_property(&mut self, property: &str) -> Result<Value, Error> {
        self._command(json!(["get_property", property]).as_array().unwrap())
    }

    /// # Description
    ///
    /// Retrieves the property value from mpv.
    /// The result is always of type String, regardless of the type of the value of the mpv property
    ///
    /// ## Input arguments
    ///
    /// - **property** defines the mpv property that should be retrieved
    ///
    /// # Example
    ///
    /// ```
    /// use mpvipc::{Mpv, Error};
    /// fn main() -> Result<(), Error> {
    ///     let mpv = Mpv::connect("/tmp/mpvsocket")?;
    ///     let title = mpv.get_property_string("media-title")?;
    ///     Ok(())
    /// }
    /// ```
    pub fn get_property_string(&mut self, property: &str) -> Result<String, Error> {
        match self.get_property(property)? {
            Value::Bool(b) => Ok(b.to_string()),
            Value::Number(ref n) => Ok(n.to_string()),
            Value::String(ref s) => Ok(s.to_string()),
            Value::Array(ref array) => Ok(format!("{:?}", array)),
            Value::Object(ref map) => Ok(format!("{:?}", map)),
            Value::Null => Err(Error::MissingValue),
        }
    }

    /// # Description
    ///
    /// Sets the mpv property _<property>_ to _<value>_.
    ///
    /// ## Supported types
    /// - String
    /// - bool
    /// - f64
    /// - usize
    ///
    /// ## Input arguments
    ///
    /// - **property** defines the mpv property that should be retrieved
    /// - **value** defines the value of the given mpv property _<property>_
    ///
    /// # Example
    /// ```
    /// use mpvipc::{Mpv, Error};
    /// fn main() -> Result<(), Error> {
    ///     let mpv = Mpv::connect("/tmp/mpvsocket")?;
    ///     mpv.set_property("pause", true)?;
    ///     Ok(())
    /// }
    /// ```
    pub fn set_property<T: Serialize>(&mut self, property: &str, value: T) -> Result<(), Error> {
        self._command(json!(["set_property", property, value]).as_array().unwrap()).map(|_| ())
    }

    pub fn add_property(&mut self, property: &str, value: f64) -> Result<(), Error> {
        self._command(json!(["add", property, value]).as_array().unwrap()).map(|_| ())
    }

    pub fn observe_property(&mut self, id: isize, property: &str) -> Result<(), Error> {
        self._command(json!(["observe_property", id, property]).as_array().unwrap()).map(|_| ())
    }

    pub fn unobserve_property(&mut self, id: isize) -> Result<(), Error> {
        self._command(json!(["unobserve_property", id]).as_array().unwrap()).map(|_| ())
    }

    /// # Description
    ///
    /// Blocks until an mpv event occurs and returns the Event.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let mut mpv = Mpv::connect("/tmp/mpvsocket")?;
    /// loop {
    ///     let event = mpv.event_listen()?;
    ///     println!("{:?}", event);
    /// }
    /// ```
    pub fn listen(&mut self) -> Result<Map<String, Value>, Error> {
        // sometimes we get responses unrelated to events, so we read a new line until we receive one
        // with an event field
        if !self.responses.is_empty() {
            return Ok(self.responses.remove(0));
        }
        loop {
            let mut response = String::new();
            let n = self.reader.read_line(&mut response)
                .map_err(|why| Error::ReadError(why.to_string()))?;
            if n == 0 {
                return Err(Error::ReadError("EOF reached".to_string()));
            }
            response = response.trim_end().to_string();
            debug!("Event: {}", response);

            let e = serde_json::from_str::<Value>(&response)
                .map_err(|why| Error::JsonParseError(why.to_string()))?;

            if let Value::Object(map) = e {
                if let Some(Value::String(_)) = map.get("event") {
                    return Ok(map);
                } else {
                    debug!("Bad response: {:?}", response);
                }
            } else {
                return Err(Error::UnexpectedValue);
            }
        }
    }

    pub fn listen_raw(&mut self) -> Result<String, Error> {
        let mut response = String::new();
        let n = self.reader.read_line(&mut response)
            .map_err(|why| Error::ReadError(why.to_string()))?;
        if n == 0 {
            return Err(Error::ReadError("EOF reached".to_string()));
        }
        Ok(response.trim_end().to_string())
    }
}
