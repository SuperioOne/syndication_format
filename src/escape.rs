use crate::{
  error::XmlSerializeError,
  serializer::Write,
  utils::byte_search::{swar_search::SwarByteSearchIter, BasicByteSearch, BytePosition},
};

pub const XML_ESCAPE_PATTERNS: &[u8; 5] = &[b'<', b'>', b'&', b'\'', b'"'];
pub const XML_ATTR_ESCAPE_PATTERNS: &[u8; 4] = &[b'<', b'>', b'&', b'"'];

static ESCAPE_LOOKUP_TABLE: [Option<&&str>; 256] = {
  let mut table = [None; 256];
  table[b'"' as usize] = Some(&"&quot;");
  table[b'&' as usize] = Some(&"&amp;");
  table[b'<' as usize] = Some(&"&lt;");
  table[b'>' as usize] = Some(&"&gt;");
  table[b'\'' as usize] = Some(&"&apos;");
  table
};

#[cfg(feature = "std")]
#[inline]
fn internal_escape<S>(mut byte_search_iter: S, input: &str) -> std::borrow::Cow<'_, str>
where
  S: Iterator<Item = BytePosition> + Sized,
{
  if input.is_empty() {
    return std::borrow::Cow::Borrowed(input);
  }

  if let Some(BytePosition { index, value }) = byte_search_iter.next() {
    let mut escaped_input = String::with_capacity(input.len());

    if index != 0 {
      escaped_input.push_str(&input[..index]);
    }

    if let Some(escaped_char) = ESCAPE_LOOKUP_TABLE[value as usize] {
      escaped_input.push_str(escaped_char);
    }

    let mut last = index + 1;

    for BytePosition { index, value } in byte_search_iter {
      if last != index {
        escaped_input.push_str(&input[last..index]);
      }

      if let Some(escaped_char) = ESCAPE_LOOKUP_TABLE[value as usize] {
        escaped_input.push_str(escaped_char);
      }

      last = index + 1;
    }

    if last < (input.len() - 1) {
      escaped_input.push_str(&input[last..]);
    }

    std::borrow::Cow::Owned(escaped_input)
  } else {
    std::borrow::Cow::Borrowed(input)
  }
}

#[inline]
fn internal_escape_writer<W, S>(
  writer: &mut W,
  mut byte_search_iter: S,
  input: &str,
) -> Result<(), XmlSerializeError>
where
  W: Write + ?Sized,
  S: Iterator<Item = BytePosition>,
{
  if input.is_empty() {
    return Ok(());
  }

  if let Some(BytePosition { index, value }) = byte_search_iter.next() {
    if index != 0 {
      writer.write(&input[..index])?;
    }

    if let Some(escaped_char) = ESCAPE_LOOKUP_TABLE[value as usize] {
      writer.write(escaped_char)?;
    }

    let mut last = index + 1;

    for BytePosition { index, value } in byte_search_iter {
      if last != index {
        writer.write(&input[last..index])?;
      }

      if let Some(escaped_char) = ESCAPE_LOOKUP_TABLE[value as usize] {
        writer.write(escaped_char)?;
      }

      last = index + 1;
    }

    if last < (input.len() - 1) {
      writer.write(&input[last..])?;
    }

    Ok(())
  } else {
    writer.write(input)
  }
}

