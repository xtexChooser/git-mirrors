package quaedam.projection.swarm.ai

import net.minecraft.world.entity.LivingEntity
import net.minecraft.world.entity.ai.behavior.OneShot
import net.minecraft.world.entity.ai.behavior.declarative.BehaviorBuilder
import net.minecraft.world.entity.ai.behavior.declarative.Trigger
import net.minecraft.world.entity.item.ItemEntity
import net.minecraft.world.entity.npc.InventoryCarrier
import net.minecraft.world.entity.schedule.Activity

@Suppress("FunctionName")
fun <E> LostItem(chance: Int): OneShot<E>
        where E : LivingEntity, E : InventoryCarrier = BehaviorBuilder.create { instance ->
    instance.point(Trigger { level, entity: E, l: Long ->
        if (entity.brain.isActive(Activity.REST)) return@Trigger false
        if (level.random.nextInt(chance) != 0) return@Trigger false
        val inventory = entity.inventory
        val item = inventory.getItem(level.random.nextInt(inventory.containerSize))
        if (!item.isEmpty) {
            val count = level.random.nextInt(item.count)
            item.shrink(count)
            inventory.setChanged()
            val itemEntity = ItemEntity(level, entity.x, entity.y + 0.25, entity.z, item.copyWithCount(count))
            itemEntity.setDefaultPickUpDelay()
            level.addFreshEntity(itemEntity)
            return@Trigger true
        } else {
            return@Trigger false
        }
    })
}