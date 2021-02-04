pub use go::go_facet;
pub use java::jvm_facet::JvmFacet;
pub use java::JavaFacet;
pub use java::JavaModuleData;
pub use javascript::javascript_facet;
pub use python::python_facet;
pub use rust::rust_facet;

/// Java
pub mod java;

/// JavaScript
pub mod javascript;

/// Python
pub mod python;

/// golang
pub mod go;

/// rust
pub mod rust;

pub type Facet = dyn erased_serde::Serialize;
