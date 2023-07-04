package quaedam.projection

import com.mojang.brigadier.arguments.StringArgumentType.string
import com.mojang.brigadier.builder.LiteralArgumentBuilder.literal
import com.mojang.brigadier.builder.RequiredArgumentBuilder.argument
import com.mojang.brigadier.context.CommandContext
import dev.architectury.event.events.common.CommandRegistrationEvent
import net.minecraft.commands.CommandSourceStack
import net.minecraft.commands.arguments.ResourceArgument.resource
import net.minecraft.core.BlockPos
import net.minecraft.core.Holder
import net.minecraft.network.chat.Component
import quaedam.projector.Projector
import java.util.*

object ProjectionCommand {

    init {
        CommandRegistrationEvent.EVENT.register { dispatcher, ctx, _ ->
            dispatcher.register(
                literal<CommandSourceStack>("quaedam_projection")
                    .then(
                        literal<CommandSourceStack>("dump")
                            .requires { it.hasPermission(2) }
                            .then(
                                argument<CommandSourceStack, String>("path", string())
                                    .executes(::dumpPath)
                            )
                            .executes(::dump)
                    )
                    .then(
                        literal<CommandSourceStack>("get")
                            .requires { it.hasPermission(2) }
                            .then(
                                argument<CommandSourceStack, Holder.Reference<ProjectionEffectType<*>>>(
                                    "type",
                                    resource(ctx, ProjectionEffectType.registryKey)
                                )
                                    .then(
                                        argument<CommandSourceStack, String>("path", string())
                                            .executes(::getPath)
                                    )
                                    .executes(::get)
                            )
                    )
            )
        }
    }

    private fun dump(ctx: CommandContext<CommandSourceStack>, path: String = ""): Int {
        val pos = BlockPos(
            ctx.source.position.x.toInt(),
            ctx.source.position.y.toInt(),
            ctx.source.position.z.toInt()
        )
        val data = Projector.findNearbyProjectors(ctx.source.level, pos)
            .map { ctx.source.level.getBlockEntity(it)!!.saveWithFullMetadata() }
        ctx.source.sendSystemMessage(Component.nbt(path, true, Optional.empty()) { data.stream() })
        return 0
    }

    private fun dumpPath(ctx: CommandContext<CommandSourceStack>) =
        dump(ctx, path = ctx.getArgument("path", String::class.java))

    private fun get(ctx: CommandContext<CommandSourceStack>, path: String = ""): Int {
        val pos = BlockPos(
            ctx.source.position.x.toInt(),
            ctx.source.position.y.toInt(),
            ctx.source.position.z.toInt()
        )
        val type = ctx.getArgument("type", Holder.Reference::class.java).value() as ProjectionEffectType<*>
        val data = Projector.findNearbyProjections(ctx.source.level, pos, type)
            .map { it.toNbt() }
        ctx.source.sendSystemMessage(Component.nbt(path, true, Optional.empty()) { data.stream() })
        return 0
    }

    private fun getPath(ctx: CommandContext<CommandSourceStack>) =
        dump(ctx, path = ctx.getArgument("path", String::class.java))

}