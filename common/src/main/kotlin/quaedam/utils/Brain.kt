package quaedam.utils

import com.mojang.datafixers.util.Pair
import net.minecraft.world.entity.LivingEntity
import net.minecraft.world.entity.ai.behavior.BehaviorControl

@Suppress("NOTHING_TO_INLINE")
inline infix fun <B : BehaviorControl<E>, E : LivingEntity> Int.weight(behavior: B): Pair<Int, B> =
    Pair.of(this, behavior)

@Suppress("NOTHING_TO_INLINE")
inline infix fun <B, E> Int.weightR(behavior: B): Pair<B, Int>
        where B : BehaviorControl<E>, E : LivingEntity =
    Pair.of(behavior, this)
