macro_rules! println_stderr {
  ($fmt:expr) => { { writeln!(std::io::stderr(), $fmt).expect("error writing to stderr"); } };
  ($fmt:expr, $($arg:tt)*) => { { writeln!(std::io::stderr(), $fmt, $($arg)*).expect("error writing to stderr"); } };
}

macro_rules! try_or_return {
  ($expr: expr, $ret: expr) => {
    match $expr { Ok(x) => x, Err(e) => { for err in e.iter() { println_stderr!("{}", err); } return $ret; } }
  };
}

macro_rules! try_or_err_code {
  ($expr: expr) => {
    try_or_return!($expr, 1)
  }
}
