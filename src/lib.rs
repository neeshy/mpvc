extern crate log;
extern crate serde;
extern crate serde_json;

use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::io::{BufRead, BufReader, Error as IoError, ErrorKind as IoErrorKind, Write};
use std::os::unix::net::UnixStream;

use log::debug;
use serde::ser::Serialize;
use serde_json::{json, Error as JsonError, Map, Value};

pub struct Mpv {
    path: String,
    reader: BufReader<UnixStream>,
    responses: Vec<Map<String, Value>>,
    counter: i64,
}

impl Debug for Mpv {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> FmtResult {
        fmt.debug_tuple("Mpv").field(&self.path).finish()
    }
}

impl Drop for Mpv {
    fn drop(&mut self) {
        self._disconnect();
    }
}

impl Clone for Mpv {
    fn clone(&self) -> Self {
        let sock = self.reader.get_ref().try_clone().expect("cloning UnixStream");
        Mpv {
            path: self.path.clone(),
            reader: BufReader::new(sock),
            responses: self.responses.clone(),
            counter: self.counter,
        }
    }
}

pub enum Error {
    MpvError(String),
    ConnectError(IoError),
    ReadError(IoError),
    WriteError(IoError),
    JsonError(JsonError),
    UnexpectedValue,
    MissingValue,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Error::MpvError(ref e) => write!(f, "MpvError: {}", e),
            Error::ConnectError(ref e) => write!(f, "ConnectError: {}", e),
            Error::ReadError(ref e) => write!(f, "ReadError: {}", e),
            Error::WriteError(ref e) => write!(f, "WriteError: {}", e),
            Error::JsonError(ref e) => write!(f, "JsonError: {}", e),
            Error::UnexpectedValue => write!(f, "Unexpected value received"),
            Error::MissingValue => write!(f, "Missing value"),
        }
    }
}

impl Debug for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Display::fmt(self, f)
    }
}

impl Mpv {
    pub fn connect(path: &str) -> Result<Mpv, Error> {
        match UnixStream::connect(path) {
            Ok(sock) => Ok(Mpv {
                reader: BufReader::new(sock),
                path: path.to_string(),
                responses: Vec::new(),
                counter: -1,
            }),
            Err(e) => Err(Error::ConnectError(e)),
        }
    }

    fn _disconnect(&self) {
        self.reader.get_ref().shutdown(std::net::Shutdown::Both).expect("socket disconnect");
    }

    pub fn disconnect(self) {
        self._disconnect();
    }

    fn _command(&mut self, command: &Vec<Value>) -> Result<Value, Error> {
        self.counter += 1;
        let c = json!({"command": command, "request_id": self.counter}).to_string();
        debug!("Command: {}", c);
        self.reader.get_ref().write_all((c + "\n").as_bytes()).map_err(Error::WriteError)?;
        loop {
            let mut response = String::new();
            let n = self.reader.read_line(&mut response).map_err(Error::ReadError)?;
            if n == 0 {
                return Err(Error::ReadError(IoError::from(IoErrorKind::UnexpectedEof)));
            }
            response = response.trim_end().to_string();
            debug!("Response: {}", response);

            let mut r = serde_json::from_str::<Map<String, Value>>(&response)
                .map_err(Error::JsonError)?;

            if let Some(Value::Number(ref request_id)) = r.get("request_id") {
                if request_id.as_i64() != Some(self.counter) {
                    continue;
                }
            } else {
                if let Some(Value::String(_)) = r.get("event") {
                    self.responses.push(r);
                }
                continue;
            }

            let error = if let Some(Value::String(ref error)) = r.get("error") {
                Ok(error)
            } else {
                Err(Error::UnexpectedValue)
            }?;

            return if error == "success" {
                if r.contains_key("data") {
                    Ok(r["data"].take())
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
    ///     let mut mpv = Mpv::connect("/tmp/mpvsocket")?;
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
    ///     let mut mpv = Mpv::connect("/tmp/mpvsocket")?;
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
    /// ## Input arguments
    ///
    /// - **property** defines the mpv property that should be retrieved
    ///
    /// # Example
    /// ```
    /// use mpvipc::{Mpv, Error};
    /// fn main() -> Result<(), Error> {
    ///     let mut mpv = Mpv::connect("/tmp/mpvsocket")?;
    ///     let paused = mpv.get_property("pause")?.as_bool().unwrap();
    ///     let title = mpv.get_property("media-title")?.as_str().unwrap();
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
    ///     let mut mpv = Mpv::connect("/tmp/mpvsocket")?;
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
    /// ## Input arguments
    ///
    /// - **property** defines the mpv property that should be retrieved
    /// - **value** defines the value of the given mpv property _<property>_
    ///
    /// # Example
    /// ```
    /// use mpvipc::{Mpv, Error};
    /// fn main() -> Result<(), Error> {
    ///     let mut mpv = Mpv::connect("/tmp/mpvsocket")?;
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
    /// Blocks until an mpv event occurs and returns the event.
    ///
    /// # Example
    ///
    /// ```
    /// use mpvipc::{Mpv, Error};
    /// fn main() -> Result<(), Error> {
    ///     let mut mpv = Mpv::connect("/tmp/mpvsocket")?;
    ///     loop {
    ///         let event = mpv.listen()?;
    ///         println!("{:?}", event);
    ///     }
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
            let n = self.reader.read_line(&mut response).map_err(Error::ReadError)?;
            if n == 0 {
                return Err(Error::ReadError(IoError::from(IoErrorKind::UnexpectedEof)));
            }
            response = response.trim_end().to_string();
            debug!("Event: {}", response);

            let e = serde_json::from_str::<Map<String, Value>>(&response)
                .map_err(Error::JsonError)?;

            if let Some(Value::String(_)) = e.get("event") {
                return Ok(e);
            } else {
                debug!("Bad response: {:?}", response);
            }
        }
    }

    pub fn listen_raw(&mut self) -> Result<String, Error> {
        let mut response = String::new();
        let n = self.reader.read_line(&mut response).map_err(Error::ReadError)?;
        if n == 0 {
            return Err(Error::ReadError(IoError::from(IoErrorKind::UnexpectedEof)));
        }
        Ok(response.trim_end().to_string())
    }
}
