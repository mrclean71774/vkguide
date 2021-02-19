use std::fmt;

#[derive(Debug)]
pub enum Error {
  FromVkcapi(vkcapi::Error),
  FromVkcboot(vkcboot::Error),
  FromIO(std::io::Error),
  Str(&'static str),
  String(String),
}

impl std::fmt::Display for Error {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Error::FromVkcapi(e) => fmt::Display::fmt(&e, f),
      Error::FromVkcboot(e) => fmt::Display::fmt(&e, f),
      Error::FromIO(e) => fmt::Display::fmt(&e, f),
      Error::Str(s) => fmt::Display::fmt(&s, f),
      Error::String(s) => fmt::Display::fmt(&s, f),
    }
  }
}

impl std::error::Error for Error {
  fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
    match self {
      Error::FromVkcapi(e) => Some(e),
      Error::FromVkcboot(e) => Some(e),
      Error::FromIO(e) => Some(e),
      Error::Str(_) => None,
      Error::String(_) => None,
    }
  }
}
