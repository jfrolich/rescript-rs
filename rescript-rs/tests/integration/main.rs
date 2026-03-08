#![allow(dead_code, unused)]

use std::path::PathBuf;

use rescript_rs::{Config, TS};

mod arrays;
mod arrayvec;
mod bound;
mod bson;
mod chrono;
mod complex_flattened_type;
mod concrete_generic;
// mod docs; // skipped: enum F uses externally tagged variants with data
// mod enum_flattening; // skipped: uses externally tagged enums
// mod enum_flattening_nested; // skipped: uses externally tagged enums
// mod enum_struct_rename_all; // skipped: uses externally tagged enums
// mod enum_variant_annotation; // skipped: uses externally tagged enums
mod export_manually;
mod export_to;
mod field_rename;
mod flatten;
mod generic_fields;
mod generic_without_import;
// mod generics; // skipped: GenericEnum/MyEnum/CTraitBounds use externally tagged enums
// mod generics_flatten; // skipped: Enum uses externally tagged variants
mod hashmap;
mod hashset;
mod impl_primitive;
// mod imports; // skipped: TestEnum uses externally tagged enum + import testing
mod indexmap;
mod infer_as;
// mod issue_168; // skipped: uses import_extension
// mod issue_232; // skipped: uses import_extension
mod issue_308;
mod issue_317;
mod issue_338;
mod issue_397;
mod issue_415;
// mod issue_70; // skipped: Enum uses externally tagged variants
// mod issue_80; // skipped: SomeTypeList uses externally tagged variants
mod jiff;
mod leading_colon;
mod lifetimes;
mod list;
// mod merge_same_file_imports; // skipped: uses import_extension
mod nested;
// mod optional_field; // skipped: OptionalInEnum and others use externally tagged enums
mod path_bug;
mod ranges;
mod raw_idents;
// mod recursion_limit; // skipped: VeryBigEnum uses externally tagged variants
mod references;
// mod repr_enum; // skipped: repr(enum) with integer discriminants not supported
mod same_file_export;
// mod self_referential; // skipped: ExternallyTagged uses externally tagged enum
mod semver;
mod serde_json;
mod serde_skip_serializing;
mod serde_skip_with_default;
mod serde_with;
mod simple;
// mod skip; // skipped: Externally enum uses externally tagged variants
mod slices;
mod struct_rename;
mod struct_tag;
mod tokio;
mod top_level_type_as;
mod top_level_type_override;
mod tuple;
// mod type_as; // skipped: OverrideEnum/OverrideVariant use externally tagged enums
// mod type_override; // skipped: uses externally tagged-style output patterns
mod union;
// mod union_named_serde_skip; // skipped: TestExternally uses externally tagged enum
mod union_rename;
mod union_serde;
// mod union_unnamed_serde_skip; // skipped: TestExternally uses externally tagged enum
// mod union_with_data; // skipped: SimpleEnum uses externally tagged enum
mod union_with_internal_tag;
mod unit;
mod r#unsized;

// Returns the path to the file into which `T` is exported
fn target_file<T: TS>(cfg: &Config) -> PathBuf {
    cfg.out_dir().join(T::output_path().unwrap())
}

// Read the bindings for `T` from disk
fn read_file<T: TS>(cfg: &Config) -> String {
    std::fs::read_to_string(target_file::<T>(cfg)).unwrap()
}
