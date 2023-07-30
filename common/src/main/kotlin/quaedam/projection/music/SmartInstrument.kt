package quaedam.projection.music

import net.minecraft.core.BlockPos
import net.minecraft.nbt.CompoundTag
import net.minecraft.network.protocol.Packet
import net.minecraft.network.protocol.game.ClientGamePacketListener
import net.minecraft.network.protocol.game.ClientboundBlockEntityDataPacket
import net.minecraft.server.level.ServerLevel
import net.minecraft.util.RandomSource
import net.minecraft.world.InteractionHand
import net.minecraft.world.InteractionResult
import net.minecraft.world.entity.player.Player
import net.minecraft.world.item.BlockItem
import net.minecraft.world.item.Item
import net.minecraft.world.level.Level
import net.minecraft.world.level.block.Block
import net.minecraft.world.level.block.EntityBlock
import net.minecraft.world.level.block.entity.BlockEntity
import net.minecraft.world.level.block.entity.BlockEntityTicker
import net.minecraft.world.level.block.entity.BlockEntityType
import net.minecraft.world.level.block.state.BlockState
import net.minecraft.world.level.block.state.StateDefinition
import net.minecraft.world.level.block.state.properties.BlockStateProperties
import net.minecraft.world.level.block.state.properties.NoteBlockInstrument
import net.minecraft.world.level.material.MapColor
import net.minecraft.world.phys.BlockHitResult
import quaedam.Quaedam
import quaedam.misc.causality.CausalityAnchor
import quaedam.projector.Projector
import quaedam.utils.getChunksNearby
import quaedam.utils.sendBlockUpdated

object SmartInstrument {

    const val ID = "smart_instrument"

    val block = Quaedam.blocks.register(ID) { SmartInstrumentBlock }!!

    val item = Quaedam.items.register(ID) {
        BlockItem(
            SmartInstrumentBlock, Item.Properties()
                .`arch$tab`(Quaedam.creativeModeTab)
        )
    }!!

    val blockEntity = Quaedam.blockEntities.register(ID) {
        BlockEntityType.Builder.of(::SmartInstrumentBlockEntity, block.get()).build(null)
    }!!

}

object SmartInstrumentBlock : Block(
    Properties.of()
        .strength(2.7f)
        .requiresCorrectToolForDrops()
        .mapColor(MapColor.COLOR_BROWN)
        .randomTicks()
), EntityBlock {

    init {
        registerDefaultState(
            defaultBlockState()
                .setValue(BlockStateProperties.NOTEBLOCK_INSTRUMENT, NoteBlockInstrument.HARP)
        )
    }

    override fun newBlockEntity(pos: BlockPos, state: BlockState) = SmartInstrumentBlockEntity(pos, state)

    override fun createBlockStateDefinition(builder: StateDefinition.Builder<Block, BlockState>) {
        super.createBlockStateDefinition(builder)
        builder.add(BlockStateProperties.NOTEBLOCK_INSTRUMENT)
    }

    @Suppress("OVERRIDE_DEPRECATION", "DEPRECATION")
    override fun neighborChanged(
        state: BlockState,
        level: Level,
        pos: BlockPos,
        neighborBlock: Block,
        neighborPos: BlockPos,
        movedByPiston: Boolean
    ) {
        super.neighborChanged(state, level, pos, neighborBlock, neighborPos, movedByPiston)
        level.setBlock(
            pos,
            state.setValue(BlockStateProperties.NOTEBLOCK_INSTRUMENT, level.getBlockState(pos.below()).instrument()),
            UPDATE_ALL
        )
    }

    @Suppress("OVERRIDE_DEPRECATION", "DEPRECATION")
    override fun onPlace(state: BlockState, level: Level, pos: BlockPos, oldState: BlockState, movedByPiston: Boolean) {
        super.onPlace(state, level, pos, oldState, movedByPiston)
        level.setBlock(
            pos,
            state.setValue(BlockStateProperties.NOTEBLOCK_INSTRUMENT, level.getBlockState(pos.below()).instrument()),
            UPDATE_ALL
        )
    }

    @Suppress("OVERRIDE_DEPRECATION")
    override fun randomTick(
        state: BlockState,
        level: ServerLevel,
        pos: BlockPos,
        random: RandomSource
    ) {
        if (Projector.findNearbyProjections(level, pos, MusicProjection.effect.get()).isNotEmpty()) {
            val entity = level.getBlockEntity(pos) as SmartInstrumentBlockEntity
            if (entity.player == null) {
                entity.startMusic()
            }
        }
    }

    @Suppress("OVERRIDE_DEPRECATION", "DEPRECATION")
    override fun use(
        state: BlockState,
        level: Level,
        pos: BlockPos,
        player: Player,
        hand: InteractionHand,
        hit: BlockHitResult
    ): InteractionResult {
        if (Projector.findNearbyProjections(level, pos, MusicProjection.effect.get()).isNotEmpty()
            || CausalityAnchor.checkEffect(level, pos)
        ) {
            val entity = level.getBlockEntity(pos) as SmartInstrumentBlockEntity
            if (entity.player == null) {
                entity.startMusic()
            }
            return InteractionResult.SUCCESS
        }
        return super.use(state, level, pos, player, hand, hit)
    }

    override fun <T : BlockEntity?> getTicker(
        level: Level,
        state: BlockState,
        blockEntityType: BlockEntityType<T>
    ): BlockEntityTicker<T> {
        return BlockEntityTicker { _, _, _, entity ->
            (entity as? SmartInstrumentBlockEntity)?.tick()
        }
    }

}

