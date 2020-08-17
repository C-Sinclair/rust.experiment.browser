mod html;

use html::parser::HtmlParser;

fn main() {
    let nodes = HtmlParser::new(
        "<div x=1>Test</div>"
    ).parse_nodes();
    html::dom::pretty_print(&nodes[0], 1);
}