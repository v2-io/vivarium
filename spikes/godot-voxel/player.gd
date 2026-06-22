# First-person fly camera + voxel editing for the Godot spike.
#
# Noclip on purpose: the point of this build is to feel mouse-look responsiveness
# and voxel editing, not locomotion/physics (collision against voxel terrain can
# come later if the feel test warrants it). Left click digs, right click places.
#
# Editing writes to BOTH sides and that is deliberate: world.dig/place persists
# the change in vivarium-core (the source of truth), and VoxelTool.set_voxel
# updates the visible mesh immediately. Because core holds the edit, if that
# chunk later unloads and regenerates, the generator reads core and reproduces
# exactly the same result — no divergence between what you see and what is true.
extends Camera3D

var world: Object = null     # VivariumWorld bridge
var terrain: Object = null   # VoxelTerrain

const MOVE_SPEED := 24.0
const FAST_MULT := 3.0
const MOUSE_SENS := 0.0025
const REACH := 120.0
const PLACE_MATERIAL := 1     # STONE, for now

var _yaw := 0.0
var _pitch := 0.0
var _unit := 1     # voxels per world unit; movement/reach scale with it

func _ready() -> void:
	# Skip mouse capture in automated screenshot runs (no human to drive it).
	if OS.get_environment("VIVARIUM_AUTOSHOT") == "":
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

func _process(delta: float) -> void:
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
