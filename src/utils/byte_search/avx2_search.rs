use std::borrow::Cow;

use super::BytePosition;

pub struct Avx2ByteSearchIter<'a> {
  input: Cow<'a, [u8]>,
  bitmap: u64,
  search_bytes: &'a [u8],
  read_head: usize,
  current_head: usize,
}

impl<'a> Avx2ByteSearchIter<'a> {
  pub fn new(input: &'a [u8], search_bytes: &'a [u8]) -> Self {
    let input = if input.len() < 64 {
      // WARN: padding short inputs to 64 bytes is kinda `meh`.
      let mut padded = vec![0u8; 64];

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
    let addr = self.input.as_ptr();
    let tail_len = self.input.len() & 63;
    let block_len = self.input.len() - tail_len;
    let mut offset = self.read_head;

    while offset < block_len {
      let bitmap = unsafe {
        let ptr0 = addr.byte_add(offset).cast();
        let ptr1 = addr.byte_add(offset + 32).cast();
        let block0 = std::arch::x86_64::_mm256_loadu_si256(ptr0);
        let block1 = std::arch::x86_64::_mm256_loadu_si256(ptr1);
        let mut cmp_block0 = std::arch::x86_64::_mm256_setzero_si256();
        let mut cmp_block1 = std::arch::x86_64::_mm256_setzero_si256();

        for search_byte in self.search_bytes {
          let mask = std::arch::x86_64::_mm256_set1_epi8((*search_byte) as i8);
          let cmp0 = std::arch::x86_64::_mm256_cmpeq_epi8(block0, mask);
          let cmp1 = std::arch::x86_64::_mm256_cmpeq_epi8(block1, mask);

          cmp_block0 = std::arch::x86_64::_mm256_xor_si256(cmp_block0, cmp0);
          cmp_block1 = std::arch::x86_64::_mm256_xor_si256(cmp_block1, cmp1);
        }

        let pos_l = std::arch::x86_64::_mm256_movemask_epi8(cmp_block0) as u32;
        let pos_h = std::arch::x86_64::_mm256_movemask_epi8(cmp_block1) as u32;
        ((pos_h as u64) << 32) | pos_l as u64
      };

      self.current_head = offset;
      offset += 64;
      self.read_head = offset;

      if bitmap > 0 {
        self.bitmap = bitmap;
        return;
      }
    }

    if tail_len > 0 {
      offset = self.input.len().saturating_sub(64);

      let bitmap = unsafe {
        let ptr0 = addr.byte_add(offset).cast();
        let ptr1 = addr.byte_add(offset + 32).cast();
        let block0 = std::arch::x86_64::_mm256_loadu_si256(ptr0);
        let block1 = std::arch::x86_64::_mm256_loadu_si256(ptr1);
        let mut cmp_block0 = std::arch::x86_64::_mm256_setzero_si256();
        let mut cmp_block1 = std::arch::x86_64::_mm256_setzero_si256();

        for search_byte in self.search_bytes {
          let mask = std::arch::x86_64::_mm256_set1_epi8((*search_byte) as i8);
          let cmp0 = std::arch::x86_64::_mm256_cmpeq_epi8(block0, mask);
          let cmp1 = std::arch::x86_64::_mm256_cmpeq_epi8(block1, mask);

          cmp_block0 = std::arch::x86_64::_mm256_xor_si256(cmp_block0, cmp0);
          cmp_block1 = std::arch::x86_64::_mm256_xor_si256(cmp_block1, cmp1);
        }

        let pos_l = std::arch::x86_64::_mm256_movemask_epi8(cmp_block0) as u32;
        let pos_h = std::arch::x86_64::_mm256_movemask_epi8(cmp_block1) as u32;
        ((pos_h as u64) << 32) | pos_l as u64
      };

      self.current_head = block_len;
      self.read_head += tail_len;
      self.bitmap = bitmap.wrapping_shr((64 - tail_len) as u32);
    }
  }
}

impl<'a> Iterator for Avx2ByteSearchIter<'a> {
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
      let byte_position: usize = self.current_head + (bit_pos as usize);

      self.current_head = byte_position + 1;
      self.bitmap = self.bitmap.wrapping_shr(bit_pos + 1);

      Some(BytePosition {
        index: byte_position,
        value: unsafe { *self.input.get_unchecked(byte_position) },
      })
    } else {
      None
    }
  }
}
