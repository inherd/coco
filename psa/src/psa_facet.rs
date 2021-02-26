#[derive(Serialize)]
pub struct Facet {
    pub name: String,
}

#[cfg(test)]
mod tests {
    use crate::Facet;

    #[test]
    fn should_create_facet() {
        let facet = Facet {
            name: "spring framework".to_string(),
        };

        assert_eq!(facet.name, "spring framework");
    }
}
