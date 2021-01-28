use pest::Parser;

#[derive(Parser)]
#[grammar = "infrastructure/include_parser/include.pest"]
struct IncludeParser;

pub fn parse_code(code: &str) {
    let pairs = IncludeParser::parse(Rule::c_import, code).unwrap_or_else(|e| panic!("{}", e));

    println!("{:?}", pairs);
    for pair in pairs {
        println!("Rule:    {:?}", pair.as_rule());
        println!("Span:    {:?}", pair.as_span());
        println!("Text:    {}", pair.as_str());
    }
}

#[cfg(test)]
mod test {
    use crate::infrastructure::include_parser::parse_code;

    #[test]
    fn should_parse_c_include() {
        parse_code("#include<stdio.h>");
        parse_code("#include \"stdio.h\"");
        parse_code(
            "#include
        \"stdio.h\"",
        );
    }
}
