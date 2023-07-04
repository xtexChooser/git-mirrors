package quaedam.projection

import com.mojang.brigadier.builder.LiteralArgumentBuilder.literal
import com.mojang.brigadier.builder.RequiredArgumentBuilder.argument
import com.mojang.brigadier.context.CommandContext
import dev.architectury.event.events.common.CommandRegistrationEvent
import net.minecraft.commands.CommandSourceStack
import net.minecraft.commands.arguments.ResourceArgument.resource
import net.minecraft.core.BlockPos
import net.minecraft.core.Holder
import net.minecraft.nbt.ListTag
import net.minecraft.nbt.NbtUtils
import quaedam.projector.Projector

object ProjectionCommand {

    init {
        CommandRegistrationEvent.EVENT.register { dispatcher, ctx, _ ->
            dispatcher.register(
                literal<CommandSourceStack>("quaedam_projection")
                    .then(
                        literal<CommandSourceStack>("dump")
                            .requires { it.hasPermission(2) }
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
                                    .executes(::get)
                            )
                    )
            )
        }
    }

    private fun dump(ctx: CommandContext<CommandSourceStack>): Int {
        val pos = BlockPos(
            ctx.source.position.x.toInt(),
            ctx.source.position.y.toInt(),
            ctx.source.position.z.toInt()
        )
        val data = Projector.findNearbyProjectors(ctx.source.level, pos)
            .map { ctx.source.level.getBlockEntity(it)!!.saveWithFullMetadata() }
        val tag = ListTag()
        tag.addAll(data)
        ctx.source.sendSystemMessage(NbtUtils.toPrettyComponent(tag))
        return 0
    }

    private fun get(ctx: CommandContext<CommandSourceStack>): Int {
        val pos = BlockPos(
            ctx.source.position.x.toInt(),
            ctx.source.position.y.toInt(),
            ctx.source.position.z.toInt()
        )
        val type = ctx.getArgument("type", Holder.Reference::class.java).value() as ProjectionEffectType<*>
        val data = Projector.findNearbyProjections(ctx.source.level, pos, type)
            .map { it.toNbt() }
        val tag = ListTag()
        tag.addAll(data)
        ctx.source.sendSystemMessage(NbtUtils.toPrettyComponent(tag))
        return 0
    }

}