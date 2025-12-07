use godot::{
    classes::{EditorPlugin, IEditorPlugin},
    prelude::*,
};

struct CybuildExtension;

#[gdextension]
unsafe impl ExtensionLibrary for CybuildExtension {}

#[derive(GodotClass)]
#[class(tool, init, base=EditorPlugin)]
struct CybuildEditorPlugin {
    base: Base<EditorPlugin>,
}

#[godot_api]
impl IEditorPlugin for CybuildEditorPlugin {
    fn enter_tree(&mut self) {
        // Perform typical plugin operations here.
        godot_print!("Hello from cybuild");
    }

    fn exit_tree(&mut self) {
        // Perform typical plugin operations here.
    }
}
