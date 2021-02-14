use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Architecture {
    pub analysis: String,
    pub synthesis: String,
    pub evaluation: String,
    pub implementation: String,
    pub maintenance: String,
    pub evolution: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct ArchitectureEvolution {
    pub fitness: ArchitectureFitness,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct ArchitectureFitness {
    pub tests: String,
    pub tests_coverage: i32,
}

pub enum Practise {
    /// ?
    TDD,
}

pub enum LayerArchitecture {
    /// PresentationDomainDataLayering
    FlatMVC,
    /// Domain/PresentationDomainDataLayering
    NestedMVC,
    /// Domain-driven design
    DDD,
    /// Model with Behavior
    DomainObject,
    /// Model Object
    ModelObject,
}
