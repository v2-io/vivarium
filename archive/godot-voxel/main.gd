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
# LOD config for VoxelLodTerrain. The octree fills a sphere of radius
# `view_distance` around the viewer; `lod_distance` is the radius kept at FULL
# detail (LOD0), and each coarser level roughly doubles the reach again.
#
# EMPIRICAL (2026-06-23): these values — lod_distance 1024, lod_count 7 — are the
# best-performing config we found by *flying it*, and they are restored here after
# two attempts to "improve" them regressed. The story is worth keeping because the
# obvious reasoning was wrong:
#   - zylann's docs say extend reach with more LOD *levels*, keeping lod_distance
#     modest. Followed literally (lod_distance 128, lod_count 9) it felt and looked
#     WORSE: more LOD levels → more total blocks in the no-occlusion sphere → far
#     longer to develop, and it spent itself rendering occluded/behind-camera
#     terrain. Observation overruled the doc-derived model.
#   - the big full-detail shell (1024) costs more in theory, but in practice it
#     develops the near field fast and stays there, which is what reads as "good".
# So: trust the felt result. If you re-tune, change ONE knob and fly it; do not
# reason your way to a new default. The fast-camera "giant near blocks" artifact
# is a known, accepted cost of this config — chasing it is what caused the
# regressions (see git log around this date).
#
# Caveat this revival exposed (sourced, not a tuning choice): the octree loads
# SPHERICALLY by distance with NO frustum/occlusion culling — it meshes chunks
# behind you and behind mountains as eagerly as the one in view. Smaller
# view_distance is the only lever for that from the public API. A real limitation
# to weigh in the Bevy-vs-Godot decision.
#
# MEASURED (2026-06-23, bench/ campaign): view_distance is the DOMINANT cost knob,
# by far — because the octree fills its whole sphere with no occlusion culling, the
# cost grows with the sphere volume. The benchmark cost curve (3 runs each, this
# machine):
#     view 32768 (16 km): ~13 fps, 442k data blocks   ← the old default; painful
#     view 16384 ( 8 km): ~106 fps, 140k data blocks   ← 8× faster
#     view  8192 ( 4 km): ~145 fps,  89k data blocks   ← reach too short, lands clip
# 32768 was reaching ~20 km — far past the ±12 000-voxel continent edge, into empty
# ocean — so it paid 8× the framerate to render *nothing*. 16384 still spans the
# whole landmass from a central vantage (16384 > the 12000-voxel half-extent) AND
# reads richly in the screenshots, so it is the default: full view of the land at a
# fraction of the cost. Push VIVARIUM_VIEWCAP=32768 for the extreme edge-to-edge
# vista if you accept ~13 fps. (Deltas clear the ±8% fps / ±7% data-block noise
# floor by ~700%, so this is real, not jitter — see bench/README.md.)
#
# All env-overridable so the reach can be swept without editing the file:
#   VIVARIUM_VIEWCAP    view distance in voxels   (default 16384 → spans the land, fast)
#   VIVARIUM_LOD_DIST   LOD0 full-detail radius   (default 1024)
#   VIVARIUM_LOD_COUNT  octree levels             (default 7)
const LOD_COUNT_DEFAULT := 7
const LOD_DISTANCE_DEFAULT := 1024.0
const VIEW_DISTANCE_DEFAULT := 16384

var world: Object     # VivariumWorld (Rust bridge)
var terrain: Object   # VoxelTerrain (kept for the automated dig self-test)
var cam: Camera3D     # the player camera (repositioned for the dig close-up)
var detail: int = 1
var _view_distance: int = 0

var _trace_file: FileAccess

# --- Telemetry & repeatable benchmark (2026-06-23) -------------------------------
# We spent a long stretch judging LOD/streaming by feel and could not tell a
# regression from "noticing an old artifact" from "an actual improvement". The cure
# is measurement: a live readout while flying, and a *scripted, identical* flight so
# two configs produce comparable numbers instead of impressions. Engaged by env:
#   VIVARIUM_TELEMETRY=1  on-screen live readout (also on in bench mode)
#   VIVARIUM_BENCH=1      fly a fixed deterministic path, log CSV, quit
#   VIVARIUM_BENCH_SECS   bench duration (default 30)
# CSV lands at user://telemetry.csv (path printed at exit). One row per sample.
const TELEMETRY_HZ := 2.0           # samples/sec (CSV + label refresh)
var _telemetry_on := false
var _bench := false
var _bench_secs := 30.0
var _bench_t := 0.0                 # elapsed *path* time (sum of delta) — frame-rate
                                    # independent, so both runs visit the same
                                    # viewpoint at the same timestamp.
