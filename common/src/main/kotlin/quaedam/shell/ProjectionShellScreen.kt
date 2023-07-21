package quaedam.shell

import net.minecraft.client.gui.GuiGraphics
import net.minecraft.client.gui.components.Button
import net.minecraft.client.gui.components.StringWidget
import net.minecraft.client.gui.layouts.GridLayout
import net.minecraft.client.gui.screens.Screen
import net.minecraft.client.gui.screens.inventory.AbstractContainerScreen
import net.minecraft.core.BlockPos
import net.minecraft.network.chat.Component
import net.minecraft.world.level.Level
import quaedam.shell.network.ServerboundPSHLockReleasePacket

class ProjectionShellScreen(val level: Level, val pos: BlockPos, val shell: ProjectionEffectShell) :
    Screen(Component.translatable("quaedam.screen.projection_shell")) {

    val layout = GridLayout()

    override fun init() {
        super.init()
        layout.spacing(4)
        val rows = layout.createRowHelper(2)
        val renderContext = ShellRenderContext(this)
        shell.rows.forEach {
            rows.addChild(StringWidget(150, 20, it.text, font))
            rows.addChild(it.renderer(renderContext))
        }
        run { // Buttons
            rows.addChild(StringWidget(Component.empty(), font))
            rows.addChild(Button.builder(Component.translatable("quaedam.screen.projection_shell.save")) {
                val block = level.getBlockState(pos).block
                if (block is ProjectionShellBlock) {
                    block.applyFromShell(level, pos, shell)
                }
            }.build())
        }
        layout.arrangeElements()
        layout.visitWidgets(::addRenderableWidget)
    }

    fun getFont() = font

    override fun renderBackground(guiGraphics: GuiGraphics) {
        super.renderBackground(guiGraphics)
        guiGraphics.blit(AbstractContainerScreen.INVENTORY_LOCATION, width / 2, height / 2, 0, 0, 176, 166)
    }

    override fun removed() {
        super.removed()
        ProjectionShell.channel.sendToServer(ServerboundPSHLockReleasePacket(pos))
    }

}