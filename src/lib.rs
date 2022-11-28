use swc_core::ecma::{
    ast::Ident,
    ast::Program,
    atoms::JsWord,
    transforms::testing::test,
    visit::{as_folder, FoldWith, VisitMut},
};
use swc_core::plugin::{
    metadata::TransformPluginMetadataContextKind, plugin_transform,
    proxies::TransformPluginProgramMetadata,
};

pub struct TransformVisitor {
    pub path: String,
}

impl VisitMut for TransformVisitor {
    // Implement necessary visit_mut_* methods for actual custom transform.
    // A comprehensive list of possible visitor methods can be found here:
    // https://rustdoc.swc.rs/swc_ecma_visit/trait.VisitMut.html
    fn visit_mut_ident(&mut self, ident: &mut Ident) {
        // ident.sym = format!("swc_{}", ident.sym).into();
        print!(" ");
        if ident.sym == String::from("__translationGroup") {
            let path_string_ident = format!("'{}'", self.path);
            ident.sym = JsWord::from(path_string_ident);
        }
    }
}

/// An example plugin function with macro support.
/// `plugin_transform` macro interop pointers into deserialized structs, as well
/// as returning ptr back to host.
///
/// It is possible to opt out from macro by writing transform fn manually
/// if plugin need to handle low-level ptr directly via
/// `__transform_plugin_process_impl(
///     ast_ptr: *const u8, ast_ptr_len: i32,
///     unresolved_mark: u32, should_enable_comments_proxy: i32) ->
///     i32 /*  0 for success, fail otherwise.
///             Note this is only for internal pointer interop result,
///             not actual transform result */`
///
/// This requires manual handling of serialization / deserialization from ptrs.
/// Refer swc_plugin_macro to see how does it work internally.
#[plugin_transform]
pub fn process_transform(program: Program, _metadata: TransformPluginProgramMetadata) -> Program {
    print!(
        "Filename: {:?}",
        _metadata.get_context(&TransformPluginMetadataContextKind::Filename)
    );
    print!(
        "Cwd: {:?}",
        _metadata.get_context(&TransformPluginMetadataContextKind::Cwd)
    );

    let filename = _metadata
        .get_context(&TransformPluginMetadataContextKind::Filename)
        .unwrap();

    let cwd = _metadata
        .get_context(&TransformPluginMetadataContextKind::Cwd)
        .unwrap();

    let prefix = format!("{}/src/", cwd);

    let relative_path = filename.strip_prefix(&prefix).unwrap_or("").to_string();

    program.fold_with(&mut as_folder(TransformVisitor {
        path: String::from(relative_path),
    }))
}

// An example to test plugin transform.
// Recommended strategy to test plugin's transform is verify
// the Visitor's behavior, instead of trying to run `process_transform` with mocks
// unless explicitly required to do so.
test!(
    Default::default(),
    |_| as_folder(TransformVisitor {
        path: String::from("none")
    }),
    boo,
    // Input codes
    r#"{t(__translationGroup)`Skip to content`}"#,
    // Output codes after transformed with plugin
    r#"{t(test)`Skip to content`}"#
);
