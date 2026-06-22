# Builds the Godot voxel-view spike scene in code (more robust than hand-written
# .tscn resource refs for the godot_voxel classes).
#
# The whole tree is assembled here: the Rust bridge node, a VoxelTerrain whose
# generator reads from that bridge, a colour palette mapping vivarium materials
# to colours, a camera with a VoxelViewer so terrain streams around it, and a
# light. After a few seconds it saves a screenshot and quits, so the render can
# be inspected without a human watching (see SHOT_PATH in the output).
extends Node3D

# vivarium material id -> colour. Index must match vivarium_core::voxel::Voxel.
# Air (0) is transparent so the cubes mesher leaves it empty.
const PALETTE := {
	0: Color(0, 0, 0, 0),          # AIR
	1: Color(0.50, 0.50, 0.52),    # STONE
	2: Color(0.55, 0.40, 0.25),    # DIRT
	3: Color(0.32, 0.70, 0.27),    # GRASS
	4: Color(0.20, 0.45, 0.85),    # WATER
}

var world: Object  # VivariumWorld (Rust bridge)

func _ready() -> void:
	world = ClassDB.instantiate("VivariumWorld")
	world.name = "VivariumWorld"
	add_child(world)

	var palette := VoxelColorPalette.new()
	for idx in PALETTE:
		palette.set_color(idx, PALETTE[idx])

	var mesher := VoxelMesherCubes.new()
	mesher.color_mode = VoxelMesherCubes.COLOR_MESHER_PALETTE
	mesher.palette = palette

	var gen = load("res://generator.gd").new()
	gen.world = world

	var terrain := VoxelTerrain.new()
	terrain.mesher = mesher
	terrain.generator = gen
	var mat := StandardMaterial3D.new()
	mat.vertex_color_use_as_albedo = true
	terrain.material_override = mat
	add_child(terrain)

	# Camera looking at the origin column from above and to the side.
	var sh: int = world.surface_height(0, 0)
	var look := Vector3(0, sh, 0)
	var cam := Camera3D.new()
	cam.position = Vector3(50, sh + 35, 50)
	add_child(cam)               # must be in-tree before look_at()
	cam.look_at(look, Vector3.UP)

	# A VoxelViewer is what tells the terrain where to stream chunks in.
	var viewer := VoxelViewer.new()
	viewer.view_distance = 256
	cam.add_child(viewer)

	var sun := DirectionalLight3D.new()
	sun.rotation_degrees = Vector3(-55, -45, 0)
	add_child(sun)

	var we := WorldEnvironment.new()
	var env := Environment.new()
	env.background_mode = Environment.BG_COLOR
	env.background_color = Color(0.53, 0.72, 0.95)
	env.ambient_light_source = Environment.AMBIENT_SOURCE_COLOR
	env.ambient_light_color = Color(1, 1, 1)
	env.ambient_light_energy = 0.45
	we.environment = env
	add_child(we)

	# Give the streaming threads a few seconds, then capture and quit.
	get_tree().create_timer(4.0).timeout.connect(_capture_and_quit)

func _capture_and_quit() -> void:
	var img := get_viewport().get_texture().get_image()
	var path := "user://terrain_shot.png"
	img.save_png(path)
	print("[vivarium] SHOT_PATH=", ProjectSettings.globalize_path(path))
	get_tree().quit()
