use std::io::{IoSlice, Write};

const LINE_FEED: &'static [u8; 1] = &[b'\n'];

pub struct IndentedWriter<'a, T>
where
  T: Write + ?Sized + 'a,
{
  inner_writer: &'a mut T,
  level: usize,
  tab_spaces: usize,
}

pub enum IndentedWriterErrors {
  WriteFailed,
}

impl<'a, T> IndentedWriter<'a, T>
where
  T: Write + ?Sized + 'a,
{
  pub fn new(writer: &'a mut T) -> Self {
    Self::new_with_tab_spaces(writer, 4)
  }

  pub fn new_with_tab_spaces(writer: &'a mut T, tab_spaces: usize) -> Self {
    Self {
      level: 0,
      inner_writer: writer,
      tab_spaces,
    }
  }

  #[inline]
  pub fn set_level(&mut self, level: usize) {
    self.level = level;
  }

  #[inline]
  pub fn increment_level_by(&mut self, level: usize) {
    self.level = self.level.saturating_add(level);
  }

  #[inline]
  pub fn decrement_level_by(&mut self, level: usize) {
    self.level = self.level.saturating_sub(level);
  }

  #[inline]
  pub fn increment_level(&mut self) {
    self.increment_level_by(1);
  }

  #[inline]
  pub fn decrement_level(&mut self) {
    self.decrement_level_by(1);
  }

  #[inline]
  pub fn write_line(&mut self, line: &str) -> Result<(), std::io::Error> {
    let len = line.len();
    let bytes = line.as_bytes();

    if len > 0 {
      let padding = " ".repeat(self.tab_spaces * self.level);

      let padding_slice = IoSlice::new(padding.as_bytes());
      let line_slice = IoSlice::new(bytes);
      let lf_slice = IoSlice::new(LINE_FEED);

      self
        .inner_writer
        .write_vectored(&[padding_slice, line_slice, lf_slice])?;
    } else {
      self.inner_writer.write_all(LINE_FEED)?;
    }

    Ok(())
  }
}

mod test {
  use super::IndentedWriter;
  use std::str::from_utf8_unchecked;

  #[test]
  fn indented_writer() {
    let mut buffer: Vec<u8> = Vec::new();
    let mut writer = IndentedWriter::new_with_tab_spaces(&mut buffer, 2);

    writer.write_line("test:").unwrap();
    writer.increment_level();
    writer.write_line("name: zawarudo").unwrap();
    writer.write_line("level: 12").unwrap();
    writer.write_line("hello:").unwrap();
    writer.increment_level();
    writer.write_line("megustaw: 'lol'").unwrap();
    writer.decrement_level();
    writer.write_line("done: true").unwrap();
    writer.decrement_level();
    writer.write_line("completed: true").unwrap();

    println!("{}", unsafe { from_utf8_unchecked(&buffer) });
    assert!(true);
  }
}
