extern crate libxdm;

use libxdm::node_collection::NodeCollection;
use libxdm::nodes::Node;

fn main() {
    let mut con = NodeCollection::new("someBaseUri".to_string(), "someUri".to_string());
    let doc1 = Node::document();
    con.add_node(doc1);
    let doc2 = Node::document();
    con.add_node(doc2);
    let doc3 = Node::document();
    con.add_node(doc3);
    for x in 3..10 {
        let el = Node::element(
            1,
            con.qname("a".to_string(), "b".to_string(), "c".to_string()),
            1,
            Vec::new(),
        );
        con.add_node(el);
        con.add_child_node(1, x);
    }
    println!("{:?}", con);
    let chld = match con.nodes.get(1).as_ref() {
        None => None,
        Some(&Node { children: c, .. }) => c.as_ref(),
    };
    println!("{:?}", chld);
    for n in chld.expect("children") {
        let nn = con.nodes.get(*n);
        println!("{:?}", nn);
    }
    // let f = libxdm::xpath::name_match(&mut con, Some("a".to_string()), None);
    // let f2 = libxdm::xpath::type_match(NodeKind::Comment);
    // let con = con;
    // let desc = con.descendants(1);
    // let fil = desc.iter().filter(f).filter(f2).collect::<Vec<&NodeTup>>();
    // println!("{:?}", fil)
}
