@tool
extends EditorPlugin


func _enable_plugin() -> void:
	print("Cybuild plugin for Godot is enabled.")


func _disable_plugin() -> void:
	# Remove autoloads here.
	pass


func _enter_tree() -> void:
	# Initialization of the plugin goes here.
	var result = OS.execute("cybuild", ["--help"])
	if result != 0:
		push_warning("cybuild cannot called from cmdline!")
	
	if not ProjectSettings.has_setting("cybuild/path"):
		ProjectSettings.set_setting("cybuild/path", "")
	
	if not ProjectSettings.has_setting("cybuild/targets"):
		ProjectSettings.set_setting("cybuild/targets", PackedStringArray([]))
		
	if not ProjectSettings.has_setting("cybuild/output"):
		ProjectSettings.set_setting("cybuild/output", "")

func _exit_tree() -> void:
	# Clean-up of the plugin goes here.
	pass

func _build() -> bool:
	return true
