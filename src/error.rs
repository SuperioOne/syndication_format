#[derive(Debug)]
pub enum XmlSerializeError {
  IOError { inner: Box<std::io::Error> },
  InvalidAttributeName,
  InvalidNamespace,
}

impl From<std::io::Error> for XmlSerializeError {
  fn from(value: std::io::Error) -> Self {
    Self::IOError {
      inner: Box::new(value),
    }
  }
}

#[derive(Debug)]
pub struct InvalidAttributeName;

impl From<InvalidAttributeName> for XmlSerializeError {
  fn from(_: InvalidAttributeName) -> Self {
    Self::InvalidAttributeName
  }
}

#[derive(Debug)]
pub enum TimeStampFormatErrors {
  InvalidFormat,
  UnsupportedYear,
  NonExistingDate,
  InvalidTime,
  InvalidDate,
  InvalidTimeOffset,
}
