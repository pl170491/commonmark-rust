mod atx_heading;
mod common;
mod thematic_break;

use thematic_break::line_hr;

use crate::html::NodeTree;

pub fn parse(input: &str) -> String {
    let nodes = mdx_ast(input);
    let mut html = String::from("");

    for node in nodes.nodes() {
        html += node.to_html().as_str();
    }

    html
}

fn mdx_ast(input: &str) -> NodeTree {
    let mut node_tree = NodeTree::new();

    match line_hr(input) {
        Ok((_, node)) => {
            node_tree.add(node);
        }
        _ => {}
    };

    node_tree
}
