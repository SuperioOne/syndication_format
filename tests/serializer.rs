use std::str::from_utf8_unchecked;

use atom_syndication_format::{
  common::{AttributeMap, AttributeName, AttributeValue},
  error::XmlSerializeError,
  serializer::{
    formatter::{DefaultWriter, IndentedWriter, SpaceStyle},
    ElementSerializer, Serialize, Serializer, XmlSerializer,
  },
};

pub struct Title {
  text: String,
}

pub struct Root {
  title: Title,
  list: List,
}

pub struct List {
  inner: Vec<ListItem>,
}

pub struct ListItem {
  value: usize,
}

impl Serialize for ListItem {
  fn serialize<S>(&self, serializer: S, namespace: Option<&str>) -> Result<(), XmlSerializeError>
  where
    S: Serializer,
  {
    let mut attr = AttributeMap::new();

    attr.set(
      AttributeName::new("value")?,
      AttributeValue::new(&self.value.to_string()),
    );

    serializer.serialize_empty_element("li", namespace, Some(&attr))?;

    Ok(())
  }
}

impl Serialize for List {
  fn serialize<S>(&self, serializer: S, namespace: Option<&str>) -> Result<(), XmlSerializeError>
  where
    S: Serializer,
  {
    let mut list = serializer.serialize_element("ul", namespace, None)?;

    for item in self.inner.iter() {
      list.serialize(item, namespace)?;
    }

    list.end()?;

    Ok(())
  }
}

impl Serialize for Title {
  fn serialize<S>(&self, serializer: S, namespace: Option<&str>) -> Result<(), XmlSerializeError>
  where
    S: Serializer,
  {
    let title = serializer.serialize_element("h1", Some("xhtml"), None)?;
    title.serialize_escaped_str(&self.text)?;

    Ok(())
  }
}

impl Serialize for Root {
  fn serialize<S>(&self, serializer: S, namespace: Option<&str>) -> Result<(), XmlSerializeError>
  where
    S: Serializer,
  {
    let child_namespace = Some("xhtml");
    let mut root = serializer.serialize_element("root", namespace, None)?;

    root.serialize(&self.title, child_namespace)?;
    root.serialize(&self.list, child_namespace)?;
    root.end()?;

    Ok(())
  }
}

#[test]
fn xml_serializer_write() {
  let expected = r#"<root><xhtml:h1>Hello world!</xhtml:h1><xhtml:ul><xhtml:li value="12"/><xhtml:li value="13"/></xhtml:ul></root>"#;
  let obj = Root {
    list: List {
      inner: vec![ListItem { value: 12 }, ListItem { value: 13 }],
    },
    title: Title {
      text: String::from("Hello world!"),
    },
  };

  let mut buffer: Vec<u8> = Vec::new();
  let mut writer = DefaultWriter::new(&mut buffer);
  let mut xml_writer = XmlSerializer::new(&mut writer);
  let _ = xml_writer.serialize(obj, None);
  let serialized_text = unsafe { from_utf8_unchecked(&buffer) };

  assert_eq!(expected, serialized_text);
}

#[test]
fn xml_serializer_formatted_write() {
  let expected = "<root>
    <xhtml:h1>
        Hello world!
    </xhtml:h1>
    <xhtml:ul>
        <xhtml:li value=\"12\"/>
        <xhtml:li value=\"13\"/>
    </xhtml:ul>
</root>
";

  let obj = Root {
    list: List {
      inner: vec![ListItem { value: 12 }, ListItem { value: 13 }],
    },
    title: Title {
      text: String::from("Hello world!"),
    },
  };

  let mut buffer: Vec<u8> = Vec::new();
  let mut writer = IndentedWriter::new(&mut buffer, SpaceStyle::WhiteSpace, 4);
  let mut xml_writer = XmlSerializer::new(&mut writer);
  let _ = xml_writer.serialize(obj, None);
  let serialized_text = unsafe { from_utf8_unchecked(&buffer) };

  assert_eq!(expected, serialized_text);
}

#[test]
fn xml_serializer_tab_formatted_write() {
  let expected = "<root>
\t<xhtml:h1>
\t\tHello world!
\t</xhtml:h1>
\t<xhtml:ul>
\t\t<xhtml:li value=\"12\"/>
\t\t<xhtml:li value=\"13\"/>
\t</xhtml:ul>
</root>
";

  let obj = Root {
    list: List {
      inner: vec![ListItem { value: 12 }, ListItem { value: 13 }],
    },
    title: Title {
      text: String::from("Hello world!"),
    },
  };

  let mut buffer: Vec<u8> = Vec::new();
  let mut writer = IndentedWriter::new(&mut buffer, SpaceStyle::Tabs, 1);
  let mut xml_writer = XmlSerializer::new(&mut writer);
  let _ = xml_writer.serialize(obj, None);
  let serialized_text = unsafe { from_utf8_unchecked(&buffer) };

  assert_eq!(expected, serialized_text);
}
