package quaedam.mixin;

import net.minecraft.core.registries.BuiltInRegistries;
import org.spongepowered.asm.mixin.Mixin;
import org.spongepowered.asm.mixin.injection.At;
import org.spongepowered.asm.mixin.injection.Inject;
import org.spongepowered.asm.mixin.injection.callback.CallbackInfo;
import quaedam.projection.ProjectionEffectType;

@Mixin(BuiltInRegistries.class)
public class MixinBuiltInRegistries {

    @Inject(at = @At("HEAD"), method = "bootStrap()V")
    private static void bootStrap(CallbackInfo info) {
        // init projection effect type registry
        ProjectionEffectType.Companion.getRegistry();
    }

}
