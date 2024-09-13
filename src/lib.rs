#[derive(Debug, Clone, PartialEq, Eq)]
struct Element {
    name: String,
    attributes: Vec<(String, String)>,
    children: Vec<Element>,
}

fn match_literal(expected: &'static str) -> impl Fn(&str) -> Result<(&str, ()), &str> {
    move |input| match input.split_at(expected.len()) {
        (next, rest) if next == expected => Ok((rest, ())),
        _ => Err(input),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn literal_parser() {
        let parse_joe = match_literal("Hello Joe!");
        assert_eq!(Ok(("", ())), parse_joe("Hello Joe!"));

        assert_eq!(
            Ok((" Hello Robert!", ())),
            parse_joe("Hello Joe! Hello Robert!")
        );

        assert_eq!(Err("Hello Mike!"), parse_joe("Hello Mike!"));
    }
}