var _sample_accum := 0.0
var _worst_ms := 0.0                # worst frame in the current sample window
var _csv: FileAccess
var _label: Label
var _bench_eye0: Vector3            # path is defined relative to the start pose
var _bench_yaw0: float
var _sea_level: int = 0             # clamp the flight above this (never underwater)
var _tag := ""                      # VIVARIUM_TAG: prefixes CSV + shots so sweep
                                    # runs don't clobber each other's output
var _shot_interval := 5.0           # seconds between bench screenshots
var _next_shot_t := 1e9             # path-time of the next screenshot
var _clearance := 250.0             # bench eye height above ground/sea (VIVARIUM_CLEAR)

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

	# Mesher thread count. NOTE (2026-06-23): raising this to cores-2 (14/16)
	# REGRESSED overall performance — oversubscribing the mesher pool (each worker
	# calls back into Rust through the FFI/RwLock per chunk) starved the main +
	# render threads. So we now leave the engine default (≈half the cores) UNLESS
	# VIVARIUM_THREADS is set, and even then sweep upward cautiously.
	var threads_env := OS.get_environment("VIVARIUM_THREADS")
	if threads_env != "":
		var engine = Engine.get_singleton("VoxelEngine")
		if engine != null:
			engine.set_thread_count(int(threads_env))
			_trace("voxel mesher threads=%s (of %d cores)" % [threads_env, OS.get_processor_count()])
	world = ClassDB.instantiate("VivariumWorld")
	world.name = "VivariumWorld"
	add_child(world)
	detail = world.voxels_per_unit()
	_sea_level = world.sea_level()
	_trace("world added, detail=%d, sea_level=%d" % [detail, _sea_level])

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
	# Mesh block size (16 or 32). Bigger = fewer, larger mesh tasks (less per-block
	# overhead) at coarser streaming granularity — a candidate "free" win (same
	# view). VIVARIUM_MESH_BLOCK to A/B it; default is the engine's 16.
	var mb_env := OS.get_environment("VIVARIUM_MESH_BLOCK")
	if mb_env != "":
		terrain.mesh_block_size = int(mb_env)

	# --- LOD responsiveness under fast motion (Joseph 2026-06-23) -------------
	# Slow-and-steady prioritizes well; "whipping around" leaves the thing in
	# front macro while something distant refines. Two causes, two levers:
	#
	#  1. threaded_update_enabled moves the octree LOD-subdivision *decision* off the
	#     main thread. In theory that helps when the main thread is the bottleneck —
	#     but enabling it (2026-06-23) ADDED visible artifacts (cross-thread
	#     inconsistency), so it's OFF by default now. Opt in with
	#     VIVARIUM_THREADED_UPDATE=1 to A/B it.
	terrain.threaded_update_enabled = OS.get_environment("VIVARIUM_THREADED_UPDATE") == "1"
	#  2. LOD cross-fade. Steadies a developed area (no jostle), but mid-transition
	#     it renders two LODs (overdraw) — cost exactly while you're moving. So OFF
	#     by default (engine default 0.0); set VIVARIUM_LOD_FADE=0.25 to try it.
	var fade_env := OS.get_environment("VIVARIUM_LOD_FADE")
	if fade_env != "":
		terrain.lod_fade_duration = float(fade_env)
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
	# Flying over a surface, the full-height column (ratio 1.0 = mesh 32 k voxels
	# up and down) is mostly wasted budget — squashing it *should* be a pure win,
	# but bundled with the other changes (2026-06-23) the net was a regression and
	# it wasn't isolated, so default is left at the engine's 1.0 until measured
	# alone. Set VIVARIUM_VRATIO=0.3 to try it (lower = less vertical reach).
	var vratio_env := OS.get_environment("VIVARIUM_VRATIO")
	if vratio_env != "":
		viewer.view_distance_vertical_ratio = float(vratio_env)
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

	# Telemetry / benchmark wiring (see the block by the member vars).
	_bench = OS.get_environment("VIVARIUM_BENCH") == "1"
	_telemetry_on = _bench or OS.get_environment("VIVARIUM_TELEMETRY") == "1"
	_tag = OS.get_environment("VIVARIUM_TAG")   # "" unless set
	if _bench:
		var secs_env := OS.get_environment("VIVARIUM_BENCH_SECS")
		if secs_env != "":
			_bench_secs = float(secs_env)
		var shot_env := OS.get_environment("VIVARIUM_SHOT_SECS")
		if shot_env != "":
			_shot_interval = float(shot_env)
		var clear_env := OS.get_environment("VIVARIUM_CLEAR")
		if clear_env != "":
			_clearance = float(clear_env)
		_next_shot_t = _shot_interval
		# Start over LAND, not the seaward spawn — otherwise the first half of the
		# out-and-back path is over open ocean and those samples don't measure
		# terrain work. Begin at the highest sampled point (deep in the continent),
		# so the whole path stays over real relief and every sample is comparable.
		cam.position = _pick_bench_start()
		# The scripted path is defined relative to the camera's start pose, so it
		# flies the same shape regardless of where we spawn.
		_bench_eye0 = cam.position
		_bench_yaw0 = cam.rotation.y
		cam.set_process(false)        # take the controls; drive the camera ourselves
		cam.set_process_input(false)
		_setup_csv()
	elif _telemetry_on:
		_setup_csv()
	if _telemetry_on:
		_setup_label()

	if OS.get_environment("VIVARIUM_AUTOSHOT") != "":
		# A whole-landmass view distance has a *lot* to stream initially; give the
		# mesher threads generous time before the vista shot. Env-tunable so a
		# bigger reach can wait longer.
		var settle := 25.0
		var settle_env := OS.get_environment("VIVARIUM_SETTLE")
		if settle_env != "":
			settle = float(settle_env)
		get_tree().create_timer(settle).timeout.connect(_capture_and_quit)
	elif _bench:
		print("[vivarium] BENCH: flying a fixed %ds path; telemetry -> user://telemetry.csv" % int(_bench_secs))
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

