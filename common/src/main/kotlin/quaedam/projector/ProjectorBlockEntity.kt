package quaedam.projector

import net.minecraft.core.BlockPos
import net.minecraft.core.Vec3i
import net.minecraft.nbt.CompoundTag
import net.minecraft.network.protocol.Packet
import net.minecraft.network.protocol.game.ClientGamePacketListener
import net.minecraft.network.protocol.game.ClientboundBlockEntityDataPacket
import net.minecraft.resources.ResourceLocation
import net.minecraft.server.level.ServerLevel
import net.minecraft.world.level.ChunkPos
import net.minecraft.world.level.block.entity.BlockEntity
import net.minecraft.world.level.block.state.BlockState
import net.minecraft.world.level.levelgen.structure.BoundingBox
import net.minecraft.world.phys.AABB
import quaedam.projection.ProjectionEffect
import quaedam.projection.ProjectionEffectType
import quaedam.projection.ProjectionProvider
import quaedam.utils.sendBlockUpdated

class ProjectorBlockEntity(pos: BlockPos, state: BlockState) :
    BlockEntity(Projector.blockEntity.get(), pos, state) {

    val effectAreaChunk by lazy {
        val chunk = level!!.getChunk(pos).pos
        ChunkPos(chunk.x - Projector.EFFECT_RADIUS, chunk.z - Projector.EFFECT_RADIUS) to
                ChunkPos(chunk.x + Projector.EFFECT_RADIUS, chunk.z + Projector.EFFECT_RADIUS)
    }

    val effectArea: BoundingBox by lazy {
        val (minChunk, maxChunk) = effectAreaChunk
        val minBlock = BlockPos(minChunk.minBlockX, level!!.minBuildHeight, minChunk.minBlockZ)
        val maxBlock = BlockPos(maxChunk.maxBlockX, level!!.maxBuildHeight, maxChunk.maxBlockZ)
        BoundingBox.fromCorners(minBlock, maxBlock)
    }

    val effectAreaAABB by lazy {
        val (minChunk, maxChunk) = effectAreaChunk
        val minBlock = BlockPos(minChunk.minBlockX, level!!.minBuildHeight, minChunk.minBlockZ)
        val maxBlock = BlockPos(maxChunk.maxBlockX, level!!.maxBuildHeight, maxChunk.maxBlockZ)
        AABB(minBlock, maxBlock)
    }

    val checkArea: BoundingBox by lazy {
        BoundingBox.fromCorners(pos.offset(-2, -1, -2), pos.offset(2, -2, 2))
    }

    var effects: Map<ProjectionEffectType<*>, ProjectionEffect> = emptyMap()

    override fun saveAdditional(tag: CompoundTag) {
        super.saveAdditional(tag)
        val effectsTag = CompoundTag()
        effects.map { (type, effect) ->
            effectsTag.put(type.id.toString(), effect.toNbt())
        }
        tag.put("ProjectionEffects", effectsTag)
    }

    override fun load(tag: CompoundTag) {
        super.load(tag)
        val effectsTag = tag["ProjectionEffects"]
        val effects = mutableMapOf<ProjectionEffectType<*>, ProjectionEffect>()
        if (effectsTag != null && effectsTag is CompoundTag) {
            effectsTag.allKeys.forEach { id ->
                val type = ProjectionEffectType.registry[ResourceLocation(id)]
                if (type != null) {
                    val effect = type.constructor().apply { fromNbt(effectsTag[id] as CompoundTag) }
                    effects[type] = effect
                }
            }
        }
        updateEffects(effects, notify = false)
    }

    override fun getUpdateTag(): CompoundTag = saveWithoutMetadata()

    override fun getUpdatePacket(): Packet<ClientGamePacketListener> = ClientboundBlockEntityDataPacket.create(this)

    override fun setRemoved() {
        super.setRemoved()
        updateEffects(emptyMap(), notify = false)
    }

    operator fun contains(pos: Vec3i) = effectArea.isInside(pos)

    operator fun contains(pos: ChunkPos) =
        this.contains(Vec3i(pos.middleBlockX, level!!.minBuildHeight, pos.middleBlockZ))

    fun checkUpdate() {
        if (level!!.isClientSide)
            return
        val effects = collectEffects()
        updateEffects(effects)
    }

    fun updateEffects(effects: Map<ProjectionEffectType<*>, ProjectionEffect>, notify: Boolean = true) {
        if (effects != this.effects) {
            val oldEffects = this.effects
            this.effects = effects
            if (level != null) {
                val level = level!!
                if (!level.isClientSide && notify) {
                    sendBlockUpdated()
                }
                val addedEffects = effects.filterKeys { it !in oldEffects }
                val removedEffects = oldEffects.filterKeys { it !in effects }
                val updatedEffects = effects.filter { (k, v) -> oldEffects[k] != null && oldEffects[k] != v }
                addedEffects.values.forEach { it.activate(level, blockPos) }
                removedEffects.values.forEach { it.deactivate(level, blockPos) }
                updatedEffects.forEach { (k, v) ->
                    oldEffects[k]!!.deactivate(level, blockPos)
                    v.activate(level, blockPos)
                }
            }
        }
    }

    fun collectEffects(): Map<ProjectionEffectType<*>, ProjectionEffect> {
        val level = level!! as ServerLevel
        if (!level.getBlockState(blockPos.below()).isAir) {
            return emptyMap()
        }
        val effects = mutableMapOf<ProjectionEffectType<*>, ProjectionEffect>()
        for (x in checkArea.minX()..checkArea.maxX()) {
            for (y in checkArea.minY()..checkArea.maxY()) {
                for (z in checkArea.minZ()..checkArea.maxZ()) {
                    val pos = BlockPos(x, y, z)
                    val blockState = level.getBlockState(pos)
                    val block = blockState.block
                    if (block is ProjectionProvider<*>) {
                        val projection = block.createProjectionEffect(level, blockState, pos)
                        if (projection != null) {
                            effects[projection.type] = projection
                        }
                    }
                }
            }
        }
        return effects
    }

}
