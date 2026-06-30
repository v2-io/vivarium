# Isometric navigator camera for the Godot spike — a RimWorld/DF-style overhead
# rig over the *same* voxel world the first-person walk view uses. Selected with
# VIVARIUM_VIEW=iso (otherwise main.gd attaches player.gd and you walk).
#
# WHY a separate script (not a mode flag inside player.gd): the navigator owns a
# different contract with the renderer. Walk/fly is a first-person camera whose
# VoxelViewer rides *with the eye*; the navigator is an orthographic camera that
# floats far off at a fixed iso angle while a *separate* viewer sits on the
# ground at the focus point so streaming centres on what you're looking at, not
# on the floating eye. Keeping the two rigs in separate files keeps each
# contract legible.
#
# DELIBERATELY SEPARABLE FROM THE RENDERER. Joseph's read (2026-06-30) is that
# this real-voxel iso view is the fun first step but that a lighter, abstract
# heightmap navigator (option #2) is the likely next move for performance. So
# everything here is camera + controls + a ground-focus model — it talks to the
# renderer only through `world.surface_height()` (focus follow) and the shared
# VoxelViewer streaming. Swap the voxel terrain for a heightmap mesh underneath
# and this controller should carry over almost untouched.
extends Camera3D

var world: Object = null       # VivariumWorld bridge (for surface_height focus-follow)
var terrain: Object = null     # VoxelLodTerrain (kept for symmetry / future picking)
var view_distance: int = 16384 # voxel streaming reach ceiling, passed from main.gd

# --- Iso framing -----------------------------------------------------------------
# True isometric tilt: looking down the diagonal of a cube projects its three
# visible faces equally, the angle that reads as "isometric" rather than a shallow
# 3/4 view. atan(1/sqrt(2)) ≈ 35.264°. Tunable so dimetric (RimWorld-ish 30°) or a
# steeper near-top-down can be tried without code surgery.
const ISO_PITCH := 0.6154797   # radians, atan(1/sqrt(2)) — true isometric
# Yaw snaps to the four diagonals (45°, 135°, …). Starting at 45° puts "north"
# (−Z in world space) up-and-to-the-right, the conventional iso orientation.
const YAW_START := PI / 4.0
const YAW_STEP := PI / 2.0      # Q/E rotate by a quarter turn

# Camera stand-off distance along the view ray. Orthographic, so this does NOT
# affect apparent size (that's `cam.size`/zoom) — it only sets how far the eye
# floats, which must clear the tallest terrain and stay inside the far plane.
# WORLD_HEIGHT is ~8 km of voxels; 12 km of stand-off clears any peak.
const STANDOFF := 12000.0

# Zoom is the orthographic frustum's vertical extent in voxels (= metres at the
# 0.5 m anchor only via detail; here 1 voxel == 1 unit). Smaller = more zoomed in.
const ZOOM_MIN := 60.0         # tight, a few buildings' worth
const ZOOM_MAX := 6000.0       # most of the landmass
const ZOOM_START := 250.0      # in close on the origin — agent/colony scale, not the whole continent
const ZOOM_STEP := 1.12        # multiplicative per wheel notch (feels linear-in-log)

# Pan speed in *screen-fractions per second* — scaled by zoom so the world slides
# under the cursor at a constant felt rate whether zoomed in or out.
const PAN_KEY_RATE := 0.9      # fraction of the visible span per second at full tilt
const ROT_LERP := 12.0         # how snappily yaw eases to its target (higher = snappier)

var _focus := Vector3.ZERO     # the ground point the camera orbits/looks at
var _yaw := YAW_START
var _yaw_target := YAW_START
var _zoom := ZOOM_START
var _unit := 1                 # voxels per world unit (movement scales with it)

var _dragging := false
var _drag_last := Vector2.ZERO

func _ready() -> void:
	if world != null:
		_unit = world.voxels_per_unit()

	# Orthographic is what makes it read as iso: parallel projection, no
	# perspective convergence, so a cube is the same size near and far.
	projection = PROJECTION_ORTHOGONAL
	size = _zoom
	far = STANDOFF + float(view_distance) + 8192.0
	near = 1.0

	# Start centred on the region origin, dropped onto the ground there.
	_focus = Vector3(0, 0, 0)
	_ground_focus()
	_place_camera(true)

	# A VoxelViewer pinned to the FOCUS (not to this floating eye) so the terrain
	# streams around what you're looking at. Decoupling it from the camera is the
	# whole reason streaming stays cheap when the eye is 12 km away — and it's
	# exactly the focus-centred streaming the future abstract navigator wants.
	_viewer = VoxelViewer.new()
	add_child(_viewer)   # must be in-tree before we set its GLOBAL position
	_sync_viewer()

	if OS.get_environment("VIVARIUM_AUTOSHOT") != "":
		# Automated verification run: main.gd's timer grabs the frame. Frame a
		# wide vista of the whole region and then hand off — don't process input.
		_zoom = 5000.0
		size = _zoom
		_place_camera(true)
		_sync_viewer()
		set_process(false)
		set_process_input(false)
	else:
		# Navigator wants a free cursor (for drag-pan and, later, picking) — unlike
		# the first-person rig which captures the mouse for look.
		Input.mouse_mode = Input.MOUSE_MODE_VISIBLE
		print("[vivarium] ISO NAVIGATOR: WASD/arrows pan · drag (LMB) pan · ",
			"wheel zoom · Q/E rotate 90° · R re-centre origin")

