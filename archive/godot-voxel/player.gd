# First-person controller + voxel editing for the Godot spike.
#
# Two modes, toggled with F:
#   WALK (default) — feet on the ground: move horizontally and stay a couple metres
#     above the surface (terrain-follow via core's surface_height). Shift = run.
#   FLY (noclip)   — free 6-DoF traversal for crossing the ~12 km landmass fast;
#     Space/Shift = up/down, Ctrl = fast. The old behaviour, kept because walking
#     across a continent is glacial.
# Look is mouse in both; vertical look never tilts walking direction (forward is
# always horizontal on foot). Left click digs, right click places.
#
# Editing writes to BOTH sides and that is deliberate: world.dig/place persists
# the change in vivarium-core (the source of truth), and VoxelTool.set_voxel
# updates the visible mesh immediately. Because core holds the edit, if that
# chunk later unloads and regenerates, the generator reads core and reproduces
# exactly the same result — no divergence between what you see and what is true.
extends Camera3D

var world: Object = null     # VivariumWorld bridge
var terrain: Object = null   # VoxelTerrain

# Fly speeds (the "m/s" convention: ×_unit → voxels/s). ~100 m/s normal, ~400 fast.
const MOVE_SPEED := 100.0
const FAST_MULT := 4.0
# Walk speeds, same convention. Brisker than a real 1.5 m/s walk so exploring the
# km-scale world on foot isn't painful, but unmistakably grounded vs flying.
const WALK_SPEED := 6.0
const RUN_SPEED := 16.0
# Eye height above the surface while walking, in metres ("a couple metres").
const EYE_HEIGHT := 2.0
const MOUSE_SENS := 0.0025
const REACH := 120.0
const PLACE_MATERIAL := 1     # STONE, for now

var _yaw := 0.0
var _pitch := 0.0
var _unit := 1      # voxels per world unit; movement/reach/eye-height scale with it
var _walk := true   # start on foot; F toggles to fly

func _ready() -> void:
	if OS.get_environment("VIVARIUM_AUTOSHOT") != "":
		# Automated screenshot run: main.gd owns the camera for a fixed vista, so
		# disable this controller entirely — otherwise stray input moves the shot.
		set_process(false)
		set_process_input(false)
	else:
		Input.mouse_mode = Input.MOUSE_MODE_CAPTURED
	if world != null:
		_unit = world.voxels_per_unit()
	resync()

# Re-read yaw/pitch from the current rotation. Called by main.gd after it aims
# the camera with look_at(), so the first mouse move continues from that heading.
func resync() -> void:
	_yaw = rotation.y
	_pitch = rotation.x

func _input(event: InputEvent) -> void:
	if event is InputEventMouseMotion and Input.mouse_mode == Input.MOUSE_MODE_CAPTURED:
		_yaw -= event.relative.x * MOUSE_SENS
		_pitch = clampf(_pitch - event.relative.y * MOUSE_SENS, -1.4, 1.4)
		rotation = Vector3(_pitch, _yaw, 0.0)
	elif event is InputEventMouseButton and event.pressed:
		if event.button_index == MOUSE_BUTTON_LEFT:
			_edit(true)
		elif event.button_index == MOUSE_BUTTON_RIGHT:
			_edit(false)
	elif event is InputEventKey and event.pressed and event.keycode == KEY_ESCAPE:
		# Release/recapture the mouse so the window isn't a trap.
		Input.mouse_mode = Input.MOUSE_MODE_VISIBLE \
			if Input.mouse_mode == Input.MOUSE_MODE_CAPTURED \
			else Input.MOUSE_MODE_CAPTURED
	elif event is InputEventKey and event.pressed and event.keycode == KEY_F:
		# Toggle walk/fly. Next _process frame re-grounds the camera if now walking.
		_walk = not _walk
		print("[vivarium] mode: ", "WALK" if _walk else "FLY")

func _process(delta: float) -> void:
	if _walk:
		_walk_step(delta)
	else:
		_fly_step(delta)

# On foot: move in the horizontal plane only (look pitch never tilts the walk
# direction), then ride the surface a couple of metres up. surface_height is core's
# topmost solid voxel, so you walk the real eroded ground; below sea that's the
# seabed (wading underwater — collision/water handling is a later concern).
func _walk_step(delta: float) -> void:
	var b := global_transform.basis
	var fwd := Vector3(-b.z.x, 0.0, -b.z.z).normalized()   # heading, flattened
	var rgt := Vector3(b.x.x, 0.0, b.x.z).normalized()
	var dir := Vector3.ZERO
	if Input.is_key_pressed(KEY_W): dir += fwd
	if Input.is_key_pressed(KEY_S): dir -= fwd
	if Input.is_key_pressed(KEY_D): dir += rgt
	if Input.is_key_pressed(KEY_A): dir -= rgt
	if dir != Vector3.ZERO:
		var speed := (RUN_SPEED if Input.is_key_pressed(KEY_SHIFT) else WALK_SPEED) * _unit
		position += dir.normalized() * speed * delta
	# Re-ground every frame (also catches a fresh walk-mode toggle and moving terrain).
	if world != null:
		var gh: int = world.surface_height(int(round(position.x)), int(round(position.z)))
		if gh > 0:
			position.y = gh + EYE_HEIGHT * _unit

func _fly_step(delta: float) -> void:
	var dir := Vector3.ZERO
	var b := global_transform.basis
	if Input.is_key_pressed(KEY_W): dir -= b.z
	if Input.is_key_pressed(KEY_S): dir += b.z
	if Input.is_key_pressed(KEY_A): dir -= b.x
	if Input.is_key_pressed(KEY_D): dir += b.x
	if Input.is_key_pressed(KEY_SPACE): dir += Vector3.UP
	if Input.is_key_pressed(KEY_SHIFT): dir += Vector3.DOWN
	if dir != Vector3.ZERO:
		# Scale by resolution so a finer (physically larger) world traverses at a
		# comparable felt speed.
		var speed := MOVE_SPEED * _unit * (FAST_MULT if Input.is_key_pressed(KEY_CTRL) else 1.0)
		position += dir.normalized() * speed * delta

func _edit(is_dig: bool) -> void:
	if world == null or terrain == null:
		return
	var vt = terrain.get_voxel_tool()
	vt.channel = VoxelBuffer.CHANNEL_COLOR
	# The terrain is scaled by 1/detail, and VoxelTool works in the terrain's
	# local (voxel) space — so convert the camera ray out of world space.
	# `terrain` is an untyped Object, so these return Variant — use plain `=`
	# (not `:=`, which needs an inferable static type).
	var inv = terrain.global_transform.affine_inverse()
	var local_origin = inv * global_position
	var local_dir = (inv.basis * (-global_transform.basis.z)).normalized()
	var detail: int = world.voxels_per_unit()
	var hit = vt.raycast(local_origin, local_dir, REACH * detail)
	if hit == null:
		return
	if is_dig:
		var p: Vector3i = hit.position             # the solid voxel hit
		world.dig(p.x, p.y, p.z)                   # persist in core
		vt.set_voxel(p, 0)                         # immediate remesh: air
	else:
		var p: Vector3i = hit.previous_position    # the empty cell before it
		world.place(p.x, p.y, p.z, PLACE_MATERIAL)
		vt.set_voxel(p, PLACE_MATERIAL)