# --- Telemetry & benchmark ------------------------------------------------------

func _process(delta: float) -> void:
	if not _telemetry_on:
		return
	# Worst frame in the current window — the smoothed FPS hides the hitches that
	# are the actual complaint, so we surface the worst separately.
	var ms := delta * 1000.0
	if ms > _worst_ms:
		_worst_ms = ms

	if _bench:
		_bench_t += delta
		_drive_bench_camera(_bench_t, delta)
		# Periodic screenshots along the path: numbers tell us the cost, but only
		# the image tells us whether the terrain actually developed. Captured at
		# fixed path-times so two configs' shots are directly comparable.
		if _bench_t >= _next_shot_t:
			_capture_bench_shot(_bench_t)
			_next_shot_t += _shot_interval

	_sample_accum += delta
	if _sample_accum >= 1.0 / TELEMETRY_HZ:
		_emit_sample(_bench_t if _bench else Time.get_ticks_msec() / 1000.0)
		_sample_accum = 0.0
		_worst_ms = 0.0

	if _bench and _bench_t >= _bench_secs:
		_capture_bench_shot(_bench_t)   # always grab a final frame
		_finish_bench()

# Pick the highest land point on a coarse grid over the continent — a deep-interior
# start, so the out-and-back flight stays over relief the whole way. One-time scan
# (~81 cheap FFI height lookups). Y is left at 0; terrain-follow sets it next frame.
func _pick_bench_start() -> Vector3:
	var best := Vector3.ZERO
	var best_h := -1
	for gz in range(-4, 5):
		for gx in range(-4, 5):
			var x := gx * 1000
			var z := gz * 1000
			var h: int = world.surface_height(x, z)
			if h > best_h:
				best_h = h
				best = Vector3(float(x), 0.0, float(z))
	_trace("bench start (highest of grid) = %v, h=%d" % [best, best_h])
	return best

# Deterministic stress path: cruise straight, hard 180° whip-around (the regime
# that exposed the LOD churn), then cruise the new heading. Yaw is a pure function
# of path-time; position integrates a constant forward speed and follows the
# ground. Identical every run, so two configs are directly comparable.
func _drive_bench_camera(t: float, delta: float) -> void:
	const V := 300.0          # voxels/sec cruise (~150 m/s at detail 2)
	const WHIP_START := 10.0
	const WHIP_DUR := 3.0
	var yaw := _bench_yaw0
	if t > WHIP_START:
		var w: float = clampf((t - WHIP_START) / WHIP_DUR, 0.0, 1.0)
		yaw = _bench_yaw0 + PI * w        # smooth 180° turn over WHIP_DUR seconds
	cam.rotation = Vector3(-0.30, yaw, 0.0)   # look down ~17° so terrain fills frame
	var fwd := Vector3(-sin(yaw), 0.0, -cos(yaw))   # Godot forward is -Z at yaw 0
	cam.position += fwd * V * delta
	# Terrain-follow, but stay ABOVE the water surface: surface_height under the
	# ocean is the seabed, so following it alone flies the camera underwater and we
	# render the world from below (useless intel). Clamp to max(ground, sea) + a
	# real clearance so the flight is always a useful over-the-terrain oblique.
	# Clearance is the framing knob (VIVARIUM_CLEAR): lower = closer/more detail,
	# higher = wider vista.
	var gh: int = world.surface_height(int(cam.position.x), int(cam.position.z))
	var floor_y: int = maxi(gh, _sea_level)
	cam.position.y = floor_y + _clearance

