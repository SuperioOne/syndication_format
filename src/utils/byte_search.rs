#[cfg(all(
  target_arch = "x86_64",
  target_feature = "avx2",
  not(all(feature = "avx512", target_feature = "avx512f"))
))]
pub(crate) mod avx2_search;

#[cfg(all(feature = "avx512", target_arch = "x86_64", target_feature = "avx512f",))]
mod avx512_search;
pub(crate) mod swar_search;

#[derive(Debug)]
pub struct BytePosition {
  pub index: usize,
  pub value: u8,
}

pub(crate) trait ByteSearch {
  fn search_bytes<'a>(&'a self, search_bytes: &'a [u8]) -> ByteSearchIter<'a>;
}

#[cfg(all(
  target_arch = "x86_64",
  target_feature = "avx2",
  not(all(feature = "avx512", target_feature = "avx512f"))
))]
type IteratorType<'a> = avx2_search::Avx2ByteSearchIter<'a>;

#[cfg(all(feature = "avx512", target_arch = "x86_64", target_feature = "avx512f"))]
type IteratorType<'a> = avx512_search::Avx512ByteSearchIter<'a>;

#[cfg(not(any(target_feature = "avx2", target_feature = "avx512f")))]
type IteratorType<'a> = swar_search::SwarByteSearchIter<'a>;

pub struct ByteSearchIter<'a> {
  inner: IteratorType<'a>,
}

impl<'a> ByteSearchIter<'a> {
  pub fn new(input: &'a [u8], search_bytes: &'a [u8]) -> Self {
    Self {
      inner: IteratorType::new(input, search_bytes),
    }
  }
}

impl Iterator for ByteSearchIter<'_> {
  type Item = BytePosition;

  #[inline]
  fn next(&mut self) -> Option<Self::Item> {
    self.inner.next()
  }
}

impl<T> ByteSearch for T
where
  T: AsRef<[u8]>,
{
  fn search_bytes<'a>(&'a self, search_bytes: &'a [u8]) -> ByteSearchIter<'a> {
    let input = self.as_ref();
    ByteSearchIter::new(input, search_bytes)
  }
}
