use crate::event::Event;
use libxdm::nodes::{Namespace, Node, NodeId};
use libxdm::NodeCollection;

// Take a list of events and build a NodeCollection.
pub fn handle_event(
    event: Event,
    state: (NodeId, Vec<NodeId>, u16, NodeCollection),
) -> (NodeId, Vec<NodeId>, u16, NodeCollection) {
    let (curr_id, mut parent_stack, depth, mut container) = state;
    match event {
        Event::StartDocument => {
            let node = Node::document();
            container.add_node(node);
            parent_stack.push(curr_id);
            (curr_id + 1, parent_stack, depth + 1, container)
        }
        Event::EndDocument => {
            parent_stack.pop();
            (curr_id, parent_stack, depth - 1, container)
        }
        Event::StartPrefixMapping { prefix: p, uri: u } => {
            let ns = Namespace {
                pos: curr_id + 1,
                prefix: p,
                uri: u,
            };
            container.namespaces.push(ns);
            (curr_id, parent_stack, depth, container)
        }
        Event::EndPrefixMapping => (curr_id, parent_stack, depth, container),
        Event::StartElement {
            uri: u,
            local_name: l,
            prefix: p,
            attributes: atts,
        } => {
            let parent_id = parent_stack.last().unwrap();
            let (next_id, atts) =
                atts.iter()
                    .fold((curr_id + 1, Vec::new()), |(id, mut vc), (u, p, l, v)| {
                        let att = Node::attribute(
                            curr_id,
                            container.qname(l.to_string(), p.to_string(), u.to_string()),
                            depth + 2,
                            v.to_string(),
                        );
                        container.add_node(att);
                        vc.push(id);
                        (id + 1, vc)
                    });
            let node = Node::element(*parent_id, container.qname(l, p, u), depth + 1, atts);
            container.insert_node(curr_id, node); // pushes atts right
            container.add_child_node(*parent_id, curr_id);
            parent_stack.push(curr_id);
            (next_id, parent_stack, depth + 1, container)
        }
        Event::EndElement => {
            parent_stack.pop();
            (curr_id, parent_stack, depth - 1, container)
        }
        Event::Characters { text } => {
            let parent_id = parent_stack.last().unwrap();
            let node = Node::text(*parent_id, depth, text);
            container.add_node(node);
            container.add_child_node(*parent_id, curr_id);
            (curr_id + 1, parent_stack, depth, container)
        }
        Event::IgnorableWhitespace { text } => {
            let parent_id = parent_stack.last().unwrap();
            let node = Node::text(*parent_id, depth, text);
            container.add_node(node);
            container.add_child_node(*parent_id, curr_id);
            (curr_id + 1, parent_stack, depth, container)
        }
        Event::ProcessingInstruction { target, data } => {
            let parent_id = parent_stack.last().unwrap();
            let node = Node::processing_instruction(
                *parent_id,
                depth,
                container.qname(target, "".to_string(), "".to_string()),
                data,
            );
            container.add_node(node);
            container.add_child_node(*parent_id, curr_id);
            (curr_id + 1, parent_stack, depth, container)
        }
        Event::Comment { text } => {
            let parent_id = parent_stack.last().unwrap();
            let node = Node::comment(*parent_id, depth, text);
            container.add_node(node);
            container.add_child_node(*parent_id, curr_id);
            (curr_id + 1, parent_stack, depth, container)
        }
        // skipping DTD stuff
        _ => {
            println!("skipping something");
            (curr_id, parent_stack, depth, container)
        }
    }
}
