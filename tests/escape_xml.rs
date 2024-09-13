use atom_syndication_format::escape::{escape_writer, XML_ESCAPE_PATTERNS};
use atom_syndication_format::escape_xml;
use atom_syndication_format::serializer::formatter::DefaultWriter;

const INPUT_HTML: &'static str = "<div>'\"COOL&CREATE\"'</div>";
const INPUT_HTML_ESCAPED: &'static str =
  "&lt;div&gt;&apos;&quot;COOL&amp;CREATE&quot;&apos;&lt;/div&gt;";
const INPUT_PLAIN: &'static str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit.
Donec efficitur enim nisi, vel fringilla velit dapibus ac. Donec sit amet lobortis mi, vitae mattis ligula.
In sit amet erat purus. Fusce mauris lacus, cursus ornare mauris nec, hendrerit mollis dui.
Integer vestibulum lacinia tellus vitae pretium. Phasellus malesuada dictum ex, ut dapibus nisi auctor ut.
Vivamus tincidunt, nunc vel consequat viverra, arcu lacus tristique nisl, eget posuere ipsum felis quis nibh.
Phasellus magna sapien, ullamcorper non sem ac, dignissim ornare lacus. Integer egestas id felis sed tristique.
Vivamus vel neque diam. Quisque vel pulvinar risus.";

#[test]
fn escape_html() {
  let escaped = escape_xml!(&INPUT_HTML);

  match escaped {
    std::borrow::Cow::Borrowed(_) => {
      assert!(false, "It shouldn't return borrowed text back.")
    }
    std::borrow::Cow::Owned(escaped_text) => assert_eq!(INPUT_HTML_ESCAPED, &escaped_text),
  }
}

// Tests potential edge cases
#[test]
fn escape_edge_cases() {
  let start_input = "&Test";
  let escaped = escape_xml!(&start_input);

  match escaped {
    std::borrow::Cow::Borrowed(_) => {
      assert!(false, "It shouldn't return borrowed text back.")
    }
    std::borrow::Cow::Owned(escaped_text) => assert_eq!("&amp;Test", &escaped_text),
  }

  let end_input = "Test&";
  let escaped = escape_xml!(&end_input);
  match escaped {
    std::borrow::Cow::Borrowed(_) => {
      assert!(false, "It shouldn't return borrowed text back.")
    }
    std::borrow::Cow::Owned(escaped_text) => assert_eq!("Test&amp;", &escaped_text),
  }
}

#[test]
fn escape_text() {
  let escaped = escape_xml!(&INPUT_PLAIN);

  match escaped {
    std::borrow::Cow::Borrowed(escaped_text) => {
      assert_eq!(INPUT_PLAIN, escaped_text)
    }
    std::borrow::Cow::Owned(_) => assert!(false, "It shouldn't allocate new string."),
  }
}

#[test]
fn escape_writer_html() {
  let mut mem_buffer: Vec<u8> = Vec::new();
  let mut writer = DefaultWriter::new(&mut mem_buffer);

  match escape_writer(INPUT_HTML, &mut writer, XML_ESCAPE_PATTERNS) {
    Ok(()) => {
      assert_eq!(&mem_buffer, INPUT_HTML_ESCAPED.as_bytes())
    }
    Err(_) => assert!(false, "Writer failed."),
  }
}

// Tests potential edge cases
#[test]
fn escape_writer_edge_cases() {
  let start_input = "&Test";
  let mut mem_buffer: Vec<u8> = Vec::new();
  let mut writer = DefaultWriter::new(&mut mem_buffer);

  match escape_writer(start_input, &mut writer, XML_ESCAPE_PATTERNS) {
    Ok(()) => {
      assert_eq!(&mem_buffer, "&amp;Test".as_bytes())
    }
    Err(_) => assert!(false, "Writer failed."),
  }

  let end_input = "Test&";
  let mut mem_buffer: Vec<u8> = Vec::new();
  let mut writer = DefaultWriter::new(&mut mem_buffer);

  match escape_writer(end_input, &mut writer, XML_ESCAPE_PATTERNS) {
    Ok(()) => {
      assert_eq!(&mem_buffer, "Test&amp;".as_bytes())
    }
    Err(_) => assert!(false, "Writer failed."),
  }
}

#[test]
fn escape_writer_text() {
  let mut mem_buffer: Vec<u8> = Vec::new();
  let mut writer = DefaultWriter::new(&mut mem_buffer);

  match escape_writer(INPUT_PLAIN, &mut writer, XML_ESCAPE_PATTERNS) {
    Ok(()) => {
      assert_eq!(&mem_buffer, INPUT_PLAIN.as_bytes())
    }
    Err(_) => assert!(false, "Writer failed."),
  }
}
