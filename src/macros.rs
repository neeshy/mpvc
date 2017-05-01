macro_rules! error(
    ($($arg:tt)*) => { {
        use ::std::io::Write;
        let r = writeln!(&mut ::std::io::stderr(), $($arg)*);
        r.expect("failed printing to stderr");
        ::std::process::exit(1);
    } }
);

macro_rules! println_stderr(
    ($($arg:tt)*) => { {
        use ::std::io::Write;
        let r = writeln!(&mut ::std::io::stderr(), $($arg)*);
        r.expect("failed printing to stderr");
    } }
);