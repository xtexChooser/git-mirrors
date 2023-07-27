package quaedam.mixin;

import net.minecraft.network.Connection;
import net.minecraft.server.level.ServerPlayer;
import net.minecraft.server.players.PlayerList;
import org.spongepowered.asm.mixin.Mixin;
import org.spongepowered.asm.mixin.injection.At;
import org.spongepowered.asm.mixin.injection.Inject;
import org.spongepowered.asm.mixin.injection.callback.CallbackInfo;
import quaedam.config.SimpleQuaedamConfigPush;

@Mixin(PlayerList.class)
public class MixinPlayerList {

    @Inject(at = @At("RETURN"), method = "placeNewPlayer(Lnet/minecraft/network/Connection;Lnet/minecraft/server/level/ServerPlayer;)V")
    public void placeNewPlayer(Connection netManager, ServerPlayer player, CallbackInfo ci) {
        SimpleQuaedamConfigPush.INSTANCE.sendCurrent(player);
    }

}
