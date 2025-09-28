use crate::html::node::IRNode;
use crate::html::trust::HtmlBlock;

pub trait Renderer:Clone {
    type Output;
    fn visit_node_begin(&self, node: &IRNode) -> Self;
    fn visit_node_end(&self, node: &IRNode) -> Self;
    fn finalize(&self) -> Self::Output;
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

    fn visit_node_begin(&self, node: &IRNode) -> HtmlRenderer {
        let buffer = self.buffer.clone();
        let mut buffer = buffer.to_str();
        buffer.push('<');
        buffer.push_str(&node.get_tag());
        buffer.push_str(&node.get_attrs());
        buffer.push('>');

        if let Some(content) = node.get_content() {
            buffer.push_str(&content);
        }
        HtmlRenderer {
            buffer: HtmlBlock::from_str(&buffer),
        }
    }

    fn visit_node_end(&self, node: &IRNode) -> Self {
        let buffer = self.buffer.clone();
        let mut buffer = buffer.to_str();
        if !node.is_self_closing() {
            buffer.push_str("</");
            buffer.push_str(&node.get_tag());
            buffer.push('>');
        }

        HtmlRenderer {
            buffer: HtmlBlock::from_str(&buffer),
        }
    }

    fn finalize(&self) -> Self::Output {
        self.buffer.clone()
    }
}
