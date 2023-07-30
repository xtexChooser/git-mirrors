package quaedam.misc

import net.minecraft.world.item.Item
import quaedam.Quaedam

object CraftingMaterials {

    val ironCopperMetal = Quaedam.items.register("iron_copper_metal") {
        Item(Item.Properties().`arch$tab`(Quaedam.creativeModeTab))
    }!!

    val projectionMetal = Quaedam.items.register("projection_metal") {
        Item(Item.Properties().`arch$tab`(Quaedam.creativeModeTab))
    }!!

}