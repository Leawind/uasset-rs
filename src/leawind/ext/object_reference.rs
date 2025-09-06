use crate::ObjectReference;

impl core::fmt::Debug for ObjectReference {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ObjectReference::None => write!(f, "None"),
            ObjectReference::Export { export_index: i } => write!(f, "Export[{}]", i),
            ObjectReference::Import { import_index: i } => write!(f, "Import[{}]", i),
        }
    }
}
