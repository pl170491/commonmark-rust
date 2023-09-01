pub struct NodeTree {
    root: Vec<Node>,
}

impl NodeTree {
    pub fn new() -> Self {
        NodeTree { root: vec![] }
    }

    pub fn add(&mut self, node: Node) {
        self.root.push(node);
    }

    pub fn nodes(&self) -> &Vec<Node> {
        &self.root
    }
}

#[derive(Debug, PartialEq)]
pub struct Node {
    child_nodes: Vec<Node>, // Derived from "NodeList" object
    node_type: NodeType,
}

impl Node {
    pub fn new(node_type: NodeType) -> Self {
        Node {
            child_nodes: vec![],
            node_type: node_type,
        }
    }

    pub fn to_html(&self) -> String {
        match &self.node_type {
            NodeType::ElementNode(_) => "".to_string(),
            NodeType::TextNode(t) => {
                let html = "".to_string();
                html + "<p>" + t.data() + "</p>"
            }
        }
    }

    pub fn node_name(&self) -> &str {
        match &self.node_type {
            NodeType::TextNode(_) => "#text",
            NodeType::ElementNode(e) => e.tag.node_name(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum NodeType {
    ElementNode(Element),
    TextNode(Text),
}

#[derive(Debug, PartialEq)]
pub struct Element {
    tag: HtmlTag,
}

impl Element {
    pub fn new(tag: HtmlTag) -> Self {
        Element { tag: tag }
    }
}

// There are also technically XML or XHTML tags,
// but we will only concern ourselves with HTML.
#[derive(Debug, PartialEq)]
pub enum HtmlTag {
    HR,
}

impl HtmlTag {
    fn node_name(&self) -> &str {
        match &self {
            Self::HR => "HR",
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Text {
    data: String, // Derived from 'CharacterData' object
}

impl Text {
    pub fn new(s: &str) -> Self {
        Text {
            data: s.to_string(),
        }
    }

    pub fn data(&self) -> &str {
        &self.data
    }
}
