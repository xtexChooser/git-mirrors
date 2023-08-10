package quaedam.projection.swarm.ai

import net.minecraft.core.BlockPos
import net.minecraft.server.level.ServerLevel
import net.minecraft.sounds.SoundEvents
import net.minecraft.world.Container
import net.minecraft.world.entity.Mob
import net.minecraft.world.entity.ai.behavior.Behavior
import net.minecraft.world.entity.ai.memory.MemoryModuleType
import net.minecraft.world.entity.ai.memory.MemoryStatus
import net.minecraft.world.entity.ai.memory.WalkTarget
import net.minecraft.world.entity.npc.InventoryCarrier
import net.minecraft.world.item.ItemStack
import net.minecraft.world.level.block.entity.BaseContainerBlockEntity
import net.minecraft.world.level.block.entity.ChestBlockEntity
import kotlin.math.min

class ExchangeItem<E> : Behavior<E>(
    mapOf(
        MemoryModuleType.WALK_TARGET to MemoryStatus.VALUE_ABSENT,
        NearestVisibleContainer.memory.get() to MemoryStatus.VALUE_PRESENT,
    ), 5 * 20, 12 * 20
) where E : Mob, E : InventoryCarrier {

    private var target: BlockPos? = null
    private var closeAt: Long? = null

    override fun start(level: ServerLevel, entity: E, l: Long) {
        target = entity.brain.getMemory(NearestVisibleContainer.memory.get()).get()
        closeAt = null
        entity.brain.setMemory(MemoryModuleType.WALK_TARGET, WalkTarget(target!!, 1.0f, 2))
        entity.brain.eraseMemory(MemoryModuleType.CANT_REACH_WALK_TARGET_SINCE)
    }

    override fun canStillUse(level: ServerLevel, owner: E, gameTime: Long) =
        owner.brain.getMemory(MemoryModuleType.WALK_TARGET).isPresent
                || owner.brain.getMemory(MemoryModuleType.CANT_REACH_WALK_TARGET_SINCE).isEmpty
                || (closeAt != null && closeAt!! < gameTime)

    override fun tick(level: ServerLevel, owner: E, gameTime: Long) {
        if (closeAt == null) {
            if (owner.brain.getMemory(MemoryModuleType.WALK_TARGET).isEmpty) {
                // reached
                val chest = level.getBlockEntity(target!!) ?: return
                if (chest !is BaseContainerBlockEntity)
                    return
                if (chest is ChestBlockEntity) {
                    ChestBlockEntity.playSound(level, target!!, level.getBlockState(target!!), SoundEvents.CHEST_OPEN)
                }
                if (chest.isEmpty && level.random.nextBoolean()) {
                    closeAt = gameTime + 7
                } else {
                    closeAt = gameTime + 10 + level.random.nextInt(100)
                    exchangeItems(level, owner)
                }
            }
        }
    }

    override fun stop(level: ServerLevel, owner: E, gameTime: Long) {
        owner.brain.eraseMemory(MemoryModuleType.WALK_TARGET)
        owner.brain.eraseMemory(MemoryModuleType.CANT_REACH_WALK_TARGET_SINCE)
        if (closeAt != null) {
            // opened
            val chest = level.getBlockEntity(target!!)!!
            if (chest is ChestBlockEntity) {
                ChestBlockEntity.playSound(level, target!!, level.getBlockState(target!!), SoundEvents.CHEST_CLOSE)
            }
        }
    }

    private fun exchangeItems(level: ServerLevel, entity: E) {
        val container = level.getBlockEntity(target!!) ?: return
        if (container !is Container)
            return
        val inventory = entity.inventory
        for (i in 1..10) {
            val maxCount = 1 + level.random.nextInt(16)
            if (level.random.nextBoolean()) {
                // take
                val slot = level.random.nextInt(container.containerSize)
                val item = container.getItem(slot)
                if (!item.isEmpty) {
                    val takeCount = min(item.count, maxCount)
                    val takeItem = item.copyWithCount(takeCount)
                    if (entity.canHoldItem(takeItem)) {
                        val remaining = inventory.addItem(/*entity.equipItemIfPossible(takeItem)*/ takeItem)
                        val actualCount = takeCount - remaining.count
                        item.shrink(actualCount)
                        container.setItem(slot, item)
                    }
                }
            } else {
                // put
                val slot = level.random.nextInt(inventory.containerSize)
                val item = inventory.getItem(slot)
                if (!item.isEmpty) {
                    val takeCount = min(item.count, maxCount)
                    val takeItem = item.copyWithCount(takeCount)
                    for (target in 0 until container.containerSize) {
                        val targetItem = container.getItem(target)
                        if (ItemStack.isSameItemSameTags(targetItem, takeItem)) {
                            val resultCount = min(targetItem.count + takeItem.count, item.maxStackSize)
                            val putCount = resultCount - targetItem.count
                            if (putCount != 0) {
                                targetItem.grow(putCount)
                                container.setItem(target, targetItem)
                                takeItem.shrink(putCount)
                                if (takeItem.isEmpty) break
                            }
                        }
                    }
                    if (!takeItem.isEmpty) {
                        for (target in 0 until container.containerSize) {
                            val targetItem = container.getItem(target)
                            if (targetItem.isEmpty) {
                                container.setItem(target, takeItem.copyAndClear())
                                break
                            }
                        }
                    }
                    val putCount = takeCount - takeItem.count
                    item.shrink(putCount)
                    inventory.setItem(slot, item)
                }
            }
        }
        container.setChanged()
        inventory.setChanged()
    }

}
