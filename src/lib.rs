use std::error::Error as StdError;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::io::{BufRead, BufReader, Error as IoError, ErrorKind as IoErrorKind, Write};
use std::iter::once;
use std::os::unix::net::UnixStream;
use std::path::Path;

use log::debug;
use serde_json::{Error as JsonError, Map, Number, Value};

pub struct Mpv {
    reader: BufReader<UnixStream>,
    responses: Vec<Map<String, Value>>,
    counter: i64,
}

impl Debug for Mpv {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> FmtResult {
        let mut builder = fmt.debug_struct("Mpv");
        if let Ok(addr) = self.reader.get_ref().peer_addr() {
            if let Some(pathname) = addr.as_pathname() {
                builder.field("path", &pathname);
            }
        }
        builder.finish()
    }
}

impl Drop for Mpv {
    fn drop(&mut self) {
        self._disconnect();
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

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Error::MpvError(_) => None,
            Error::ConnectError(ref e) => Some(e),
            Error::ReadError(ref e) => Some(e),
            Error::WriteError(ref e) => Some(e),
            Error::JsonError(ref e) => Some(e),
            Error::UnexpectedValue => None,
            Error::MissingValue => None,
        }
    }
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
    /// Connect to the mpv socket located at the given path.
    pub fn connect<P: AsRef<Path>>(path: P) -> Result<Mpv, Error> {
        match UnixStream::connect(path) {
            Ok(sock) => Ok(Mpv {
                reader: BufReader::new(sock),
                responses: Vec::new(),
                counter: -1,
            }),
            Err(e) => Err(Error::ConnectError(e)),
        }
    }

    fn _disconnect(&self) {
        self.reader.get_ref().shutdown(std::net::Shutdown::Both).expect("socket disconnect");
    }

    /// Close the mpv socket.
    pub fn disconnect(self) {
        self._disconnect();
    }

    fn _command<I: Iterator<Item = Value>>(&mut self, command: I) -> Result<Value, Error> {
        self.counter += 1;
        let c = Value::Object({
            let mut m = Map::with_capacity(2);
            m.insert("command".to_owned(), command.collect());
            m.insert("request_id".to_owned(), self.counter.into());
            m
        }).to_string();
        debug!("Command: {}", c);
        self.reader.get_ref().write_all((c + "\n").as_bytes()).map_err(Error::WriteError)?;
        loop {
            let mut response = String::new();
            let n = self.reader.read_line(&mut response).map_err(Error::ReadError)?;
            if n == 0 {
                return Err(Error::ReadError(IoError::from(IoErrorKind::UnexpectedEof)));
            }
            response = response.trim_end().to_owned();
            debug!("Response: {}", response);

            let r = response.parse::<Value>().map_err(Error::JsonError)?;

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
                Err(Error::MpvError(error.to_owned()))
            };
        }
    }

    /// Run an mpv command. The arguments are passed as a collection whose elements can be
    /// converted to JSON values.
    ///
    /// ```
    /// let mut mpv = Mpv::connect("/tmp/mpvsocket")?;
    /// mpv.command_arg("seek", ["0", "absolute"])?;
    /// ```
    pub fn command_arg<I: IntoIterator>(&mut self, command: &str, args: I) -> Result<(), Error>
    where I::Item: Into<Value> {
        // XXX: Drop return value for now, change interface later if needed
        self._command(once(command.into()).chain(args.into_iter().map(|v| v.into()))).map(|_| ())
    }

    /// Run an mpv command without any arguments.
    ///
    /// ```
    /// let mut mpv = Mpv::connect("/tmp/mpvsocket")?;
    /// mpv.command("playlist-shuffle")?;
    /// ```
    pub fn command(&mut self, command: &str) -> Result<(), Error> {
        self._command([command.into()].into_iter()).map(|_| ())
    }

    /// Retrieve a property from mpv.
    ///
    /// ```
    /// let mut mpv = Mpv::connect("/tmp/mpvsocket")?;
    /// let paused = mpv.get_property("pause")?.as_bool().ok_or(Error::UnexpectedValue)?;
    /// ```
    pub fn get_property(&mut self, property: &str) -> Result<Value, Error> {
        self._command(["get_property".into(), property.into()].into_iter())
    }

    /// Set an mpv property to the given value.
    ///
    /// ```
    /// let mut mpv = Mpv::connect("/tmp/mpvsocket")?;
    /// mpv.set_property("pause", true)?;
    /// ```
    pub fn set_property<T: Into<Value>>(&mut self, property: &str, value: T) -> Result<(), Error> {
        self._command(["set_property".into(), property.into(), value.into()].into_iter()).map(|_| ())
    }

    /// Add the given value to an mpv property. Runs the 'add' mpv command.
    ///
    /// ```
    /// let mut mpv = Mpv::connect("/tmp/mpvsocket")?;
    /// mpv.add_property("volume", 20.0)?;
    /// ```
    pub fn add_property(&mut self, property: &str, value: f64) -> Result<(), Error> {
        self._command(["add".into(), property.into(),
            Number::from_f64(value).ok_or(Error::UnexpectedValue)?.into()].into_iter()).map(|_| ())
    }

    /// Multiply an mpv property by the given value. Runs the 'multiply' mpv command.
    ///
    /// ```
    /// let mut mpv = Mpv::connect("/tmp/mpvsocket")?;
    /// mpv.multiply_property("speed", 2.0)?;
    /// ```
    pub fn multiply_property(&mut self, property: &str, value: f64) -> Result<(), Error> {
        self._command(["multiply".into(), property.into(),
            Number::from_f64(value).ok_or(Error::UnexpectedValue)?.into()].into_iter()).map(|_| ())
    }

    /// Watch a property for changes. Runs the 'observe_property' mpv command.
    pub fn observe_property(&mut self, id: isize, property: &str) -> Result<(), Error> {
        self._command(["observe_property".into(), id.into(), property.into()].into_iter()).map(|_| ())
    }

    /// Undo the corresponding 'observe_property'. Runs the 'unobserve_property' mpv command.
    pub fn unobserve_property(&mut self, id: isize) -> Result<(), Error> {
        self._command(["unobserve_property".into(), id.into()].into_iter()).map(|_| ())
    }

    /// Block until an mpv event occurs and return the event.
    ///
    /// ```
    /// let mut mpv = Mpv::connect("/tmp/mpvsocket")?;
    /// loop {
    ///     let event = mpv.listen()?;
    ///     println!("{:?}", event);
    /// }
    /// ```
    pub fn listen(&mut self) -> Result<Map<String, Value>, Error> {
        if !self.responses.is_empty() {
            return Ok(self.responses.remove(0));
        }
        loop {
            let response = self.listen_raw()?;
            debug!("Event: {}", response);

            let e = response.parse::<Value>().map_err(Error::JsonError)?;

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

    /// Block until an mpv event occurs and return the event as a string.
    ///
    /// ```
    /// let mut mpv = Mpv::connect("/tmp/mpvsocket")?;
    /// while let Ok(response) = mpv.listen_raw() {
    ///     println!("{}", response);
    /// }
    /// ```
    pub fn listen_raw(&mut self) -> Result<String, Error> {
        let mut response = String::new();
        let n = self.reader.read_line(&mut response).map_err(Error::ReadError)?;
        if n == 0 {
            return Err(Error::ReadError(IoError::from(IoErrorKind::UnexpectedEof)));
        }
        Ok(response.trim_end().to_owned())
    }
}
