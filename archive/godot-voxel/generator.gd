# Custom godot_voxel generator that sources every voxel from vivarium-core.
#
# This is the heart of the Godot spike: the simulation core is the single source
# of truth for terrain, and the engine asks *it* what each chunk contains. The
# core never learns that Godot exists. godot_voxel calls _generate_block on
# worker threads, so this leans on the bridge's generate_block being a pure,
# &self read (safe to call concurrently).
extends VoxelGeneratorScript

# The Rust bridge node (vivarium_core::World). Set by main.gd before the terrain
# starts streaming.
var world: Object = null

func _get_used_channels_mask() -> int:
	# Cubes mesher reads colours (here, palette indices) from CHANNEL_COLOR.
	return 1 << VoxelBuffer.CHANNEL_COLOR

func _generate_block(buffer: VoxelBuffer, origin: Vector3i, lod: int) -> void:
	if world == null:
		return
	var size: Vector3i = buffer.get_size()
	# One FFI crossing for the whole chunk; core point-samples at stride 2^lod so
	# distant blocks come back coarse (view resolution decoupled from intrinsic).
	var data: PackedByteArray = world.generate_block(origin, size, lod)
	var i := 0
	for z in size.z:
		for y in size.y:
			for x in size.x:
				var m := data[i]
				i += 1
				# Material id == palette index. 0 (air) stays empty.
				if m != 0:
					buffer.set_voxel(m, x, y, z, VoxelBuffer.CHANNEL_COLOR)
