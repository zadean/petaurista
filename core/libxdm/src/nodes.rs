pub trait Node {
    fn node_name(&self) -> String;
}

pub enum Nodes {
    Document,
    Element,
}

pub struct Document<'a> {
    id: u64,
    uri: String,
    base_uri: String,
    children: Vec<&'a Nodes>,
}

pub struct Fragment<'a> {
    id: u64,
    uri: String,
    base_uri: String,
    children: Vec<&'a Nodes>,
}

pub struct Element<'a> {
    id: u64,
    //namespaces: Vec<&Namespace>,
    //attributes: Vec<&Attribute>,
    children: Vec<&'a Node>,
    parent: &'a Node,
    name: (String, String, String),
}

impl<'a> Node for Nodes {
    fn node_name(&self) -> std::string::String {
        "".to_string()
    }
}
