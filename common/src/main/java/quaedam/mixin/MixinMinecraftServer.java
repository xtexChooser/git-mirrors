package quaedam.mixin;

import net.minecraft.core.GlobalPos;
import net.minecraft.server.MinecraftServer;
import org.spongepowered.asm.mixin.Mixin;
import quaedam.mixininterface.ProjectionShellMutexAccessor;
import quaedam.shell.ProjectionShellMutex;

import java.util.LinkedHashMap;

@Mixin(MinecraftServer.class)
public class MixinMinecraftServer implements ProjectionShellMutexAccessor {

    private LinkedHashMap<GlobalPos, ProjectionShellMutex.Lock> quaedam$projectionShellMutex;

    @Override
    public LinkedHashMap<GlobalPos, ProjectionShellMutex.Lock> quaedam$getProjectionShellMutex() {
        if (quaedam$projectionShellMutex == null) {
            quaedam$projectionShellMutex = new LinkedHashMap<>();
        }
        return quaedam$projectionShellMutex;
    }

}
