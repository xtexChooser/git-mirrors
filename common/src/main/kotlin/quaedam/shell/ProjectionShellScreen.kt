package quaedam.shell

import dev.architectury.utils.GameInstance
import net.minecraft.client.gui.GuiGraphics
import net.minecraft.client.gui.components.Button
import net.minecraft.client.gui.components.StringWidget
import net.minecraft.client.gui.layouts.GridLayout
import net.minecraft.client.gui.screens.Screen
import net.minecraft.core.BlockPos
import net.minecraft.network.chat.Component
import net.minecraft.world.level.Level
import quaedam.shell.network.ServerboundPSHLockReleasePacket

class ProjectionShellScreen(val level: Level, val pos: BlockPos, val shell: ProjectionEffectShell) :
    Screen(Component.translatable("quaedam.screen.projection_shell")) {

    companion object {
        const val BORDER = 15
    }

    var layout = GridLayout()

    override fun init() {
        super.init()
        layout = GridLayout()
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
                GameInstance.getClient().setScreen(null)
            }.build())
        }
        layout.arrangeElements()
        layout.x = (width - layout.width) / 2
        layout.y = (height - layout.height) / 2
        layout.visitWidgets(::addRenderableWidget)
    }

    fun getFont() = font

    override fun render(guiGraphics: GuiGraphics, mouseX: Int, mouseY: Int, partialTick: Float) {
        renderBackground(guiGraphics)
        super.render(guiGraphics, mouseX, mouseY, partialTick)
    }

    override fun renderBackground(guiGraphics: GuiGraphics) {
        super.renderBackground(guiGraphics)
        guiGraphics.fill(
            layout.x - BORDER,
            layout.y - BORDER,
            layout.x + layout.width + BORDER,
            layout.y + layout.height + BORDER,
            0x11c6c6c6
        )
    }

    override fun removed() {
        super.removed()
        ProjectionShell.channel.sendToServer(ServerboundPSHLockReleasePacket(pos))
    }

}