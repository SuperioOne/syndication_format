use crate::error::XmlSerializeError;

use super::Write;

pub struct DefaultWriter<'a, W>
where
  W: std::io::Write,
{
  inner_writer: &'a mut W,
}

#[derive(Copy, Clone)]
pub enum SpaceStyle {
  Tabs,
  WhiteSpace,
}

const fn get_character_byte(style: SpaceStyle) -> u8 {
  match style {
    SpaceStyle::Tabs => b'\t',
    SpaceStyle::WhiteSpace => b' ',
  }
}

pub struct IndentedWriter<'a, W>
where
  W: std::io::Write,
{
  inner_writer: &'a mut W,
  level: usize,
  tab_spaces: usize,
  space_style: SpaceStyle,
  is_next_line: bool,
}

impl<'a, W> Write for DefaultWriter<'a, W>
where
  W: std::io::Write,
{
  #[inline]
  fn write(&mut self, data: &str) -> core::result::Result<(), XmlSerializeError> {
    self.inner_writer.write_all(data.as_bytes())?;
    Ok(())
  }

  #[inline]
  fn write_line(&mut self, data: &str) -> core::result::Result<(), XmlSerializeError> {
    self.inner_writer.write_fmt(format_args!("{}", data))?;
    Ok(())
  }

  #[inline]
  fn write_fmt(
    &mut self,
    fmt: core::fmt::Arguments,
  ) -> core::result::Result<(), XmlSerializeError> {
    self.inner_writer.write_fmt(fmt)?;
    Ok(())
  }

  #[inline]
  fn increment_level(&mut self) {}

  #[inline]
  fn decrement_level(&mut self) {}

  #[inline]
  fn set_level(&mut self, _level: usize) {}

  #[inline]
  fn get_level(&self) -> usize {
    0
  }
}

impl<'a, W> DefaultWriter<'a, W>
where
  W: std::io::Write,
{
  pub fn new(writer: &'a mut W) -> Self {
    Self {
      inner_writer: writer,
    }
  }
}

impl<'a, W> Write for IndentedWriter<'a, W>
where
  W: std::io::Write,
{
  #[inline]
  fn write(&mut self, data: &str) -> core::result::Result<(), XmlSerializeError> {
    if self.is_next_line {
      self.write_indentation()?;
    }

    self.inner_writer.write_all(data.as_bytes())?;
    self.is_next_line = false;
    Ok(())
  }

  #[inline]
  fn write_line(&mut self, data: &str) -> core::result::Result<(), XmlSerializeError> {
    if self.is_next_line {
      self.write_indentation()?;
    }

    self.inner_writer.write_fmt(format_args!("{}\n", data))?;
    self.is_next_line = true;
    Ok(())
  }

  #[inline]
  fn write_fmt(
    &mut self,
    fmt: core::fmt::Arguments,
  ) -> core::result::Result<(), XmlSerializeError> {
    if self.is_next_line {
      self.write_indentation()?;
    }

    self.inner_writer.write_fmt(fmt)?;
    self.is_next_line = false;
    Ok(())
  }

  #[inline]
  fn increment_level(&mut self) {
    self.level = self.level.saturating_add(1);
  }

  #[inline]
  fn decrement_level(&mut self) {
    self.level = self.level.saturating_sub(1);
  }

  #[inline]
  fn set_level(&mut self, level: usize) {
    self.level = level;
  }

  #[inline]
  fn get_level(&self) -> usize {
    self.level
  }
}

impl<'a, W> IndentedWriter<'a, W>
where
  W: std::io::Write,
{
  pub fn new(writer: &'a mut W, space_style: SpaceStyle, tab_spaces: usize) -> Self {
    Self {
      inner_writer: writer,
      tab_spaces,
      space_style,
      is_next_line: false,
      level: 0,
    }
  }

  #[inline]
  fn write_indentation(&mut self) -> core::result::Result<(), XmlSerializeError> {
    let space_size = self.tab_spaces.saturating_mul(self.level);
    let indent = vec![get_character_byte(self.space_style); space_size];

    self.inner_writer.write_all(&indent)?;

    Ok(())
  }
}