class SmartInstrumentBlockEntity(pos: BlockPos, state: BlockState) :
    BlockEntity(SmartInstrument.blockEntity.get(), pos, state) {

    companion object {
        const val TAG_MUSIC = "Music"
    }

    var player: MusicPlayer? = null

    override fun getUpdateTag(): CompoundTag = saveWithoutMetadata()

    override fun getUpdatePacket(): Packet<ClientGamePacketListener> = ClientboundBlockEntityDataPacket.create(this)

    override fun load(tag: CompoundTag) {
        super.load(tag)
        if (TAG_MUSIC in tag) {
            player = MusicPlayer(tag.getCompound(TAG_MUSIC), level!!, blockPos)
        }
    }

    override fun saveAdditional(tag: CompoundTag) {
        super.saveAdditional(tag)
        if (player != null) {
            tag.put(TAG_MUSIC, player!!.toTag())
        }
    }

    private fun checkProjections() =
        Projector.findNearbyProjections(level!!, blockPos, MusicProjection.effect.get()).isNotEmpty()
                || CausalityAnchor.checkEffect(level!!, blockPos)

    fun startMusic(force: Boolean = false, synced: Boolean = false) {
        if ((player == null || force) && !level!!.isClientSide && checkProjections()) {
            player = MusicPlayer(level!!.random.nextLong(), level!!, blockPos)
            setChanged()
            sendBlockUpdated()
            if (!synced) {
                // sync start to other instruments
                level!!.getChunksNearby(blockPos, 1)
                    .flatMap {
                        it.blockEntities
                            .filterValues { entity -> entity is SmartInstrumentBlockEntity }
                            .filterKeys { pos -> pos.distSqr(blockPos) < 100 }
                            .values
                    }
                    .filterNot { it == this }
                    .filterIsInstance<SmartInstrumentBlockEntity>()
                    .forEach { it.startMusic(force = true, synced = true) }
            }
        }
    }

    fun tick() {
        if (checkProjections()) {
            player?.tick()
            if (!level!!.isClientSide) {
                if (player?.isEnd == true) {
                    player = null
                    setChanged()
                    sendBlockUpdated()
                    if (CausalityAnchor.checkEffect(level!!, blockPos) || level!!.random.nextInt(7) != 0) {
                        startMusic()
                    }
                }
            }
        } else {
            player = null
            setChanged()
            sendBlockUpdated()
        }
    }

}
