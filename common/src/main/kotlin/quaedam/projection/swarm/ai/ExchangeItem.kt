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
    }

    override fun canStillUse(level: ServerLevel, entity: E, l: Long) =
        entity.brain.getMemory(MemoryModuleType.WALK_TARGET).isPresent || entity.brain.getMemory(MemoryModuleType.CANT_REACH_WALK_TARGET_SINCE).isEmpty || (closeAt != null && closeAt!! < l)

    override fun tick(level: ServerLevel, entity: E, l: Long) {
        if (closeAt == null) {
            if (entity.brain.getMemory(MemoryModuleType.WALK_TARGET).isEmpty) {
                // reached
                val chest = level.getBlockEntity(target!!) as BaseContainerBlockEntity
                if (chest is ChestBlockEntity) {
                    ChestBlockEntity.playSound(level, target!!, level.getBlockState(target!!), SoundEvents.CHEST_OPEN)
                }
                if (chest.isEmpty) {
                    closeAt = l + 7
                } else {
                    closeAt = l + 10 + level.random.nextInt(100)
                    exchangeItems(level, entity)
                }
            }
        }
    }

    override fun stop(level: ServerLevel, entity: E, l: Long) {
        entity.brain.eraseMemory(MemoryModuleType.WALK_TARGET)
        if (closeAt != null) {
            // opened
            val chest = level.getBlockEntity(target!!)!!
            if (chest is ChestBlockEntity) {
                ChestBlockEntity.playSound(level, target!!, level.getBlockState(target!!), SoundEvents.CHEST_CLOSE)
            }
        }
    }

    private fun exchangeItems(level: ServerLevel, entity: E) {
        val container = level.getBlockEntity(target!!) as Container
        val inventory = entity.inventory
        for (i in 1..6) {
            val maxCount = level.random.nextInt(16)
            if (level.random.nextBoolean()) {
                // take
                val slot = level.random.nextInt(container.containerSize)
                val item = container.getItem(slot)
                if (!item.isEmpty) {
                    val takeCount = min(item.count, maxCount)
                    val takeItem = item.copyWithCount(takeCount)
                    if (inventory.canTakeItem(container, slot, takeItem) && entity.canHoldItem(takeItem)) {
                        val remaining = entity.inventory.addItem(entity.equipItemIfPossible(takeItem))
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
                    if (container.canTakeItem(inventory, slot, takeItem)) {
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
                                    println("put all at $target")
                                    break
                                }
                            }
                        }
                        val putCount = takeCount - takeItem.count
                        println("put $putCount")
                        item.shrink(putCount)
                        container.setItem(slot, item)
                    }
                }
            }
        }
    }

}
