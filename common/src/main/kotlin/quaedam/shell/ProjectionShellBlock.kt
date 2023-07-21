package quaedam.shell

import net.minecraft.core.BlockPos
import net.minecraft.world.level.Level

interface ProjectionShellBlock {

    fun getProjectionEffectForShell(level: Level, pos: BlockPos): ProjectionEffectShell

    fun applyFromShell(level: Level, pos: BlockPos, shell: ProjectionEffectShell)

}