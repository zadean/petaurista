pub enum Event {
    StartDocument,
    EndDocument,
    StartPrefixMapping {
        prefix: String,
        uri: String,
    },
    EndPrefixMapping, // {prefix: String,}
    StartElement {
        uri: String,
        local_name: String,
        prefix: String,
        attributes: Vec<(String, String, String, String)>,
    },
    EndElement,
    Characters {
        text: String,
    },
    IgnorableWhitespace {
        text: String,
    },
    ProcessingInstruction {
        target: String,
        data: String,
    },
    Comment {
        text: String,
    },
    // StartDTD {
    //     name: String,
    //     public_id: String,
    //     system_id: String,
    // },
    // StartEntity {
    //     sys_id: String,
    // },
    // EndEntity {
    //     sys_id: String,
    // },
    // ElementDecl {
    //     name: String,
    //     model: String,
    // },
    // AttributeDecl {
    //     element_name: String,
    //     attribute_name: String,
    //     att_type: String,
    //     mode: String,
    //     value: String,
    // },
    // InternalEntityDecl {
    //     name: String,
    //     value: String,
    // },
    // ExternalEntityDecl {
    //     name: String,
    //     public_id: String,
    //     system_id: String,
    // },
    // UnparsedEntityDecl {
    //     name: String,
    //     public_id: String,
    //     system_id: String,
    //     ndata: String,
    // },
    // NotationDecl {
    //     name: String,
    //     public_id: String,
    //     system_id: String,
    // },
    Unkown,
}
