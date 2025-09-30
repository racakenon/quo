use crate::html::node::{ElementType, IRNode};
use crate::html::trust::{Content, HtmlBlock, SafeString};

pub trait Renderer: Clone {
    type Output;
    fn visit_node_begin(&self, node: &IRNode) -> Self;
    fn visit_node_end(&self, node: &IRNode) -> Self;
    fn visit_text(&self, content: &Content) -> Self;
    fn visit_raw(&self, html: &HtmlBlock) -> Self;
    fn finalize(&self) -> &Self::Output;
}

#[derive(Clone)]
pub struct HtmlRenderer {
    buffer: HtmlBlock,
}
impl HtmlRenderer {
    pub fn new() -> Self {
        HtmlRenderer {
            buffer: HtmlBlock::from_str(&String::new()),
        }
    }
}

impl Renderer for HtmlRenderer {
    type Output = HtmlBlock;

    fn visit_node_begin(&self, node: &IRNode) -> Self {
        let mut buffer = self.buffer.as_str().to_string();

        buffer.push('<');
        buffer.push_str(&node.get_tag().as_str());
        buffer.push_str(&node.get_attrs().into_string());

        match node.get_type() {
            ElementType::Void => {
                buffer.push_str(" >");
            }
            ElementType::Normal => {
                buffer.push('>');
            }
        }

        HtmlRenderer {
            buffer: HtmlBlock::from_str(&buffer),
        }
    }

    fn visit_node_end(&self, node: &IRNode) -> Self {
        let mut buffer = self.buffer.as_str().to_string();

        match node.get_type() {
            ElementType::Normal => {
                buffer.push_str("</");
                buffer.push_str(&node.get_tag().as_str());
                buffer.push('>');
            }
            ElementType::Void => {
            }
        }

        HtmlRenderer {
            buffer: HtmlBlock::from_str(&buffer),
        }
    }

    fn visit_text(&self, content: &Content) -> Self {
        let mut buffer = self.buffer.as_str().to_string();
        buffer.push_str(&content.as_str());

        HtmlRenderer {
            buffer: HtmlBlock::from_str(&buffer),
        }
    }

    fn visit_raw(&self, html: &HtmlBlock) -> Self {
        let mut buffer = self.buffer.as_str().to_string();
        buffer.push_str(&html.as_str());

        HtmlRenderer {
            buffer: HtmlBlock::from_str(&buffer),
        }
    }

    fn finalize(&self) -> &Self::Output {
        &self.buffer
    }
}
