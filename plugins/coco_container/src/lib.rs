use core_model::CocoConfig;
use core_model::PluginInterface;

pub struct CocoContainer {}

impl PluginInterface for CocoContainer {
    fn name(&self) -> &'static str {
        "coco.swagger"
    }

    fn on_plugin_load(&self) {}

    fn on_plugin_unload(&self) {}

    fn execute(&self, config: CocoConfig) {
        println!("{:?}", config);
    }
}

impl Default for CocoContainer {
    fn default() -> Self {
        CocoContainer {}
    }
}

#[no_mangle]
pub fn plugin() -> Box<dyn PluginInterface> {
    Box::new(CocoContainer::default())
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
