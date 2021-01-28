use pest::Parser;

#[derive(Parser)]
#[grammar = "infrastructure/include_parser/ident.pest"]
struct IdentParser;

pub fn parse_code(code: &str) {
    let pairs = IdentParser::parse(Rule::imports, code).unwrap_or_else(|e| panic!("{}", e));

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
        let include = "#include<stdio.h>";
        parse_code(include);
    }
}
