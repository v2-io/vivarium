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

var world: Object     # VivariumWorld (Rust bridge)
var terrain: Object   # VoxelTerrain (kept for the automated dig self-test)
var cam: Camera3D     # the player camera (repositioned for the dig close-up)

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

	terrain = VoxelTerrain.new()
	terrain.mesher = mesher
	terrain.generator = gen
	var mat := StandardMaterial3D.new()
	mat.vertex_color_use_as_albedo = true
	terrain.material_override = mat
	add_child(terrain)

	# First-person fly camera (player.gd), aimed at the origin column to start.
	var sh: int = world.surface_height(0, 0)
	cam = Camera3D.new()
	cam.set_script(load("res://player.gd"))
	cam.world = world
	cam.terrain = terrain
	cam.position = Vector3(50, sh + 35, 50)
	add_child(cam)                               # in-tree before look_at()
	cam.look_at(Vector3(0, sh, 0), Vector3.UP)
	cam.resync()                                 # carry that heading into yaw/pitch

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

	# Interactive by default. In an automated run (VIVARIUM_AUTOSHOT set) give the
	# streaming threads a few seconds, then screenshot and quit — used to verify
	# renders without a human watching.
	if OS.get_environment("VIVARIUM_AUTOSHOT") != "":
		get_tree().create_timer(4.0).timeout.connect(_capture_and_quit)
	else:
		print("[vivarium] interactive: WASD move, mouse look, Space/Shift up/down, ",
			"Ctrl=fast, LMB dig, RMB place, Esc frees mouse")

# Carve a small crater at the origin through the same path the player uses
# (core edit + VoxelTool remesh), so the automated screenshot proves the dig
# path end to end, not just static generation.
func _carve_test() -> void:
	var vt = terrain.get_voxel_tool()
	vt.channel = VoxelBuffer.CHANNEL_COLOR
	var sh: int = world.surface_height(0, 0)
	# Prove the core edit persisted (not just the visual).
	print("[vivarium] before dig: voxel(0,%d,0)=%d" % [sh, world.voxel_at(0, sh, 0)])
	for dx in range(-3, 4):
		for dz in range(-3, 4):
			for dy in range(0, 5):
				var p := Vector3i(dx, sh - dy, dz)
				world.dig(p.x, p.y, p.z)
				vt.set_voxel(p, 0)
	print("[vivarium] after dig:  voxel(0,%d,0)=%d (0 = air; core edit persisted)"
		% [sh, world.voxel_at(0, sh, 0)])

func _capture_and_quit() -> void:
	# Move in close and overhead so the crater is unmistakable in the shot.
	var sh: int = world.surface_height(0, 0)
	cam.position = Vector3(9, sh + 11, 9)
	cam.look_at(Vector3(0, sh - 2, 0), Vector3.UP)
	_carve_test()
	# Let the remesh settle before grabbing the frame.
	await get_tree().process_frame
	await get_tree().process_frame
	await get_tree().process_frame
	var img := get_viewport().get_texture().get_image()
	var path := "user://terrain_shot.png"
	img.save_png(path)
	print("[vivarium] SHOT_PATH=", ProjectSettings.globalize_path(path))
	get_tree().quit()
