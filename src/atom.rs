use crate::common::AttributeMap;
use crate::error::XmlSerializeError;
use crate::serializer::{ElementSerializer, Serialize};

mod entry;
mod feed;
mod metadata;

pub use entry::*;
pub use feed::*;
pub use metadata::*;

/// Generic helper node to simplify some property serializations
pub(crate) struct ElementNode<'a> {
  attributes: Option<&'a AttributeMap>,
  name: &'a str,
  value: Option<&'a str>,
}

impl Serialize for ElementNode<'_> {
  fn serialize<S>(
    &self,
    serializer: S,
    namespace: Option<&str>,
  ) -> core::result::Result<(), XmlSerializeError>
  where
    S: crate::serializer::Serializer,
  {
    match self.value {
      Some(value) => {
        let element = serializer.serialize_element(self.name, namespace, self.attributes)?;
        element.serialize_escaped_str(&value)?;
      }
      None => serializer.serialize_empty_element(self.name, namespace, self.attributes)?,
    }

    Ok(())
  }
}
