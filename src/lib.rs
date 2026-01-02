use std::path::Path;

pub(crate) use godot::prelude::*;
use godot::{
    classes::{EditorPlugin, IEditorPlugin, ProjectSettings},
    global::PropertyHint,
};

use crate::cli_wrapper::CliWrapper;

mod cli_wrapper;
mod env_detect;

/// Godot extension entry point for Cybuild integration.
struct CybuildExtension;

#[gdextension]
unsafe impl ExtensionLibrary for CybuildExtension {}

/// Editor plugin for Cybuild integration in Godot.
#[derive(GodotClass)]
#[class(tool, init, base=EditorPlugin)]
struct CybuildEditorPlugin {
    base: Base<EditorPlugin>,
    enabled: bool,
}

#[godot_api]
impl IEditorPlugin for CybuildEditorPlugin {
    fn enter_tree(&mut self) {
        let path = ProjectSettings::singleton()
            .get_setting_ex("cybuild/executable_path")
            .default_value(&GString::from("cybuild").to_variant())
            .done()
            .to_string();

        if path.is_empty() {
            self.enabled = env_detect::check_if_installed(Path::new("cybuild"));
        } else {
            self.enabled = env_detect::check_if_installed(Path::new(&path));
        }

        Self::init_settings();

        if Self::get_project_target_name().is_empty() {
            godot_warn!("project target name unset; Cybuild will not be used.");
        }
    }

    fn exit_tree(&mut self) {
        // Perform typical plugin cleanup operations here if needed.
    }

    fn build(&mut self) -> bool {
        if Self::get_project_target_name().is_empty() {
            godot_warn!("project target name unset; Cybuild will not be used.");
            return true;
        } else {
            godot_print!("Building with Cybuild...");
        }

        let mut cli = CliWrapper::new();
        let target_name = Self::get_project_target_name();
        let output_dir = Self::get_setting_as_string("cybuild/outdir");
        let exe_path = Self::get_setting_as_string("cybuild/executable_path");
        let dep_only = ProjectSettings::singleton()
            .get_setting("cybuild/dependencies_only")
            .booleanize();
        let manifest_path = Self::get_setting_as_string("cybuild/path");

        if !exe_path.is_empty() {
            cli.executable(exe_path);
        }

        // Make sure it's globalized because cybuild can't process path like `res://`
        cli.output(
            ProjectSettings::singleton()
                .globalize_path(&output_dir)
                .to_string(),
        )
        .manifest(
            ProjectSettings::singleton()
                .globalize_path(&manifest_path)
                .to_string(),
        );

        if dep_only {
            cli.build_dependencies(target_name.as_str())
                .unwrap_or(false)
        } else {
            cli.build_target(target_name.as_str()).unwrap_or(false)
        }
    }
}

impl CybuildEditorPlugin {
    /// Initialize Cybuild-related project settings if they are not already set.
    fn init_settings() {
        let mut ps = ProjectSettings::singleton();

        // Helper to set a setting if it doesn't exist
        fn set_default<T: ToGodot>(ps: &mut ProjectSettings, key: &str, value: T) {
            if !ps.has_setting(key) {
                ps.set_setting(key, &value.to_variant());
            }
        }

        set_default(&mut ps, "cybuild/path", GString::new());
        set_default(&mut ps, "cybuild/outdir", GString::from("res://artifacts"));
        set_default(&mut ps, "cybuild/project_target_name", GString::new());
        // TODO: This setting is not yet implemented in both plugin & program.
        set_default(&mut ps, "cybuild/dependencies_only", true);
        set_default(&mut ps, "cybuild/executable_path", GString::new());

        // Add property info for settings (tooltips not supported by Godot yet)
        ps.add_property_info(&vdict! {
            "name": "cybuild/path",
            "type": VariantType::STRING,
            "hint": PropertyHint::NONE,
            "hint_string": "",
        });
        ps.add_property_info(&vdict! {
            "name": "cybuild/outdir",
            "type": VariantType::STRING,
            "hint": PropertyHint::NONE,
            "hint_string": "",
        });
        ps.add_property_info(&vdict! {
            "name": "cybuild/project_target_name",
            "type": VariantType::STRING,
            "hint": PropertyHint::NONE,
            "hint_string": "",
        });
        ps.add_property_info(&vdict! {
            "name": "cybuild/dependencies_only",
            "type": VariantType::BOOL,
            "hint": PropertyHint::NONE,
            "hint_string": "",
        });
        ps.add_property_info(&vdict! {
            "name": "cybuild/executable_path",
            "type": VariantType::STRING,
            "hint": PropertyHint::NONE,
            "hint_string": "",
        });

        // Mark settings as basic for display in Project Settings
        ps.set_as_basic("cybuild/path", true);
        ps.set_as_basic("cybuild/outdir", true);
        ps.set_as_basic("cybuild/project_target_name", true);
    }

    /// Retrieve the project target name from settings as a `String`.
    fn get_project_target_name() -> String {
        Self::get_setting_as_string("cybuild/project_target_name")
    }

    /// Helper to retrieve a setting as a String.
    fn get_setting_as_string(key: &str) -> String {
        ProjectSettings::singleton().get_setting(key).to_string()
    }
}
