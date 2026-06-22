# Builds the Godot voxel-view spike scene in code (more robust than hand-written
# .tscn resource refs for the godot_voxel classes).
#
# The whole tree is assembled here: the Rust bridge node, a VoxelTerrain whose
# generator reads from that bridge, a colour palette mapping vivarium materials
# to colours, a camera with a VoxelViewer so terrain streams around it, and a
# light. After a few seconds it saves a screenshot and quits, so the render can
# be inspected without a human watching (see SHOT_PATH in the output).
#
# Resolution handling: core works in voxels; the world's `detail` is voxels per
# world unit. To keep the world the same *physical* size at any resolution, the
# terrain node is scaled by 1/detail, and core's voxel coordinates are converted
# to physical ones (÷detail) wherever the camera is placed.
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

# How far (in *physical* world units) terrain streams around the camera. Voxel
# view distance is this × detail, so detail is what makes this expensive — the
# whole point of the resolution sweep.
const PHYSICAL_VIEW := 200.0

# Hard cap on voxel view distance. FINDING: VoxelMesherCubes terrain has no LOD,
# so a full physical view at high detail (e.g. 110×8 = 880 voxels) asks for an
# astronomical chunk count and the engine hangs. Capping trades physical view
# radius for resolution — at detail 8 you get a detailed ~32-unit bubble rather
# than a fine world to the horizon. The proper fix is multi-fidelity / LOD
# (DESIGN.md) or smooth/Transvoxel meshing; noted for the findings.
# 512 voxels = 128 physical units at detail 4; held 120 FPS (vsync-capped) in
# testing, so it's a comfortable default. Override with VIVARIUM_VIEWCAP.
const VOXEL_VIEW_CAP_DEFAULT := 512

var world: Object     # VivariumWorld (Rust bridge)
var terrain: Object   # VoxelTerrain (kept for the automated dig self-test)
var cam: Camera3D     # the player camera (repositioned for the dig close-up)
var detail: int = 1
var _view_distance: int = 0

var _trace_file: FileAccess

# Flushed-per-line trace to user://trace.log, so progress is visible even when
# the process is killed (stdout is block-buffered off a tty and lost on SIGKILL).
func _trace(msg: String) -> void:
	if _trace_file == null:
		_trace_file = FileAccess.open("user://trace.log", FileAccess.WRITE)
	_trace_file.store_line("%d ms  %s" % [Time.get_ticks_msec(), msg])
	_trace_file.flush()

func _ready() -> void:
	_trace("ready: start")
	world = ClassDB.instantiate("VivariumWorld")
	world.name = "VivariumWorld"
	add_child(world)
	detail = world.voxels_per_unit()
	_trace("world added, detail=%d" % detail)

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
	# NOTE: we do NOT scale the VoxelTerrain node. FINDING: a non-identity
	# transform on VoxelTerrain breaks its streaming/rendering (terrain never
	# appears). So 1 voxel == 1 Godot unit at every resolution; a finer world is
	# simply a physically larger world, and the camera views a local bubble of it
	# (granularity is relative to the bubble, which is what we want to feel).
	var mat := StandardMaterial3D.new()
	mat.vertex_color_use_as_albedo = true
	terrain.material_override = mat
	add_child(terrain)
	_trace("terrain added (scale=%s)" % str(terrain.scale))

	# First-person fly camera (player.gd), in voxel/world coordinates (1:1).
	var sh: int = world.surface_height(0, 0)
	cam = Camera3D.new()
	cam.set_script(load("res://player.gd"))
	cam.world = world
	cam.terrain = terrain
	cam.position = Vector3(60, sh + 45, 60)
	add_child(cam)                               # in-tree before look_at()
	cam.look_at(Vector3(0, sh, 0), Vector3.UP)
	cam.resync()                                 # carry that heading into yaw/pitch

	# A VoxelViewer tells the terrain where to stream chunks. view_distance is in
	# voxels, so scale by detail to hold a constant physical view radius.
	var cap := VOXEL_VIEW_CAP_DEFAULT
	var cap_env := OS.get_environment("VIVARIUM_VIEWCAP")
	if cap_env != "":
		cap = int(cap_env)
	_view_distance = mini(int(PHYSICAL_VIEW * detail), cap)
	var viewer := VoxelViewer.new()
	viewer.view_distance = _view_distance
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
	_trace("scene built, viewer.view_distance=%d" % viewer.view_distance)
	if OS.get_environment("VIVARIUM_AUTOSHOT") != "":
		# Finer worlds have far more voxels to mesh; give the threads more time.
		get_tree().create_timer(4.0 + detail).timeout.connect(_capture_and_quit)
	else:
		print("[vivarium] interactive (detail %d): WASD move, mouse look, " % detail,
			"Space/Shift up/down, Ctrl=fast, LMB dig, RMB place, Esc frees mouse")

# Carve a crater at the origin through the same path the player uses (core edit
# + VoxelTool remesh), so the automated screenshot proves the dig path end to
# end, not just static generation. Sized in physical units so it stays visible
# at any resolution.
func _carve_test() -> void:
	var vt = terrain.get_voxel_tool()
	vt.channel = VoxelBuffer.CHANNEL_COLOR
	var sh: int = world.surface_height(0, 0)
	print("[vivarium] before dig: voxel(0,%d,0)=%d" % [sh, world.voxel_at(0, sh, 0)])
	var r := 2 * detail        # ~2 physical units radius
	var depth := 2 * detail
	for dx in range(-r, r + 1):
		for dz in range(-r, r + 1):
			for dy in range(0, depth):
				var p := Vector3i(dx, sh - dy, dz)
				world.dig(p.x, p.y, p.z)
				vt.set_voxel(p, 0)
	print("[vivarium] after dig:  voxel(0,%d,0)=%d (0 = air; core edit persisted)"
		% [sh, world.voxel_at(0, sh, 0)])

func _capture_and_quit() -> void:
	_trace("capture: start")
	# Move in close and overhead so the crater (radius/depth ~2·detail) is
	# unmistakable in the shot. All in voxel/world units (1:1).
	var sh: int = world.surface_height(0, 0)
	cam.position = Vector3(2 * detail + 8, sh + 3 * detail + 6, 2 * detail + 8)
	cam.look_at(Vector3(0, sh - 2 * detail, 0), Vector3.UP)
	_carve_test()
	_trace("carve done; fps=%.1f, view_distance=%d voxels (%d physical), detail=%d"
		% [Engine.get_frames_per_second(), _view_distance, _view_distance / detail, detail])
	# Let the remesh settle before grabbing the frame.
	for _i in 4:
		await get_tree().process_frame
	_trace("capturing image")
	var img := get_viewport().get_texture().get_image()
	var path := "user://terrain_shot.png"
	img.save_png(path)
	print("[vivarium] SHOT_PATH=", ProjectSettings.globalize_path(path))
	get_tree().quit()
