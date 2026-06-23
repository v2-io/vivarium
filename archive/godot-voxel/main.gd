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
# Muted, earthy palette — desaturated so the terrain reads as natural rock/soil
# rather than cartoon-bright, and so the grass/dirt contour banding is subtler.
const PALETTE := {
	0: Color(0, 0, 0, 0),          # AIR
	1: Color(0.52, 0.51, 0.50),    # STONE
	2: Color(0.46, 0.39, 0.31),    # DIRT
	3: Color(0.44, 0.52, 0.36),    # GRASS (sage/olive)
	4: Color(0.40, 0.52, 0.60),    # WATER
}

# How far (in *physical* world units) terrain streams around the camera. Voxel
# view distance is this × detail, so detail is what makes this expensive — the
# whole point of the resolution sweep.
# LOD config for VoxelLodTerrain. lod_distance is how far full-detail LOD0
# reaches from the viewer; each coarser level reaches twice as far again. With
# LOD the view distance can be large because far terrain is cheap (coarse).
# Geology-scale LOD, pushed for *maximum view distance* (Joseph 2026-06-23: first
# see the real landscape shape across the whole landmass; FPS/load can be dismal).
# The eroded world is ~24,000 voxels (12 km) across. VoxelLodTerrain reach is
# roughly lod_distance × 2^(lod_count-1); the coarsest voxel is 2^(lod_count-1)
# voxels, so for a given reach a *larger* lod_distance with *fewer* levels keeps
# distant terrain finer (less of the giant-block stepping). The open question this
# revival tests: how far does Godot's native octree reach before it falls over —
# where bevy_voxel_world needed a separate backdrop mesh (spawn_far_terrain).
#
# All three are env-overridable so the reach can be swept without editing the file
# (GDScript reloads instantly anyway, but env keeps runs reproducible/logged):
#   VIVARIUM_VIEWCAP    view distance in voxels   (default 32768 → spans + margin)
#   VIVARIUM_LOD_DIST   LOD0 full-detail reach    (default 1024)
#   VIVARIUM_LOD_COUNT  octree levels             (default 7)
const LOD_COUNT_DEFAULT := 7
const LOD_DISTANCE_DEFAULT := 1024.0
const VIEW_DISTANCE_DEFAULT := 32768

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
	# Fixed window/viewport size so screenshots are consistent and comparable
	# with the Bevy spike (the project.godot setting wasn't taking reliably).
	get_window().size = Vector2i(1152, 648)

	# Mesher throughput: godot_voxel meshes/generates on a worker pool, distance-
	# prioritized. Default is ~half the cores (8 of 16 here), which leaves the queue
	# draining slowly when you outrun it. Give it most of the box (reserve ~2 for the
	# main + render threads). Tune with VIVARIUM_THREADS. Must be set before the
	# terrain starts streaming.
	var engine = Engine.get_singleton("VoxelEngine")
	if engine != null:
		var threads := maxi(2, OS.get_processor_count() - 2)
		var threads_env := OS.get_environment("VIVARIUM_THREADS")
		if threads_env != "":
			threads = int(threads_env)
		engine.set_thread_count(threads)
		_trace("voxel mesher threads=%d (of %d cores)" % [threads, OS.get_processor_count()])
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

	# Reach/LOD knobs — env overrides the defaults so the field of view can be
	# swept run-to-run without editing this file.
	_view_distance = VIEW_DISTANCE_DEFAULT
	var cap_env := OS.get_environment("VIVARIUM_VIEWCAP")
	if cap_env != "":
		_view_distance = int(cap_env)
	var lod_distance := LOD_DISTANCE_DEFAULT
	var lod_dist_env := OS.get_environment("VIVARIUM_LOD_DIST")
	if lod_dist_env != "":
		lod_distance = float(lod_dist_env)
	var lod_count := LOD_COUNT_DEFAULT
	var lod_count_env := OS.get_environment("VIVARIUM_LOD_COUNT")
	if lod_count_env != "":
		lod_count = int(lod_count_env)

	# VoxelLodTerrain: an octree that stores/streams voxels at multiple levels of
	# detail, so distant terrain is meshed coarse and we can see *much* farther
	# for the same cost. This decouples view resolution from intrinsic resolution
	# — the engine asks our generator for coarse blocks far away (see the `lod`
	# arg threaded into generate_block). NOTE: still no node scaling (a
	# non-identity transform breaks streaming); 1 voxel == 1 Godot unit.
	terrain = VoxelLodTerrain.new()
	terrain.mesher = mesher
	terrain.generator = gen
	terrain.lod_count = lod_count
	terrain.lod_distance = lod_distance       # how far LOD0 (full detail) reaches
	terrain.view_distance = _view_distance

	# --- LOD responsiveness under fast motion (Joseph 2026-06-23) -------------
	# Slow-and-steady prioritizes well; "whipping around" leaves the thing in
	# front macro while something distant refines. Two causes, two levers:
	#
	#  1. The octree LOD-subdivision *decision* runs on the main thread by default
	#     (threaded_update_enabled=false). At single-digit FPS the main thread is
	#     the bottleneck, so the "this near chunk should be finer now" verdict
	#     lands several frames late. Move it to a worker thread so subdivision
	#     keeps pace with the viewer. Off with VIVARIUM_THREADED_UPDATE=0.
	terrain.threaded_update_enabled = OS.get_environment("VIVARIUM_THREADED_UPDATE") != "0"
	#  2. Soft LOD cross-fade so a late refinement dissolves in rather than popping
	#     (purely cosmetic, but it's the visible half of the complaint).
	var fade := 0.25
	var fade_env := OS.get_environment("VIVARIUM_LOD_FADE")
	if fade_env != "":
		fade = float(fade_env)
	terrain.lod_fade_duration = fade
	add_child(terrain)
	# VoxelLodTerrain rejects material_override (Node3D's); it manages its own
	# mesh instances. It takes a single terrain material via set_material(). For
	# cubes-palette the vertex colours are baked into the mesh, so the material
	# just needs to use them as albedo.
	var mat := StandardMaterial3D.new()
	mat.vertex_color_use_as_albedo = true
	terrain.set_material(mat)

	# VIVARIUM_DEBUG_LOD=1: draw the octree LOD boxes + flash mesh updates, so the
	# "near stays macro while distant refines" behaviour is *visible* while flying
	# (the thing that's otherwise hard to test). Colour = LOD level; a box that
	# flashes is a mesh being (re)built — watch whether near boxes refine before
	# far ones when you whip the camera around.
	if OS.get_environment("VIVARIUM_DEBUG_LOD") == "1":
		terrain.debug_draw_enabled = true
		terrain.debug_draw_octree_bounds = true
		terrain.debug_draw_mesh_updates = true
	_trace("terrain added")

	# First-person fly camera (player.gd), in voxel/world coordinates (1:1).
	# Geology-scale framing mirrors the Bevy spike (spikes/bevy-voxel setup): stand
	# just above the ground at the region centre and gaze *outward and slightly
	# down* across the landmass toward the far massifs — not down at one's feet.
	var sh: int = world.surface_height(0, 0)
	cam = Camera3D.new()
	cam.set_script(load("res://player.gd"))
	cam.world = world
	cam.terrain = terrain
	cam.far = float(_view_distance) + 8192.0   # don't clip the far terrain
	cam.position = Vector3(0, sh + 12, 0)        # ~6 m eye height at detail 2
	add_child(cam)                               # in-tree before look_at()
	cam.look_at(Vector3(800, sh - 60, 300), Vector3.UP)   # outward across the terrain
	cam.resync()                                 # carry that heading into yaw/pitch

	# A VoxelViewer tells the terrain where to stream chunks (and at which LOD,
	# by distance). view_distance matches the terrain's.
	var viewer := VoxelViewer.new()
	viewer.view_distance = _view_distance
	# We fly *over* a surface, so the full-height column (ratio 1.0 = mesh chunks
	# 32 k voxels above and below) is almost all wasted budget that should go to
	# near-surface refinement. Squash the vertical reach. Tune with VIVARIUM_VRATIO;
	# raise it if looking down deep shafts/oceans ever starves. (8.2 km world, so
	# 0.3 × 32 k ≈ 4.9 km vertical reach — ample for surface flight.)
	var vratio := 0.3
	var vratio_env := OS.get_environment("VIVARIUM_VRATIO")
	if vratio_env != "":
		vratio = float(vratio_env)
	viewer.view_distance_vertical_ratio = vratio
	cam.add_child(viewer)

	# Soft overcast key light. Enough energy that block faces shade by their
	# orientation (top vs side vs angled) — the face-to-face contrast that gives
	# voxel terrain form — but no hard shadows, keeping the diffuse overcast mood.
	var sun := DirectionalLight3D.new()
	sun.rotation_degrees = Vector3(-50, -40, 0)
	sun.light_energy = 1.4
	sun.light_color = Color(0.96, 0.97, 1.0)
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
	env.ambient_light_energy = 0.45

	# Depth fog: opacity rises linearly with distance (FOG_MODE_DEPTH, curve 1.0),
	# tinted to the overcast sky. Pulled in *close* — fully grey by ~40 units of
	# detail — so only the hill in front of you reads clearly and everything
	# beyond dissolves into the murk (this also hides the LOD transition).
	# Fog drowns the whole-landmass vista in grey (it's tuned to dissolve the far
	# edge, but at 16 km everything is "far"). Toggle off with VIVARIUM_FOG=0 when
	# the goal is to read the real terrain shape rather than an atmospheric mood.
	env.fog_enabled = OS.get_environment("VIVARIUM_FOG") != "0"
	env.fog_mode = Environment.FOG_MODE_DEPTH
	env.fog_light_color = sky_color
	# Fog the sky too, fully — so the whole sky becomes the same fog colour the
	# distant terrain dissolves into. Both end up one uniform grey: no horizon.
	env.fog_sky_affect = 1.0
	# Geology scale: the toy-world fog (grey by ~90 units) hid everything in a
	# 12 km landmass. Pull it out to atmospheric distance so near valleys read
	# crisp, far massifs haze blue-grey, and the LOD/streaming edge dissolves
	# rather than ending in a hard line. In voxels (1:1 with metres at detail 2).
	env.fog_depth_begin = 1500.0
	env.fog_depth_end = float(_view_distance)   # dissolve by the view edge
	env.fog_depth_curve = 1.0         # linear: opacity ∝ distance

	# Screen-space ambient occlusion: darkens the creases between blocks, which
	# is the depth cue that makes voxel terrain read as sculpted rather than flat
	# (under overcast/diffuse light, AO does the form-giving a sun would). Radius
	# is in world units (= voxels), tuned to block-scale crevices.
	env.ssao_enabled = true
	env.ssao_radius = 2.0
	env.ssao_intensity = 5.0
	env.ssao_power = 1.5
	env.ssao_detail = 0.5
	we.environment = env
	add_child(we)

	# Interactive by default. In an automated run (VIVARIUM_AUTOSHOT set) give the
	# streaming threads a few seconds, then screenshot and quit — used to verify
	# renders without a human watching.
	_trace("scene built, viewer.view_distance=%d" % viewer.view_distance)
	if OS.get_environment("VIVARIUM_AUTOSHOT") != "":
		# A whole-landmass view distance has a *lot* to stream initially; give the
		# mesher threads generous time before the vista shot. Env-tunable so a
		# bigger reach can wait longer.
		var settle := 25.0
		var settle_env := OS.get_environment("VIVARIUM_SETTLE")
		if settle_env != "":
			settle = float(settle_env)
		get_tree().create_timer(settle).timeout.connect(_capture_and_quit)
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
	# Geology vista: a high oblique aerial framing the *whole* landmass so the
	# km-scale relief — ridge-and-valley drainage, peaks, coast — reads in one
	# frame. Pulled far back/up over one corner, looking across the region centre.
	# (The interactive camera stays at eye level; this is just the verification
	# framing.) Env-tunable so the vista can be swept. All in voxel units (1:1).
	var sh: int = world.surface_height(0, 0)
	var cam_back := 14000.0
	var back_env := OS.get_environment("VIVARIUM_CAM_BACK")
	if back_env != "":
		cam_back = float(back_env)
	cam.position = Vector3(-cam_back, sh + cam_back * 0.45, -cam_back)
	cam.look_at(Vector3(0, float(sh), 0), Vector3.UP)
	_trace("vista framed; fps=%.1f, view_distance=%d voxels (%d physical), detail=%d"
		% [Engine.get_frames_per_second(), _view_distance, _view_distance / detail, detail])
	# Let the remesh settle (and FPS recover) before grabbing the frame.
	await get_tree().create_timer(2.0).timeout
	_trace("capturing image; steady fps=%.1f" % Engine.get_frames_per_second())
	var img := get_viewport().get_texture().get_image()
	var path := "user://terrain_shot.png"
	img.save_png(path)
	print("[vivarium] SHOT_PATH=", ProjectSettings.globalize_path(path))
	get_tree().quit()
