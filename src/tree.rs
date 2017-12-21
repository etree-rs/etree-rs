use std::collections::HashMap;
use std::fmt;
use std::io;
use xml::name::OwnedName;
use serializer::Serializer;

pub struct Element {
    pub tag: OwnedName,
    pub text: Option<String>,
    pub tail: Option<String>,

    pub attributes: HashMap<OwnedName, String>,
    pub children: Vec<Element>,
}

impl fmt::Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try! { write!(f, "<{}", self.tag) }
        for (tag, value) in &self.attributes {
            try! { write!(f, " {}=\"{}\"", tag, value) }
        }
        write!(f, "/>")
    }
}

impl Element {
    pub fn new(tag: OwnedName) -> Element {
        Element {
            tag: tag,
            text: None,
            tail: None,
            attributes: HashMap::new(),
            children: Vec::new(),
        }
    }

    pub fn set_attribute<S>(&mut self, tag: OwnedName, value: S)
        where S: Into<String>
    {
        self.attributes.insert(tag, value.into());
    }

    pub fn serialize<'a, W: io::Write>(&'a self, writer: &'a mut W) -> io::Result<()>
        where &'a mut W: io::Write
    {
        let serializer = Serializer::new();
        serializer.start_elem(writer, &self)
    }
}

pub struct ElementTree {
    pub root: Option<Element>,
}

impl ElementTree {
    pub fn new(root: Option<Element>) -> ElementTree {
        ElementTree {
            root: root,
        }
    }
}


#[cfg(test)]
mod tests {
    use super::Element;

    #[test]
    fn it_works() {
        let mut e = Element::new("tns:domain".parse().unwrap());
        e.set_attribute("id".parse().unwrap(), "7");
        assert_eq!(
            "<tns:domain id=\"7\"/>",
            e.to_string()
            );
    }
}
