/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

#[test]
fn smoke_restyle_hints() {
    use cssparser::{Parser, ParserInput};
    use selectors::parser::SelectorList;
    use style::restyle_hints::DependencySet;
    use style::selector_parser::SelectorParser;
    use style::stylesheets::{Origin, Namespaces};
    let namespaces = Namespaces::default();
    let parser = SelectorParser {
        stylesheet_origin: Origin::Author,
        namespaces: &namespaces,
    };

    let mut dependencies = DependencySet::new();

    let mut input = ParserInput::new(":not(:active) ~ label");
    let mut p = Parser::new(&mut input);
    let selectors = SelectorList::parse(&parser, &mut p).unwrap();
    assert_eq!((selectors.0).len(), 1);

    let selector = (selectors.0).first().unwrap();
    dependencies.note_selector(selector);
    assert_eq!(dependencies.len(), 1);
}
