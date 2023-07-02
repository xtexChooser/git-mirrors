package quaedam.projection.swarm

import com.mojang.serialization.Dynamic
import dev.architectury.platform.Platform
import dev.architectury.registry.level.entity.EntityAttributeRegistry
import net.fabricmc.api.EnvType
import net.minecraft.nbt.CompoundTag
import net.minecraft.network.chat.Component
import net.minecraft.network.protocol.game.DebugPackets
import net.minecraft.network.syncher.EntityDataAccessor
import net.minecraft.network.syncher.EntityDataSerializers
import net.minecraft.network.syncher.SynchedEntityData
import net.minecraft.server.level.ServerLevel
import net.minecraft.world.DifficultyInstance
import net.minecraft.world.SimpleContainer
import net.minecraft.world.entity.*
import net.minecraft.world.entity.ai.Brain
import net.minecraft.world.entity.ai.attributes.AttributeSupplier
import net.minecraft.world.entity.ai.attributes.Attributes
import net.minecraft.world.entity.item.ItemEntity
import net.minecraft.world.entity.npc.InventoryCarrier
import net.minecraft.world.level.Level
import net.minecraft.world.level.ServerLevelAccessor
import quaedam.Quaedam
import quaedam.projector.Projector

class ProjectedPersonEntity(entityType: EntityType<out PathfinderMob>, level: Level) : PathfinderMob(entityType, level),
    InventoryCarrier {

    companion object {

        const val ID = "projected_person"

        const val KEY_ENTITY_SHAPE = "EntityShape"

        const val BOUNDING_WIDTH = 0.6f
        const val BOUNDING_HEIGHT = 1.8f

        val entity = Quaedam.entities.register(ID) {
            EntityType.Builder.of(::ProjectedPersonEntity, MobCategory.CREATURE).canSpawnFarFromPlayer()
                .sized(BOUNDING_WIDTH, BOUNDING_HEIGHT * 1.2f).build("quaedam:$ID")
        }!!

        val dataShape =
            SynchedEntityData.defineId(ProjectedPersonEntity::class.java, EntityDataSerializers.COMPOUND_TAG)

        init {
            EntityAttributeRegistry.register(entity, ::createAttributes)
            if (Platform.getEnv() == EnvType.CLIENT) ProjectedPersonRenderer
            ProjectedPersonShape
            ProjectedPersonAI
        }

        private fun createAttributes(): AttributeSupplier.Builder =
            Mob.createMobAttributes().add(Attributes.ATTACK_DAMAGE, 1.5).add(Attributes.MOVEMENT_SPEED, 0.2)
                .add(Attributes.ATTACK_SPEED)

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
            ProjectedPersonAI.updateSchedule(this)
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
        writeInventoryToTag(tag)
    }

    override fun readAdditionalSaveData(tag: CompoundTag) {
        super.readAdditionalSaveData(tag)
        shapeTag = tag.getCompound(KEY_ENTITY_SHAPE)
        readInventoryFromTag(tag)
    }

    override fun shouldShowName() = true

    override fun getTypeName(): Component =
        shape.name.takeIf { it.isNotEmpty() }?.let { Component.literal(it) } ?: super.getTypeName()

    override fun getNameTagOffsetY() = super.getNameTagOffsetY() - (bbHeight * (1f - shape.scaleY))

    override fun createNavigation(level: Level) = ProjectedPersonNavigation(this, level)

    override fun tick() {
        super.tick()
        if (tickCount % 20 == 0) {
            if (!checkProjectionEffect()) remove(RemovalReason.KILLED)
        }
    }

    private fun checkProjectionEffect() =
        Projector.findNearbyProjections(level(), blockPosition(), SwarmProjection.effect.get()).isNotEmpty()

    override fun checkDespawn() {
        super.checkDespawn()
        if (!checkProjectionEffect()) discard()
    }

    private val inventory = SimpleContainer(10)

    override fun getInventory() = inventory

    override fun pickUpItem(item: ItemEntity) {
        super.pickUpItem(item)
        InventoryCarrier.pickUpItem(this, this, item)
    }

    override fun sendDebugPackets() {
        super.sendDebugPackets()
        DebugPackets.sendEntityBrain(this)
    }

    override fun removeWhenFarAway(d: Double) = false

    // Type signature referenced from: https://github.com/bbrk24/amurians-mod/blob/7a0f0c3c7a3e84c22e5c631286ad23795207adc0/src/main/kotlin/org/bbrk24/amurians/amurian/AmurianEntity.kt#L220
    override fun brainProvider() = ProjectedPersonAI.provider()

    @Suppress("UNCHECKED_CAST")
    override fun makeBrain(dynamic: Dynamic<*>): Brain<out ProjectedPersonEntity> = brainProvider().makeBrain(dynamic)
        .also { ProjectedPersonAI.initBrain(this, it as Brain<ProjectedPersonEntity>) }

    @Suppress("UNCHECKED_CAST")
    override fun getBrain(): Brain<ProjectedPersonEntity> = super.getBrain() as Brain<ProjectedPersonEntity>

    override fun customServerAiStep() {
        super.customServerAiStep()
        getBrain().tick(level() as ServerLevel, this)
    }

    override fun isBaby() = shape.baby

}