func _emit_sample(t: float) -> void:
	var fps := Engine.get_frames_per_second()
	var draw := int(Performance.get_monitor(Performance.RENDER_TOTAL_DRAW_CALLS_IN_FRAME))
	var vmem_mb := Performance.get_monitor(Performance.RENDER_VIDEO_MEM_USED) / 1048576.0
	var st: Dictionary = terrain.get_statistics()
	var q_tot := 0
	var q_mesh := 0
	var q_gen := 0
	var e = Engine.get_singleton("VoxelEngine")
	if e != null:
		var es: Dictionary = e.get_stats()
		if es.has("tasks"):
			q_mesh = int(es["tasks"].get("meshing", 0))
			q_gen = int(es["tasks"].get("generation", 0))
		if es.has("thread_pools") and es["thread_pools"].has("general"):
			q_tot = int(es["thread_pools"]["general"].get("tasks", 0))
	var mesh_blocks: int = terrain.debug_get_mesh_block_count()
	var data_blocks: int = terrain.debug_get_data_block_count()
	var dropped := int(st.get("dropped_block_meshs", 0))
	var blocked := int(st.get("blocked_lods", 0))

	if _csv != null:
		_csv.store_line("%.2f,%.1f,%.1f,%d,%d,%d,%d,%d,%d,%d,%d,%.1f" % [
			t, fps, _worst_ms, draw, mesh_blocks, data_blocks,
			q_tot, q_mesh, q_gen, dropped, blocked, vmem_mb])
		_csv.flush()
	if _label != null:
		_label.text = ("t=%5.1fs   fps=%4.1f   worst=%5.1f ms\n" +
			"mesh blocks=%d   data blocks=%d\n" +
			"queue  total=%d  mesh=%d  gen=%d\n" +
			"dropped mesh=%d   blocked lod=%d\n" +
			"draw calls=%d   vmem=%.0f MB") % [
			t, fps, _worst_ms, mesh_blocks, data_blocks,
			q_tot, q_mesh, q_gen, dropped, blocked, draw, vmem_mb]

func _setup_csv() -> void:
	_csv = FileAccess.open("user://telemetry%s.csv" % _tag, FileAccess.WRITE)
	if _csv != null:
		_csv.store_line("t,fps,worst_ms,draw_calls,mesh_blocks,data_blocks,q_total,q_mesh,q_gen,dropped_mesh,blocked_lod,vmem_mb")
		_csv.flush()

# Grab the current frame to user://bench<tag>_t<NN>.png. Path-time in the name so
# a sweep's shots sort and pair up across configs (t05, t10, …).
func _capture_bench_shot(t: float) -> void:
	var img := get_viewport().get_texture().get_image()
	var path := "user://bench%s_t%02d.png" % [_tag, int(round(t))]
	img.save_png(path)
	_trace("shot %s (fps=%.1f)" % [path, Engine.get_frames_per_second()])

func _setup_label() -> void:
	_label = Label.new()
	_label.position = Vector2(12, 12)
	# White text with a dark outline so it reads over both bright terrain and sky.
	_label.add_theme_color_override("font_color", Color.WHITE)
	_label.add_theme_color_override("font_outline_color", Color(0, 0, 0, 0.8))
	_label.add_theme_constant_override("outline_size", 6)
	add_child(_label)

var _bench_done := false
func _finish_bench() -> void:
	if _bench_done:
		return
	_bench_done = true
	if _csv != null:
		_csv.flush()
		_csv.close()
	var p := ProjectSettings.globalize_path("user://telemetry%s.csv" % _tag)
	print("[vivarium] BENCH done (%.0fs). CSV=%s" % [_bench_secs, p])
	_trace("bench done -> " + p)
	get_tree().quit()
