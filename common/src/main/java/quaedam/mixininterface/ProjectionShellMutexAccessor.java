package quaedam.mixininterface;

import net.minecraft.core.GlobalPos;
import quaedam.shell.ProjectionShellMutex;

import java.util.LinkedHashMap;

public interface ProjectionShellMutexAccessor {

    LinkedHashMap<GlobalPos, ProjectionShellMutex.Lock> quaedam$getProjectionShellMutex();

}
