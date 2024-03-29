use template_manager::TemplateManager;

use crate::parser;
use crate::parser::ast_types::{Cont, Node};
use crate::template_manager;

pub fn render_source(source: &str) -> String {
    let input = parser::ast::from_str(source);
    render_ast(input)
}

pub fn render_path(path: &str) -> String {
    let mut tm = TemplateManager::default();
    let source = tm
        .get(path)
        .expect(&format!("Template Manager couldn't find path: {}", path));
    render_source(&source)
}

fn render_ast(ast: Vec<Node>) -> String {
    let mut result = String::new();
    for node in ast {
        result.push_str(&render_node(&node));
    }
    result
}

fn render_node(node: &Node) -> String {
    let mut result = String::new();
    let mut opening = String::new();
    let mut content = String::new();
    let elem = match node {
        Node::ELEM(el) => el,
        _ => unimplemented!(),
    };

    opening.push_str(&elem.tag);

    if let Some(id) = &elem.id {
        opening.push_str(&format!(r#" id="{}""#, &id));
    }

    if let Some(classes) = &elem.classes {
        opening.push_str(&format!(r#" class="{}""#, classes.join(" ")));
    }

    if let Some(attrs) = &elem.attr {
        for a in attrs {
            opening.push_str(&format!(r#" {}"#, a));
        }
    }

    if let Some(c) = &elem.cont {
        content.push_str(&render_elem_content(c));
    }

    if let Some(children) = &elem.children {
        for child in children {
            content.push_str(&render_node(child));
        }
    }

    if content == "" {
        result.push_str(&format!("<{} />", opening))
    } else {
        result.push_str(&format!(
            "<{op}>{co}</{cl}>",
            op = opening,
            co = content,
            cl = elem.tag
        ))
    }

    result
}

fn render_elem_content(cont: &Cont) -> String {
    match cont {
        Cont::LINE(l) => l.to_string(),
        Cont::BLOCK(b) => b.join("<br>"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::ast_types::{Attr, Elem};
    use crate::{node_el_vec, string_vec};

    macro_rules! test_elems {
        ([ $($input:expr),+ ], $expected:literal ) => {
            assert_eq!(render_ast(vec![$(Node::ELEM($input)),+]), $expected);
        }
    }

    #[test]
    fn renders_simple_tag() {
        test_elems!([Elem::from_ta("hello")], "<hello />");
    }

    #[test]
    fn renders_two_tags() {
        test_elems!(
            [Elem::from_ta("hello"), Elem::from_ta("world")],
            "<hello /><world />"
        );
    }

    #[test]
    fn renders_tag_cont() {
        test_elems!(
            [Elem::from_ta_col("hello", "world")],
            "<hello>world</hello>"
        );
    }

    #[test]
    fn renders_tag_children() {
        test_elems!(
            [Elem::from_ta_ch(
                "hello",
                node_el_vec![Elem::from_ta("world")]
            )],
            "<hello><world /></hello>"
        );
    }

    #[test]
    fn renders_id() {
        test_elems!(
            [Elem::from_ta_id("hello", "world")],
            "<hello id=\"world\" />"
        );
    }

    #[test]
    fn renders_classes() {
        test_elems!(
            [Elem::from_ta_cl("hello", string_vec!["world", "universe"])],
            "<hello class=\"world universe\" />"
        );
    }

    #[test]
    fn renders_attributes() {
        test_elems!(
            [Elem::from_ta_at(
                "hello",
                vec![
                    Attr {
                        name: "world".to_string(),
                        value: "great".to_string()
                    },
                    Attr {
                        name: "sun".to_string(),
                        value: "shining".to_string()
                    }
                ]
            )],
            "<hello world=\"great\" sun=\"shining\" />"
        );
    }

    #[test]
    fn renders_single_quoted() {
        test_elems!(
            [Elem::from_ta_at(
                "img",
                vec![Attr {
                    name: "Mr".to_string(),
                    value: "Thomas \"Neo\" Anderson".to_string()
                }]
            )],
            "<img Mr='Thomas \"Neo\" Anderson' />"
        );
    }
    #[test]
    fn renders_attributes_on_children() {
        test_elems!(
            [Elem::from_ta_at_ch(
                "hello",
                vec![Attr {
                    name: "world".to_string(),
                    value: "great".to_string()
                }],
                node_el_vec![Elem::from_ta_at(
                    "how",
                    vec![Attr {
                        name: "are".to_string(),
                        value: "you?".to_string()
                    }]
                )]
            )],
            "<hello world=\"great\"><how are=\"you?\" /></hello>"
        );
    }
}
