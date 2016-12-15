extern crate html5ever;
extern crate tendril;
extern crate regex;

mod rquery;

static HTML: &'static str = "
<html>
<head>
</head>
<body>
<div id='container'>
    <div class='cch'></div>
</div>
</body>
</html>
";

fn main() {
    // let stdin = io::stdin();
    let mut input = HTML.as_bytes();
    let s = rquery::load(&mut input);
    for node in s("div") {
        println!("{}", node);
    }
    // let dom = parse_document(RcDom::default(), Default::default())
    //     .from_utf8()
    //     .read_from(&mut input)
    //     .unwrap();
    // let root = parse(dom.document);
    // println!("{:?}", root);
    // if !dom.errors.is_empty() {
    //     println!("\nParse errors:");
    //     for err in dom.errors.into_iter() {
    //         println!("    {}", err);
    //     }
    // }
}
