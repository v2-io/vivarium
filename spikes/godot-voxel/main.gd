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
# LOD config for VoxelLodTerrain. lod_distance is how far full-detail LOD0
# reaches from the viewer; each coarser level reaches twice as far again. With
# LOD the view distance can be large because far terrain is cheap (coarse).
const LOD_COUNT := 6
const LOD_DISTANCE := 64.0
# Max view distance in voxels (physical reach = this / detail). 2048 with LOD is
# cheap where 2048 without LOD hung the engine. Override with VIVARIUM_VIEWCAP.
const VIEW_DISTANCE_DEFAULT := 2048

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

	_view_distance = VIEW_DISTANCE_DEFAULT
	var cap_env := OS.get_environment("VIVARIUM_VIEWCAP")
	if cap_env != "":
		_view_distance = int(cap_env)

	# VoxelLodTerrain: an octree that stores/streams voxels at multiple levels of
	# detail, so distant terrain is meshed coarse and we can see *much* farther
	# for the same cost. This decouples view resolution from intrinsic resolution
	# — the engine asks our generator for coarse blocks far away (see the `lod`
	# arg threaded into generate_block). NOTE: still no node scaling (a
	# non-identity transform breaks streaming); 1 voxel == 1 Godot unit.
	terrain = VoxelLodTerrain.new()
	terrain.mesher = mesher
	terrain.generator = gen
	terrain.lod_count = LOD_COUNT
	terrain.lod_distance = LOD_DISTANCE       # how far LOD0 (full detail) reaches
	terrain.view_distance = _view_distance
	add_child(terrain)
	# VoxelLodTerrain rejects material_override (Node3D's); it manages its own
	# mesh instances. It takes a single terrain material via set_material(). For
	# cubes-palette the vertex colours are baked into the mesh, so the material
	# just needs to use them as albedo.
	var mat := StandardMaterial3D.new()
	mat.vertex_color_use_as_albedo = true
	terrain.set_material(mat)
	_trace("terrain added")

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

	# A VoxelViewer tells the terrain where to stream chunks (and at which LOD,
	# by distance). view_distance matches the terrain's.
	var viewer := VoxelViewer.new()
	viewer.view_distance = _view_distance
	cam.add_child(viewer)

	# Overcast: soft, dim, slightly cool key light so nothing casts hard shadows.
	var sun := DirectionalLight3D.new()
	sun.rotation_degrees = Vector3(-55, -45, 0)
	sun.light_energy = 0.6
	sun.light_color = Color(0.95, 0.96, 1.0)
	add_child(sun)

	var we := WorldEnvironment.new()
	var env := Environment.new()
	# Bright, desaturated overcast sky. Distance haze fades terrain *up* toward
	# this light grey (the natural overcast look) rather than down into gloom, so
	# lit terrain dissolves into it seamlessly — no horizon. Linear tonemapping
	# (below) makes the colour render exactly as set, not darkened.
	var sky_color := Color(0.80, 0.82, 0.84)
	env.background_mode = Environment.BG_COLOR
	env.background_color = sky_color
	# Render colours as authored (no filmic/AgX darkening of the flat sky).
	env.tonemap_mode = Environment.TONE_MAPPER_LINEAR
	env.ambient_light_source = Environment.AMBIENT_SOURCE_COLOR
	env.ambient_light_color = Color(0.86, 0.88, 0.92)
	env.ambient_light_energy = 0.7

	# Depth fog: opacity rises linearly with distance (FOG_MODE_DEPTH, curve 1.0),
	# tinted to the overcast sky. Pulled in *close* — fully grey by ~40 units of
	# detail — so only the hill in front of you reads clearly and everything
	# beyond dissolves into the murk (this also hides the LOD transition).
	env.fog_enabled = true
	env.fog_mode = Environment.FOG_MODE_DEPTH
	env.fog_light_color = sky_color
	# Fog the sky too, fully — so the whole sky becomes the same fog colour the
	# distant terrain dissolves into. Both end up one uniform grey: no horizon.
	env.fog_sky_affect = 1.0
	env.fog_depth_begin = float(14 * detail)
	env.fog_depth_end = float(90 * detail)   # fully sky-coloured by here -> no horizon
	env.fog_depth_curve = 1.0         # linear: opacity ∝ distance
	we.environment = env
	add_child(we)

	# Interactive by default. In an automated run (VIVARIUM_AUTOSHOT set) give the
	# streaming threads a few seconds, then screenshot and quit — used to verify
	# renders without a human watching.
	_trace("scene built, viewer.view_distance=%d" % viewer.view_distance)
	if OS.get_environment("VIVARIUM_AUTOSHOT") != "":
		# LOD over a large view distance has a lot to stream initially; give the
		# threads time before the vista shot.
		get_tree().create_timer(10.0 + detail).timeout.connect(_capture_and_quit)
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
	# Low, near-ground camera looking forward, so the near terrain emerges from
	# the fog and everything beyond dissolves into the overcast murk. All in
	# voxel/world units (1:1).
	var sh: int = world.surface_height(0, 0)
	cam.position = Vector3(0, sh + 5 * detail, -26 * detail)
	cam.look_at(Vector3(0, sh + 2 * detail, 60 * detail), Vector3.UP)
	_carve_test()
	_trace("carve done; fps=%.1f, view_distance=%d voxels (%d physical), detail=%d"
		% [Engine.get_frames_per_second(), _view_distance, _view_distance / detail, detail])
	# Let the remesh settle (and FPS recover) before grabbing the frame.
	await get_tree().create_timer(2.0).timeout
	_trace("capturing image; steady fps=%.1f" % Engine.get_frames_per_second())
	var img := get_viewport().get_texture().get_image()
	var path := "user://terrain_shot.png"
	img.save_png(path)
	print("[vivarium] SHOT_PATH=", ProjectSettings.globalize_path(path))
	get_tree().quit()
