package quaedam.mixin;

import net.minecraft.client.multiplayer.ClientLevel;
import net.minecraft.core.BlockPos;
import net.minecraft.world.phys.Vec3;
import org.joml.Math;
import org.spongepowered.asm.mixin.Mixin;
import org.spongepowered.asm.mixin.injection.At;
import org.spongepowered.asm.mixin.injection.Inject;
import org.spongepowered.asm.mixin.injection.callback.CallbackInfoReturnable;
import quaedam.projection.SkylightProjection;
import quaedam.projection.SkylightProjectionEffect;
import quaedam.projector.Projector;

import java.util.List;

@Mixin(ClientLevel.class)
public class MixinClientLevel {

    @Inject(at = @At("RETURN"), method = "getSkyColor(Lnet/minecraft/world/phys/Vec3;F)Lnet/minecraft/world/phys/Vec3;", cancellable = true)
    public void getSkyColor(Vec3 pos, float f, CallbackInfoReturnable<Vec3> cir) {
        ClientLevel this0 = (ClientLevel) (Object) this;
        List<SkylightProjectionEffect> projections = Projector.INSTANCE.findNearbyProjections(this0,
                new BlockPos((int) pos.x, (int) pos.y, (int) pos.z), SkylightProjection.INSTANCE.getEffect().get());
        if (!projections.isEmpty()) {
            Vec3 color = cir.getReturnValue();
            if (color.x == 0 || color.y == 0 || color.z == 0) {
                // scale compensate
                color = color.add(0.1, 0.1, 0.1);
            }
            for (SkylightProjectionEffect effect : projections) {
                double factor = effect.getFactor();
                color = color.multiply(factor, factor, factor);
            }
            color = new Vec3(Math.min(color.x, 1.0), Math.min(color.y, 1.0), Math.min(color.z, 1.0));
            cir.setReturnValue(color);
        }
    }

}