pub fn escape_writer<W>(
  input: &str,
  writer: &mut W,
  search_bytes: &[u8],
) -> Result<(), XmlSerializeError>
where
  W: Write + ?Sized,
{
  //
  // Generic
  //

  #[cfg(not(any(
    target_feature = "avx2",
    all(target_feature = "avx512f", feature = "avx512")
  )))]
  {
    match input.len() {
      0..=7 => {
        let iter = BasicByteSearch::new(input.as_bytes(), search_bytes);
        internal_escape_writer(writer, iter, input)
      }
      _ => {
        let iter = SwarByteSearchIter::new(input.as_bytes(), search_bytes);
        internal_escape_writer(writer, iter, input)
      }
    }
  }

  //
  // x86-64 with AVX2
  //

  #[cfg(all(
    target_arch = "x86_64",
    target_feature = "avx2",
    not(all(feature = "avx512", target_feature = "avx512f"))
  ))]
  {
    match input.len() {
      0..=7 => {
        let iter = BasicByteSearch::new(input.as_bytes(), search_bytes);
        internal_escape_writer(writer, iter, input)
      }
      8..=63 => {
        let iter = SwarByteSearchIter::new(input.as_bytes(), search_bytes);
        internal_escape_writer(writer, iter, input)
      }
      _ => {
        let iter = crate::utils::byte_search::avx2_search::Avx2ByteSearchIter::new(
          input.as_bytes(),
          search_bytes,
        );
        internal_escape_writer(writer, iter, input)
      }
    }
  }

  //
  // x86-64 with AVX512 CPU and avx512 feature is enabled
  //

  #[cfg(all(feature = "avx512", target_arch = "x86_64", target_feature = "avx512f"))]
  {
    match input.len() {
      0..=7 => {
        let iter = BasicByteSearch::new(input.as_bytes(), search_bytes);
        internal_escape_writer(writer, iter, input)
      }
      8..=63 => {
        let iter = SwarByteSearchIter::new(input.as_bytes(), search_bytes);
        internal_escape_writer(writer, iter, input)
      }
      _ => {
        let iter = crate::utils::byte_search::avx512_search::Avx512ByteSearchIter::new(
          input.as_bytes(),
          search_bytes,
        );
        internal_escape_writer(writer, iter, input)
      }
    }
  }
}

#[cfg(feature = "std")]
pub fn escape<'a>(input: &'a str, search_bytes: &'a [u8]) -> std::borrow::Cow<'a, str> {
  //
  // Generic
  //

  #[cfg(not(any(
    target_feature = "avx2",
    all(target_feature = "avx512f", feature = "avx512")
  )))]
  {
    match input.len() {
      0..=7 => {
        let iter = BasicByteSearch::new(input.as_bytes(), search_bytes);
        internal_escape(iter, input)
      }
      _ => {
        let iter = SwarByteSearchIter::new(input.as_bytes(), search_bytes);
        internal_escape(iter, input)
      }
    }
  }

  //
  // x86-64 with AVX2
  //

  #[cfg(all(
    target_arch = "x86_64",
    target_feature = "avx2",
    not(all(feature = "avx512", target_feature = "avx512f"))
  ))]
  {
    match input.len() {
      0..=7 => {
        let iter = BasicByteSearch::new(input.as_bytes(), search_bytes);
        internal_escape(iter, input)
      }
      8..=63 => {
        let iter = SwarByteSearchIter::new(input.as_bytes(), search_bytes);
        internal_escape(iter, input)
      }
      _ => {
        let iter = crate::utils::byte_search::avx2_search::Avx2ByteSearchIter::new(
          input.as_bytes(),
          search_bytes,
        );
        internal_escape(iter, input)
      }
    }
  }

  //
  // x86-64 with AVX512 CPU and avx512 feature is enabled
  //

  #[cfg(all(feature = "avx512", target_arch = "x86_64", target_feature = "avx512f"))]
  {
    match input.len() {
      0..=7 => {
        let iter = BasicByteSearch::new(input.as_bytes(), search_bytes);
        internal_escape(iter, input)
      }
      8..=63 => {
        let iter = SwarByteSearchIter::new(input.as_bytes(), search_bytes);
        internal_escape(iter, input)
      }
      _ => {
        let iter = crate::utils::byte_search::avx512_search::Avx512ByteSearchIter::new(
          input.as_bytes(),
          search_bytes,
        );
        internal_escape(iter, input)
      }
    }
  }
}

#[cfg(feature = "std")]
#[macro_export]
macro_rules! escape_xml {
  ($input:expr) => {
    $crate::escape::escape($input, $crate::escape::XML_ESCAPE_PATTERNS)
  };
}

#[cfg(feature = "std")]
#[macro_export]
macro_rules! escape_xml_attr {
  ($input:expr) => {
    $crate::escape::escape($input, $crate::escape::XML_ATTR_ESCAPE_PATTERNS)
  };
}