var _viewer: VoxelViewer

# --- Per-frame ------------------------------------------------------------------

func _process(delta: float) -> void:
	# Keyboard pan, in the screen plane. "Up" on screen is the camera's forward
	# projected onto the ground; "right" is the camera's right. Pan distance is a
	# fraction of the visible span (= zoom), so it feels the same at any zoom.
	var dir := Vector2.ZERO
	if Input.is_key_pressed(KEY_W) or Input.is_key_pressed(KEY_UP):    dir.y += 1.0
	if Input.is_key_pressed(KEY_S) or Input.is_key_pressed(KEY_DOWN):  dir.y -= 1.0
	if Input.is_key_pressed(KEY_D) or Input.is_key_pressed(KEY_RIGHT): dir.x += 1.0
	if Input.is_key_pressed(KEY_A) or Input.is_key_pressed(KEY_LEFT):  dir.x -= 1.0
	if dir != Vector2.ZERO:
		var step := PAN_KEY_RATE * _zoom * delta
		_pan_screen(dir.normalized() * step)

	# Ease yaw toward its snapped target so Q/E reads as a turn, not a jump.
	if absf(_yaw - _yaw_target) > 0.0001:
		_yaw = lerp_angle(_yaw, _yaw_target, clampf(ROT_LERP * delta, 0.0, 1.0))
		_place_camera(false)

	# Keep the focus glued to the ground as it pans (also catches the eased turn).
	_ground_focus()
	_place_camera(false)
	_sync_viewer()

# Move the focus by (right, up) screen amounts, where "up" is forward-on-ground.
func _pan_screen(screen: Vector2) -> void:
	var b := global_transform.basis
	var right := Vector3(b.x.x, 0.0, b.x.z).normalized()
	var fwd := Vector3(-b.z.x, 0.0, -b.z.z).normalized()   # camera forward, flattened
	_focus += right * screen.x + fwd * screen.y

# Drop the focus' Y onto the terrain (or sea) so the iso frame is vertically
# centred on the surface under it, not floating at y=0 or buried.
func _ground_focus() -> void:
	if world == null:
		return
	var gh: int = world.surface_height(int(round(_focus.x)), int(round(_focus.z)))
	var sea: int = world.sea_level()
	_focus.y = float(maxi(gh, sea))

# Position + aim the camera from focus, yaw, pitch, stand-off. `snap` re-applies
# zoom too (used on init / explicit framing).
func _place_camera(snap: bool) -> void:
	if snap:
		size = _zoom
	# Look direction: horizontal heading rotated by yaw, tilted down by ISO_PITCH.
	var look := Vector3(sin(_yaw), 0.0, cos(_yaw)) * cos(ISO_PITCH) + Vector3.DOWN * sin(ISO_PITCH)
	look = look.normalized()
	global_position = _focus - look * STANDOFF
	look_at(_focus, Vector3.UP)

# Park the streaming viewer on the ground focus and size its reach to the zoom,
# so we stream a bubble a little larger than what's on screen — not the whole
# 12 km landmass. Clamped to the configured ceiling.
func _sync_viewer() -> void:
	if _viewer == null:
		return
	_viewer.global_position = _focus
	# Visible span ~= zoom; stream ~1.6× that radius so panning has headroom.
	var reach := int(clampf(_zoom * 1.6, 256.0, float(view_distance)))
	_viewer.view_distance = reach

# --- Input ----------------------------------------------------------------------

func _input(event: InputEvent) -> void:
	if event is InputEventMouseButton and event.pressed:
		if event.button_index == MOUSE_BUTTON_WHEEL_UP:
			_zoom = clampf(_zoom / ZOOM_STEP, ZOOM_MIN, ZOOM_MAX)
			_place_camera(true)
		elif event.button_index == MOUSE_BUTTON_WHEEL_DOWN:
			_zoom = clampf(_zoom * ZOOM_STEP, ZOOM_MIN, ZOOM_MAX)
			_place_camera(true)
		elif event.button_index == MOUSE_BUTTON_LEFT:
			_dragging = true
			_drag_last = event.position
	elif event is InputEventMouseButton and not event.pressed:
		if event.button_index == MOUSE_BUTTON_LEFT:
			_dragging = false
	elif event is InputEventMouseMotion and _dragging:
		# Drag-pan: a pixel of cursor travel moves the world by the same fraction
		# of the visible span, so the ground tracks the cursor 1:1. Screen +x is
		# camera-right; screen +y (downward) should pull the world toward you, so
		# forward gets −y.
		var px := get_viewport().get_visible_rect().size
		var frac := Vector2(event.relative.x / px.y, -event.relative.y / px.y)  # px.y: size is vertical-extent
		_pan_screen(Vector2(-frac.x, -frac.y) * _zoom)   # negate: drag moves world with the cursor
	elif event is InputEventKey and event.pressed and not event.echo:
		match event.keycode:
			KEY_Q: _yaw_target -= YAW_STEP
			KEY_E: _yaw_target += YAW_STEP
			KEY_R:
				_focus = Vector3(0, 0, 0)
				_ground_focus()
				_place_camera(true)
