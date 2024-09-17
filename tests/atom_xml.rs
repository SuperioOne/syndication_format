use std::str::from_utf8_unchecked;

use atom_syndication_format::atom::{
  Author, Category, Content, ContentValue, Contributor, Entry, Feed, Icon, Link, Logo, Rights,
  SubTitle, Summary,
};
use atom_syndication_format::common::{AttributeName, DateTime, XmlText};
use atom_syndication_format::serializer::formatter::{IndentedWriter, SpaceStyle};
use atom_syndication_format::serializer::{Serializer, XmlSerializer};
use atom_syndication_format::{html, text};

struct TestEntry {
  content: String,
  id: String,
  summary: String,
  title: String,
}

fn generate_entries() -> Vec<TestEntry> {
  let entries: Vec<TestEntry> = vec![
    TestEntry {
      content: "<div> Test content for 0 </div>".to_owned(),
      id: "entry-0000".to_owned(),
      summary: "Summary for 0".to_owned(),
      title: "Entry 0".to_owned(),
    },
    TestEntry {
      content: "<div> Test content for 1 </div>".to_owned(),
      id: "entry-0001".to_owned(),
      summary: "Summary for 1".to_owned(),
      title: "Entry 1".to_owned(),
    },
    TestEntry {
      content: "<div> Test content for 2 </div>".to_owned(),
      id: "entry-0002".to_owned(),
      summary: "Summary for 2".to_owned(),
      title: "Entry 2".to_owned(),
    },
    TestEntry {
      content: "<div> Test content for 3 </div>".to_owned(),
      id: "entry-0003".to_owned(),
      summary: "Summary for 3".to_owned(),
      title: "Entry 3".to_owned(),
    },
  ];

  entries
}

#[test]
fn simple_atom_feed() {
  let entries = generate_entries();
  let mut feed = Feed::new(
    "00abcd",
    XmlText::PlainText("Hello & World"),
    DateTime::default(),
  );

  let mut author = Author::new("SuperiorOne");
  author.email = Some("pm@smdd.dev");
  author.uri = Some("https://smdd.dev");

  feed
    .title
    .set_attribute(AttributeName::new("xml:lang").unwrap(), "en-US".into());

  let mut subtitle = SubTitle::new(html!("<i>Test</i>"));
  subtitle.set_attribute(AttributeName::new("xml:lang").unwrap(), "en-US".into());

  feed.authors.push(author);
  feed.contributors.push(Contributor::new("SuperiorTwo"));
  feed.categories.push(Category::new("Technology"));
  feed.categories.push(Category::new("Atom & RSS"));
  feed.categories.push(Category::new("Syndication Formats"));
  feed.links.push(Link::new("https://smdd.dev"));
  feed.subtitle = Some(subtitle);
  feed.rights = Some(Rights::new(text!("Copyright Nobody")));
  feed.icon = Some(Icon::new("https://fake-address.nope/icon.jpg"));
  feed.logo = Some(Logo::new("https://fake-address.nope/logo.jpg"));

  for e in entries.iter() {
    let mut entry = Entry::new(&e.id, text!(&e.title), DateTime::default());
    let mut content = Content::new(html!(&e.content).into());

    content.set_attribute(AttributeName::new("xml:lang").unwrap(), "en-US".into());
    content.set_attribute(
      AttributeName::new("custom-attr").unwrap(),
      format!("id-{}", &e.id).into(),
    );

    entry.content = Some(content);
    entry.summary = Some(Summary::new(text!(&e.summary)));

    feed.entries.push(entry);
  }

  let mut bytes: Vec<u8> = Vec::new();
  let mut writer = IndentedWriter::new(&mut bytes, SpaceStyle::WhiteSpace, 2);
  let mut xml_serializer = XmlSerializer::new(&mut writer);

  xml_serializer.serialize(feed, Some("atom")).unwrap();

  println!("{}", unsafe { from_utf8_unchecked(&bytes) });

  assert!(true)
}

#[test]
fn atom_feed_invalid_namespace() {
  let feed = Feed::new("uuid", text!("test"), DateTime::default());
  let mut bytes: Vec<u8> = Vec::new();
  let mut writer = IndentedWriter::new(&mut bytes, SpaceStyle::WhiteSpace, 2);
  let mut xml_serializer = XmlSerializer::new(&mut writer);

  match xml_serializer.serialize(feed, Some("-123-invalid:namespace")) {
    Ok(_) => assert!(
      false,
      "Feed serializer should have been failed due to invalid namespace"
    ),
    Err(atom_syndication_format::error::XmlSerializeError::InvalidNamespace) => assert!(true),
    Err(_) => assert!(false, "Unexpected error type"),
  }
}
