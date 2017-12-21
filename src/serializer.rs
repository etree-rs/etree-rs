use std::io::{Write, Result};
use tree::Element;

pub struct Serializer { }

impl Serializer {
    pub fn new() -> Self {
        Serializer {
        }
    }

    pub fn start_elem(&self, writer: &mut Write, elem: &Element) -> Result<()> {
        writer.write_fmt(format_args!("<{}", elem.tag))
    }
}

#[cfg(test)]
mod tests {
    use super::Element;
    use std::io::Cursor;

    #[test]
    fn empty_element() {
        let root = Element::new("tns:document".parse().unwrap());
        let mut w = Cursor::new(Vec::new());
        let result = root.serialize(&mut w);
        assert!(result.is_ok());

        let xml = String::from_utf8(w.into_inner()).unwrap();
        assert_eq!(
            "<tns:document",
            xml
            );

    }
}
