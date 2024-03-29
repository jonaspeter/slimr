use std::fmt;

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Node {
    ELEM(Elem),
    INCLUDE(Include),
}

#[non_exhaustive] //TODO remove once stable
#[derive(Default, PartialEq, Eq, Debug, Clone)]
pub struct Elem {
    pub tag: String,
    pub id: Option<String>,
    pub classes: Option<Vec<String>>,
    pub attr: Option<Vec<Attr>>,
    pub cont: Option<Cont>,
    pub children: Option<Vec<Node>>,
}

#[derive(Default, PartialEq, Eq, Debug, Clone)]
pub struct Include {
    pub path: String,
}

#[derive(Default, PartialEq, Eq, Debug, Clone)]
pub struct Attr {
    pub name: String,
    pub value: String,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Cont {
    LINE(String),
    BLOCK(Vec<String>),
}

impl fmt::Display for Attr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.value.contains('\"') {
            write!(f, r#"{name}='{val}'"#, name = self.name, val = self.value)
        } else {
            write!(f, r#"{name}="{val}""#, name = self.name, val = self.value)
        }
    }
}

impl fmt::Display for Cont {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Cont::LINE(text) => write!(f, "{}", text),
            Cont::BLOCK(texts) => {
                let mut res = String::new();
                for t in texts {
                    res.push_str(t);
                    res.push('\n');
                }
                write!(f, "{}", res)
            }
        }
    }
}

///Implement some helpers for testing
#[cfg(test)]
impl<'a> Elem {
    ///Creates an element from a tag
    pub fn from_ta(tag: &str) -> Elem {
        Elem {
            tag: String::from(tag),
            ..Elem::default()
        }
    }

    ///Creates an element from a tag and a content line
    pub fn from_ta_col(tag: &str, cont: &str) -> Elem {
        Elem {
            tag: String::from(tag),
            cont: Some(Cont::LINE(cont.to_string())),
            ..Elem::default()
        }
    }

    ///Creates an element from a tag and a content block
    pub fn from_ta_cob(tag: &str, cont: Vec<String>) -> Elem {
        Elem {
            tag: String::from(tag),
            cont: Some(Cont::BLOCK(cont)),
            ..Elem::default()
        }
    }
    ///Creates an element from a tag and child elements
    pub fn from_ta_ch(tag: &str, children: Vec<Node>) -> Elem {
        Elem {
            tag: String::from(tag),
            children: Some(children),
            ..Elem::default()
        }
    }

    ///Creates an element from a tag and classes
    pub fn from_ta_cl(tag: &str, classes: Vec<String>) -> Elem {
        Elem {
            tag: String::from(tag),
            classes: Some(classes),
            ..Elem::default()
        }
    }

    ///Creates an element from a tag and an id
    pub fn from_ta_id(tag: &str, id: &str) -> Elem {
        Elem {
            tag: String::from(tag),
            id: Some(String::from(id)),
            ..Elem::default()
        }
    }

    ///Creates an element from a tag, ids, and classes
    pub fn from_ta_id_cl(tag: &str, id: &str, classes: Vec<String>) -> Elem {
        Elem {
            tag: String::from(tag),
            classes: Some(classes),
            id: Some(String::from(id)),
            ..Elem::default()
        }
    }
    ///Creates an element from a tag, ids, classes, and content block
    pub fn from_ta_id_cl_cob(
        tag: &str,
        id: &str,
        classes: Vec<String>,
        content: Vec<String>,
    ) -> Elem {
        Elem {
            tag: String::from(tag),
            cont: Some(Cont::BLOCK(content)),
            classes: Some(classes),
            id: Some(String::from(id)),
            ..Elem::default()
        }
    }

    ///Creates an element from a tag, attr, and content block
    pub fn from_ta_at_cob(tag: &str, attributes: Vec<Attr>, content: Vec<String>) -> Elem {
        Elem {
            tag: String::from(tag),
            cont: Some(Cont::BLOCK(content)),
            attr: Some(attributes),
            ..Elem::default()
        }
    }
    ///Creates an element from a tag, and attr
    pub fn from_ta_at(tag: &str, attributes: Vec<Attr>) -> Elem {
        Elem {
            tag: String::from(tag),
            attr: Some(attributes),
            ..Elem::default()
        }
    }

    ///Creates an element from a tag, attr, and children
    pub fn from_ta_at_ch(tag: &str, attributes: Vec<Attr>, children: Vec<Node>) -> Elem {
        Elem {
            tag: String::from(tag),
            attr: Some(attributes),
            children: Some(children),
            ..Elem::default()
        }
    }
}


