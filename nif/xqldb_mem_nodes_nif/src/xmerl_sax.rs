use crate::collection_builder;
use crate::event::Event;

use libxdm::nodes::NodeId;
use libxdm::NodeCollection;
use rustler::types::atom::Atom;
use rustler::Error;
use rustler::{ListIterator, Term};

// Take a list of xmerl_sax events and build a NodeCollection.
pub fn parse_list<'a>(args: &[Term<'a>]) -> Result<libxdm::NodeCollection, Error> {
    let list_iter: ListIterator = args[0].decode()?;
    let base_uri: String = args[1].decode()?;
    let (_, _, _, container) = list_iter.fold(
        (
            0,
            Vec::<NodeId>::new(),
            0,
            NodeCollection::new(base_uri, "".to_string()),
        ),
        |state, tuple| {
            let event = decode_event(tuple);
            collection_builder::handle_event(event, state)
        },
    );
    Ok(container)
}

fn decode_event(event: rustler::Term) -> Event {
    if let Ok((t1, t2)) = event.decode::<(Atom, ListIterator)>() {
        if t1 == atoms::end_prefix_mapping() {
            Event::EndPrefixMapping
        } else if t1 == atoms::characters() {
            Event::Characters {
                text: erl_char_list_to_string(t2),
            }
        } else if t1 == atoms::ignorable_whitespace() {
            Event::IgnorableWhitespace {
                text: erl_char_list_to_string(t2),
            }
        } else if t1 == atoms::comment() {
            Event::Comment {
                text: erl_char_list_to_string(t2),
            }
        // } else if t1 == atoms::start_entity() {
        //     Event::StartEntity {
        //         sys_id: erl_char_list_to_string(t2),
        //     }
        // } else if t1 == atoms::end_entity() {
        //     Event::EndEntity {
        //         sys_id: erl_char_list_to_string(t2),
        //     }
        } else {
            Event::Unkown
        }
    } else if let Ok((t1, t2, t3)) = event.decode::<(Atom, ListIterator, ListIterator)>() {
        if t1 == atoms::start_prefix_mapping() {
            Event::StartPrefixMapping {
                prefix: erl_char_list_to_string(t2),
                uri: erl_char_list_to_string(t3),
            }
        } else if t1 == atoms::processing_instruction() {
            Event::ProcessingInstruction {
                target: erl_char_list_to_string(t2),
                data: erl_char_list_to_string(t3),
            }
        // } else if t1 == atoms::element_decl() {
        //     Event::ElementDecl {
        //         name: erl_char_list_to_string(t2),
        //         model: erl_char_list_to_string(t3),
        //     }
        // } else if t1 == atoms::internal_entity_decl() {
        //     Event::InternalEntityDecl {
        //         name: erl_char_list_to_string(t2),
        //         value: erl_char_list_to_string(t3),
        //     }
        } else {
            Event::Unkown
        }
    } else if let Ok((t1, _t2, _t3, _t4)) = event.decode::<(
        Atom,
        ListIterator,
        ListIterator,
        (ListIterator, ListIterator),
    )>() {
        if t1 == atoms::end_element() {
            Event::EndElement
        } else {
            Event::Unkown
        }
    // } else if let Ok((t1, t2, t3, t4)) = event.decode::<(Atom, String, String, String)>() {
    //     if t1 == atoms::start_dtd() {
    //         Event::StartDTD {
    //             name: t2,
    //             public_id: t3,
    //             system_id: t4,
    //         }
    //     } else if t1 == atoms::external_entity_decl() {
    //         Event::ExternalEntityDecl {
    //             name: t2,
    //             public_id: t3,
    //             system_id: t4,
    //         }
    //     } else if t1 == atoms::notation_decl() {
    //         Event::NotationDecl {
    //             name: t2,
    //             public_id: t3,
    //             system_id: t4,
    //         }
    //     } else {
    //         Event::Unkown
    //     }
    // } else if let Ok((t1, t2, t3, t4, t5)) =
    //     event.decode::<(Atom, String, String, String, String)>()
    // {
    //     if t1 == atoms::unparsed_entity_decl() {
    //         Event::UnparsedEntityDecl {
    //             name: t2,
    //             public_id: t3,
    //             system_id: t4,
    //             ndata: t5,
    //         }
    //     } else {
    //         Event::Unkown
    //     }
    } else if let Ok((t1, t2, t3, (t4, _), t5)) = event.decode::<(
        Atom,
        ListIterator,
        ListIterator,
        (ListIterator, ListIterator),
        ListIterator,
    )>() {
        if t1 == atoms::start_element() {
            let atts = t5
                .map(|e| e.decode::<(ListIterator, ListIterator, ListIterator, ListIterator)>())
                .filter_map(Result::ok)
                .map(|(a, b, c, d)| {
                    (
                        erl_char_list_to_string(a),
                        erl_char_list_to_string(b),
                        erl_char_list_to_string(c),
                        erl_char_list_to_string(d),
                    )
                })
                .collect::<Vec<(String, String, String, String)>>();
            Event::StartElement {
                uri: erl_char_list_to_string(t2),
                local_name: erl_char_list_to_string(t3),
                prefix: erl_char_list_to_string(t4),
                attributes: atts,
            }
        } else {
            Event::Unkown
        }
    // } else if let Ok((t1, t2, t3, t4, t5, t6)) =
    //     event.decode::<(Atom, String, String, String, String, String)>()
    // {
    //     if t1 == atoms::attribute_decl() {
    //         Event::AttributeDecl {
    //             element_name: t2,
    //             attribute_name: t3,
    //             att_type: t4,
    //             mode: t5,
    //             value: t6,
    //         }
    //     } else {
    //         Event::Unkown
    //     }
    } else if let Ok(t1) = event.decode::<Atom>() {
        if t1 == atoms::start_document() {
            Event::StartDocument
        } else if t1 == atoms::end_document() {
            Event::EndDocument
        } else {
            Event::Unkown
        }
    } else {
        println!("{:?}", event);
        Event::Unkown
    }
}

