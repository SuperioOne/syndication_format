use core::fmt::Display;

mod attributes;
mod timestamp;

pub use attributes::*;
pub use timestamp::*;

pub enum XmlText<'a> {
  /// Raw HTML type. Serializer will set element's type attribute to "html" and escapes any special characters.
  ///
  /// Input:
  /// ```
  /// use atom_syndication_format::common::XmlText;
  /// let example_input = XmlText::Html("<div> Example </div>");
  /// ```
  ///
  /// Serialized:
  /// ```xml
  /// <example-input type="html"> &gt;div&lt; Example &lt;/div&gt;</example-input>;
  /// ```
  Html(&'a str),

  /// XHTML text type. Serializer will set elemet's type attribute to "xhtml" and embeds data as is.
  ///
  /// Input:
  /// ```
  /// use atom_syndication_format::common::XmlText;
  /// let example_input = XmlText::Xhtml("<xhtml:div> Example </xhtml:div>");
  /// ```
  ///
  /// Serialized:
  /// ```xml
  /// <example-input type="xhtml"><xhtml:div> Example </xhtml:div></example-input>;
  /// ```
  Xhtml(&'a str),

  /// Plain text type. Serializer will set element's type attribute to "text" and escapes any special characters.
  ///
  /// Input:
  /// ```
  /// use atom_syndication_format::common::XmlText;
  /// let example_input = XmlText::PlainText("COOL&CREATE");
  /// ```
  ///
  /// Serialized:
  /// ```xml
  /// <example-input type="html">COOL&amp;CREATE</example-input>;
  /// ```
  PlainText(&'a str),

  /// Html text without escape check. Use this type when the data is already escaped.
  ///
  /// Input:
  /// ```
  /// use atom_syndication_format::common::XmlText;
  /// let example_input = XmlText::HtmlUnchecked("&lt;b&gt;COOL&amp;CREATE&lt;/b&gt;");
  /// ```
  ///
  /// Serialized:
  /// ```xml
  /// <example-input type="html">&lt;b&gt;COOL&amp;CREATE&lt;/b&gt;</example-input>;
  /// ```
  HtmlUnchecked(&'a str),

  /// Text without escape check. Use this type when the data is already escaped.
  ///
  /// Input:
  /// ```
  /// use atom_syndication_format::common::XmlText;
  /// let example_input = XmlText::PlainTextUnchecked("COOL&amp;CREATE");
  /// ```
  ///
  /// Serialized:
  /// ```xml
  /// <example-input type="text">COOL&amp;CREATE</example-input>;
  /// ```
  PlainTextUnchecked(&'a str),
}

#[derive(Copy, Clone, Debug)]
pub enum LinkRelation {
  Alternate,
  Related,
  SelfRel,
  Enclosure,
  Via,
}

impl Display for LinkRelation {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    f.write_str(self.as_str())
  }
}

impl LinkRelation {
  pub fn as_str(&self) -> &'static str {
    match self {
      LinkRelation::Alternate => "alternate",
      LinkRelation::Related => "related",
      LinkRelation::SelfRel => "self",
      LinkRelation::Enclosure => "enclosure",
      LinkRelation::Via => "via",
    }
  }
}

#[macro_export]
macro_rules! html {
  ($input:expr) => {
    $crate::common::XmlText::Html(&$input)
  };
}

#[macro_export]
macro_rules! xhtml {
  ($input:expr) => {
    $crate::common::XmlText::Xhtml(&$input)
  };
}

#[macro_export]
macro_rules! text {
  ($input:expr) => {
    $crate::common::XmlText::PlainText(&$input)
  };
}

#[macro_export]
macro_rules! text_unchecked {
  ($input:expr) => {
    $crate::common::XmlText::PlainTextUnchecked(&$input)
  };
}

#[macro_export]
macro_rules! html_unchecked {
  ($input:expr) => {
    $crate::common::XmlText::HtmlUnchecked(&$input)
  };
}
