#[cfg(all(
  target_arch = "x86_64",
  target_feature = "avx2",
  not(all(feature = "avx512", target_feature = "avx512f"))
))]
pub(crate) mod avx2_search;

#[cfg(all(feature = "avx512", target_arch = "x86_64", target_feature = "avx512f",))]
pub(crate) mod avx512_search;
pub(crate) mod swar_search;

#[derive(Debug)]
pub(crate) struct BytePosition {
  pub index: usize,
  pub value: u8,
}

pub(crate) struct BasicByteSearch<'a> {
  input: &'a [u8],
  search_bytes: &'a [u8],
  current_head: usize,
}

impl<'a> BasicByteSearch<'a> {
  pub fn new(input: &'a [u8], search_bytes: &'a [u8]) -> Self {
    Self {
      input,
      current_head: 0,
      search_bytes,
    }
  }
}

impl Iterator for BasicByteSearch<'_> {
  type Item = BytePosition;

  fn next(&mut self) -> Option<Self::Item> {
    for byte in &self.input[self.current_head..] {
      for search in self.search_bytes {
        if search == byte {
          let position = BytePosition {
            index: self.current_head,
            value: *byte,
          };

          self.current_head += 1;
          return Some(position);
        }
      }

      self.current_head += 1;
    }

    None
  }
}