fn erl_char_list_to_string(chars: ListIterator) -> String {
    let mut buf = Vec::new();
    for c in chars {
        let d = match c.decode::<u32>() {
            Ok(d) => d,
            _ => 0,
        };
        match d {
            // 1 byte
            0x0000..=0x007F => buf.push(d as u8),
            // 2 bytes
            0x0080..=0x07FF => {
                let b1 = ((d >> 6) | 0xC0) as u8;
                let b2 = ((d & 0x3F) | 0x80) as u8;
                buf.push(b1);
                buf.push(b2);
            }
            // 3 bytes
            0x0800..=0xFFFF => {
                let b1 = ((d >> 12) | 0xE0) as u8;
                let b2 = (((d >> 6) & 0x3F) | 0x80) as u8;
                let b3 = ((d & 0x3F) | 0x80) as u8;
                buf.push(b1);
                buf.push(b2);
                buf.push(b3);
            }
            // 4 bytes
            0x10000..=0x10FFFF => {
                let b1 = ((d >> 18) | 0xF0) as u8;
                let b2 = (((d >> 12) & 0x3F) | 0x80) as u8;
                let b3 = (((d >> 6) & 0x3F) | 0x80) as u8;
                let b4 = ((d & 0x3F) | 0x80) as u8;
                buf.push(b1);
                buf.push(b2);
                buf.push(b3);
                buf.push(b4);
            }
            _ => panic!(print!("non UTF-8  {:?}", d)),
        }
    }
    String::from_utf8(buf).unwrap()
}

mod atoms {
    rustler_atoms! {
        atom ok;
        atom start_document = "startDocument";
        atom end_document = "endDocument";
        atom start_prefix_mapping = "startPrefixMapping";
        atom end_prefix_mapping = "endPrefixMapping";
        atom start_element = "startElement";
        atom end_element = "endElement";
        atom characters;
        atom ignorable_whitespace = "ignorableWhitespace";
        atom processing_instruction = "processingInstruction";
        atom comment;
        atom start_dtd = "startDTD";
        atom start_entity = "startEntity";
        atom end_entity = "endEntity";
        atom element_decl = "elementDecl";
        atom attribute_decl = "attributeDecl";
        atom internal_entity_decl = "internalEntityDecl";
        atom external_entity_decl = "externalEntityDecl";
        atom unparsed_entity_decl = "unparsedEntityDecl";
        atom notation_decl = "notationDecl";
    }
}

// startDocument
// endDocument

// {endPrefixMapping, Prefix}
// {characters, string()}
// {ignorableWhitespace, string()}
// {comment, string()}
// {startEntity, SysId}
// {endEntity, SysId}

// {startPrefixMapping, Prefix, Uri}
// {processingInstruction, Target, Data}
// {elementDecl, Name, Model}
// {internalEntityDecl, Name, Value}

// {endElement, Uri, LocalName, {Prefix, LocalName}}
// {startDTD, Name, PublicId, SystemId}
// {externalEntityDecl, Name, PublicId, SystemId}
// {notationDecl, Name, PublicId, SystemId}

// {startElement, Uri, LocalName, {Prefix, LocalName}, [{Uri, Prefix, AttributeName, Value}]}
// {unparsedEntityDecl, Name, PublicId, SystemId, Ndata}

// {attributeDecl, ElementName, AttributeName, Type, Mode, Value}
