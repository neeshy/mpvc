macro_rules! error(
    ($($arg:tt)*) => { {
        use ::std::io::Write;
        writeln!(&mut ::std::io::stderr(), $($arg)*).expect("failed printing to stderr");
        ::std::process::exit(1);
    } }
);
