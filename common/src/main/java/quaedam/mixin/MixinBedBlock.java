package quaedam.mixin;

import net.minecraft.core.BlockPos;
import net.minecraft.world.entity.LivingEntity;
import net.minecraft.world.level.Level;
import net.minecraft.world.level.block.BedBlock;
import net.minecraft.world.phys.AABB;
import org.spongepowered.asm.mixin.Mixin;
import org.spongepowered.asm.mixin.injection.At;
import org.spongepowered.asm.mixin.injection.Inject;
import org.spongepowered.asm.mixin.injection.callback.CallbackInfoReturnable;
import quaedam.projection.swarm.ProjectedPersonEntity;

import java.util.List;

@Mixin(BedBlock.class)
public class MixinBedBlock {

    @Inject(at = @At("RETURN"), method = "kickVillagerOutOfBed(Lnet/minecraft/world/level/Level;Lnet/minecraft/core/BlockPos;)Z", cancellable = true)
    private void kickVillagerOutOfBed(Level level, BlockPos blockPos, CallbackInfoReturnable<Boolean> cir) {
        if (!cir.getReturnValueZ()) {
            List<ProjectedPersonEntity> list = level.getEntitiesOfClass(ProjectedPersonEntity.class, new AABB(blockPos), LivingEntity::isSleeping);
            if (!list.isEmpty()) {
                list.get(0).stopSleeping();
                cir.setReturnValue(true);
            }
        }
    }

}
