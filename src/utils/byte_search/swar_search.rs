use std::borrow::Cow;

use super::BytePosition;

const SWAR_MASK_L: u64 = 0x7f7f7f7f7f7f7f7f;
const SWAR_MASK_H: u64 = 0x8080808080808080;
const SWAR_ADD: u64 = 0x0101010101010101;

macro_rules! read {
  ($ptr:expr, $offset:expr) => {
    unsafe { $ptr.byte_add($offset).read() }
  };

  ($ptr:expr) => {
    unsafe { $ptr.read() }
  };
}

pub struct SwarByteSearchIter<'a> {
  input: Cow<'a, [u8]>,
  bitmap: u64,
  search_bytes: &'a [u8],
  read_head: usize,
  current_head: usize,
}

impl<'a> SwarByteSearchIter<'a> {
  pub fn new(input: &'a [u8], search_bytes: &'a [u8]) -> Self {
    let input = if input.len() < 8 {
      let mut padded = vec![0u8; 8];

      unsafe {
        input
          .as_ptr()
          .copy_to_nonoverlapping(padded.as_mut_ptr(), input.len());
      }

      Cow::Owned(padded)
    } else {
      Cow::Borrowed(input)
    };

    Self {
      current_head: 0,
      read_head: 0,
      bitmap: 0,
      input,
      search_bytes,
    }
  }

  #[inline]
  fn search_blocks(&mut self) {
    let block_ptr: *const u64 = self.input.as_ptr().cast();
    let tail_len = self.input.len() & 7;
    let block_len = self.input.len() - tail_len;
    let mut offset = self.read_head;

    while offset < block_len {
      let block = read!(block_ptr, offset);
      let mut cmp_result: u64 = 0;

      for byte in self.search_bytes {
        let search = u64::from_ne_bytes([*byte; 8]);
        let eq = block ^ search;
        cmp_result |= (!eq & SWAR_MASK_L).wrapping_add(SWAR_ADD) & (!eq & SWAR_MASK_H);
      }

      self.current_head = offset;
      offset += 8;
      self.read_head = offset;

      if cmp_result > 0 {
        self.bitmap = cmp_result;
        return;
      }
    }

    if tail_len > 0 {
      let last_block = read!(block_ptr, self.input.len() - 8);

      let mut cmp_result: u64 = 0;

      for byte in self.search_bytes {
        let search = u64::from_ne_bytes([*byte; 8]);
        let eq = last_block ^ search;
        cmp_result |= (!eq & SWAR_MASK_L).wrapping_add(SWAR_ADD) & (!eq & SWAR_MASK_H);
      }

      self.bitmap = cmp_result.wrapping_shr((64 - (tail_len * 8)) as u32);
      self.current_head = self.input.len() - tail_len;
      self.read_head += self.input.len();
    }
  }
}

impl<'a> Iterator for SwarByteSearchIter<'a> {
  type Item = BytePosition;

  #[inline]
  fn next(&mut self) -> Option<Self::Item> {
    let len = self.input.len();

    if self.current_head >= len {
      return None;
    }

    if self.bitmap == 0 && self.read_head < len {
      self.search_blocks();
    }

    if self.bitmap > 0 {
      let bit_pos = self.bitmap.trailing_zeros();
      let byte_pos: usize = self.current_head + (bit_pos / 8) as usize;

      self.current_head = byte_pos + 1;
      self.bitmap = self.bitmap.wrapping_shr(bit_pos).wrapping_shr(1);

      Some(BytePosition {
        index: byte_pos,
        value: unsafe { *self.input.get_unchecked(byte_pos) },
      })
    } else {
      None
    }
  }
}
