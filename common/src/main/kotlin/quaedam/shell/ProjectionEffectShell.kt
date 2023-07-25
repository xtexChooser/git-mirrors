package quaedam.shell

import net.minecraft.client.gui.components.AbstractSliderButton
import net.minecraft.client.gui.components.CycleButton
import net.minecraft.client.gui.components.StringWidget
import net.minecraft.client.gui.layouts.LayoutElement
import net.minecraft.network.chat.Component
import quaedam.projection.ProjectionEffect
import kotlin.math.floor
import kotlin.reflect.KMutableProperty0

class ProjectionEffectShell(val effect: ProjectionEffect) {

    interface Provider {

        fun createShell(): ProjectionEffectShell

    }

    val rows = mutableListOf<Row>()
    val width = 150
    val height = 20

    data class Row(val text: Component, val renderer: ShellRenderer)

    fun row(key: String, renderer: ShellRenderer) {
        rows += Row(Component.translatable(key), renderer)
    }

    fun text(key: String, value: Component) = row(key) { StringWidget(value, it.font) }

    fun doubleSlider(key: String, property: KMutableProperty0<Double>, range: ClosedRange<Double>, step: Double) {
        val len = range.endInclusive - range.start
        val step = step / len
        row(key) {
            object : AbstractSliderButton(
                0, 0, width, height,
                Component.literal(property.get().toString()), (property.get() - range.start).toDouble() / len
            ) {
                override fun updateMessage() {
                    message = Component.literal(value.toString())
                }

                override fun applyValue() {
                    value = floor(value / step) * step
                    property.set(range.start + floor(value * len))
                }
            }
        }
    }

    fun intSlider(key: String, property: KMutableProperty0<Int>, range: IntProgression) {
        val len = range.last - range.first
        val step = range.step / len
        row(key) {
            object : AbstractSliderButton(
                0, 0, width, height,
                Component.literal(property.get().toString()), (property.get() - range.first).toDouble() / len
            ) {
                override fun updateMessage() {
                    message = Component.literal(value.toString())
                }

                override fun applyValue() {
                    value = floor(value / step) * step
                    property.set((range.first + floor(value * len)).toInt())
                }
            }
        }
    }

    fun intCycle(key: String, property: KMutableProperty0<Int>, range: IntProgression) =
        row(key) {
            CycleButton.builder<Int> { Component.literal(it.toString()) }
                .displayOnlyValue()
                .withValues(range.toList())
                .create(0, 0, width, height, Component.translatable(key))
        }

}

inline fun buildProjectionEffectShell(effect: ProjectionEffect, builder: ProjectionEffectShell.() -> Unit) =
    ProjectionEffectShell(effect).apply(builder)

data class ShellRenderContext(val screen: ProjectionShellScreen) {
    val font get() = screen.getFont()
}

typealias ShellRenderer = (ctx: ShellRenderContext) -> LayoutElement
