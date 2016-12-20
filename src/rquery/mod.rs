use std::default::Default;

use tendril::TendrilSink;
use html5ever::parse_document;
use html5ever::rcdom::RcDom;

use std::io::Read;

mod cond;
mod node;
mod selector;
mod context;

use rquery::context::SelectResult;

pub fn load<R: Read>(input: &mut R) -> Box<Fn(&str) -> SelectResult> {
    let dom = parse_document(RcDom::default(), Default::default())
        .from_utf8()
        .read_from(input)
        .unwrap();
    let root = node::parse(dom.document);
    context::create_context(root)
}
