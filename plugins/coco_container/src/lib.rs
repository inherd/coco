fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use dockerfile_parser::Dockerfile;

    #[test]
    pub fn demo() {
        let dockerfile = Dockerfile::parse(
            r#"
  FROM alpine:3.11 as builder
  RUN echo "hello world" > /hello-world

  FROM scratch
  COPY --from=builder /hello-world /hello-world
"#,
        )
        .unwrap();

        for stage in dockerfile.iter_stages() {
            println!("stage #{}", stage.index);
            for ins in stage.instructions {
                println!("  {:?}", ins);
            }
        }
    }
}
