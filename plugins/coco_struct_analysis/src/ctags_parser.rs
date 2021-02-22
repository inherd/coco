pub struct CtagsParser;

impl CtagsParser {
    pub fn parse_class(str: &str) {}
    pub fn parse_method_methods() {}
}

#[cfg(test)]
mod test {
    pub fn should_parse_java_class() {
        let tags = "AllowConcurrentEvents	AllowConcurrentEvents.java	/^public @interface AllowConcurrentEvents {$/;\"	interface	line:40	language:Java";
    }
}
