package quaedam.projection.swarm

import dev.architectury.platform.Platform
import dev.architectury.registry.client.level.entity.EntityRendererRegistry
import dev.architectury.registry.level.entity.EntityAttributeRegistry
import net.fabricmc.api.EnvType
import net.minecraft.nbt.CompoundTag
import net.minecraft.network.chat.Component
import net.minecraft.network.syncher.EntityDataAccessor
import net.minecraft.network.syncher.EntityDataSerializers
import net.minecraft.network.syncher.SynchedEntityData
import net.minecraft.world.DifficultyInstance
import net.minecraft.world.entity.*
import net.minecraft.world.entity.ai.attributes.AttributeSupplier
import net.minecraft.world.entity.ai.attributes.Attributes
import net.minecraft.world.level.Level
import net.minecraft.world.level.ServerLevelAccessor
import quaedam.Quaedam

class ProjectedPersonEntity(entityType: EntityType<out PathfinderMob>, level: Level) :
    PathfinderMob(entityType, level) {

    companion object {

        const val ID = "projected_person"

        const val KEY_ENTITY_SHAPE = "EntityShape"

        const val BOUNDING_WIDTH = 0.6f
        const val BOUNDING_HEIGHT = 1.8f

        val entity = Quaedam.entities.register(ID) {
            EntityType.Builder.of(::ProjectedPersonEntity, MobCategory.CREATURE)
                .canSpawnFarFromPlayer()
                .sized(BOUNDING_WIDTH, BOUNDING_HEIGHT * 1.2f)
                .build("quaedam:$ID")
        }!!

        val dataShape =
            SynchedEntityData.defineId(ProjectedPersonEntity::class.java, EntityDataSerializers.COMPOUND_TAG)

        private fun createAttributes(): AttributeSupplier.Builder = Mob.createMobAttributes()
            .add(Attributes.ATTACK_DAMAGE, 1.5)
            .add(Attributes.MOVEMENT_SPEED, 0.11)
            .add(Attributes.ATTACK_SPEED)

        init {
            EntityAttributeRegistry.register(entity, ::createAttributes)
            if (Platform.getEnv() == EnvType.CLIENT) {
                EntityRendererRegistry.register(entity, ::ProjectedPersonRenderer)
            }
            ProjectedPersonShape
        }

    }

    override fun finalizeSpawn(
        serverLevelAccessor: ServerLevelAccessor,
        difficultyInstance: DifficultyInstance,
        mobSpawnType: MobSpawnType,
        spawnGroupData: SpawnGroupData?,
        compoundTag: CompoundTag?
    ): SpawnGroupData? {
        shape = ProjectedPersonShape.create(serverLevelAccessor.random.nextLong())
        return super.finalizeSpawn(serverLevelAccessor, difficultyInstance, mobSpawnType, spawnGroupData, compoundTag)
    }

    override fun defineSynchedData() {
        super.defineSynchedData()
        entityData.define(dataShape, CompoundTag())
    }

    private var shapeTag
        get() = entityData.get(dataShape)
        set(value) = entityData.set(dataShape, value)

    var shape = ProjectedPersonShape()
        set(value) {
            field = value
            shapeTag = shape.toTag()
        }

    override fun onSyncedDataUpdated(data: EntityDataAccessor<*>) {
        if (data == dataShape) {
            shape = ProjectedPersonShape.fromTag(shapeTag)
        }
        super.onSyncedDataUpdated(data)
    }

    override fun addAdditionalSaveData(tag: CompoundTag) {
        super.addAdditionalSaveData(tag)
        tag.put(KEY_ENTITY_SHAPE, shapeTag)
    }

    override fun readAdditionalSaveData(tag: CompoundTag) {
        super.readAdditionalSaveData(tag)
        shapeTag = tag.getCompound(KEY_ENTITY_SHAPE)
    }

    override fun shouldShowName() = true

    override fun getTypeName(): Component = shape.name.takeIf { it.isNotEmpty() }?.let { Component.literal(it) }
        ?: super.getTypeName()

    override fun getNameTagOffsetY() = super.getNameTagOffsetY() - BOUNDING_HEIGHT * (1.3f - shape.scaleY)

}