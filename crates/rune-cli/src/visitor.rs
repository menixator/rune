use rune::compile::CompileVisitor;
use rune::meta::{CompileMeta, CompileMetaKind};
use rune::Hash;

/// Attribute to collect.
#[derive(Debug, Clone, Copy)]
pub(crate) enum Attribute {
    /// Do not collect any functions.
    None,
    /// Collect `#[test]` functions.
    Test,
    /// Collect `#[bench]` functions.
    Bench,
}

/// A compile visitor that collects functions with a specific attribute.
pub struct FunctionVisitor {
    attribute: Attribute,
    functions: Vec<(Hash, CompileMeta)>,
}

impl FunctionVisitor {
    pub(crate) fn new(kind: Attribute) -> Self {
        Self {
            attribute: kind,
            functions: Default::default(),
        }
    }

    /// Convert visitor into test functions.
    pub(crate) fn into_functions(self) -> Vec<(Hash, CompileMeta)> {
        self.functions
    }
}

impl CompileVisitor for FunctionVisitor {
    fn register_meta(&mut self, meta: &CompileMeta) {
        let type_hash = match (self.attribute, &meta.kind) {
            (
                Attribute::Test,
                CompileMetaKind::Function {
                    is_test, type_hash, ..
                },
            ) if *is_test => type_hash,
            (
                Attribute::Bench,
                CompileMetaKind::Function {
                    is_bench,
                    type_hash,
                    ..
                },
            ) if *is_bench => type_hash,
            _ => return,
        };

        self.functions.push((*type_hash, meta.clone()));
    }
}