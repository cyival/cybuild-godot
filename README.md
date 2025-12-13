# Cybuild for Godot

A Godot plugin for in-editor build support using [Cybuild](https://github.com/cyival/Cyival.Build).

---

## Installation

1. **Install Cybuild**  
   Ensure you have [Cybuild](https://github.com/cyival/Cyival.Build) installed and available in your system `PATH` (accessible from the command line).  
   > **Note:** This plugin requires Cybuild version **0.2 or newer**.

2. **Install the Plugin**  
   - Download the latest release from the [Releases](https://github.com/cyival/cybuild-godot/releases) page.
   - Extract the plugin into your Godot project directory.
   - The plugin will be automatically enabled when you open your project in the Godot editor.

---

## Configuration

After installing both the plugin and Cybuild, configure the following settings in **Project > Project Settings**:

| Setting Key                      | Type   | Description                                                                 |
|----------------------------------|--------|-----------------------------------------------------------------------------|
| `cybuild/path`                   | String | Path to your manifest. Defaults to `res://` (your project folder) if unset.|
| `cybuild/outdir`                 | String | Directory where build artifacts will be placed.                             |
| `cybuild/project_target_name`    | String | **REQUIRED:** Your project ID in the manifest.                              |

- **Project Target Name** (`cybuild/project_target_name`):  
  This is the target ID in your manifest. It is required for processing artifacts and packages.

---

## Usage

- Once the settings are configured, Cybuild will automatically start the build process when you click the **Build** button in Godot.
- A button is also available in the **Tool** menu, allowing you to run the default target (another Godot project) instead of the current one.

> **Tip:**  
> Create a `.gdignore` file in your output directory (default: `res://artifacts`) to prevent Godot from loading build artifacts.

---

## Building the Plugin

**Requirements:**
- [Rust](https://www.rust-lang.org/) 1.90 or newer

---

## License

This plugin is licensed under the [MIT License](LICENSE).
