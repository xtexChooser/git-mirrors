package quaedam.projection.swarm

import com.mojang.blaze3d.vertex.PoseStack
import net.minecraft.client.model.PlayerModel
import net.minecraft.client.model.geom.ModelLayers
import net.minecraft.client.renderer.entity.EntityRendererProvider
import net.minecraft.client.renderer.entity.MobRenderer
import net.minecraft.client.renderer.entity.layers.CustomHeadLayer
import net.minecraft.client.renderer.entity.layers.ItemInHandLayer

class ProjectedPersonRenderer(context: EntityRendererProvider.Context) :
    MobRenderer<ProjectedPersonEntity, PlayerModel<ProjectedPersonEntity>>(
        context,
        PlayerModel(context.bakeLayer(ModelLayers.PLAYER), false),
        0.4f
    ) {

    init {
        addLayer(CustomHeadLayer(this, context.modelSet, context.itemInHandRenderer))
        addLayer(ItemInHandLayer(this, context.itemInHandRenderer))
    }

    override fun getTextureLocation(entity: ProjectedPersonEntity) = ProjectedPersonShape.Skins[entity.shape.skin]

    override fun scale(entity: ProjectedPersonEntity, poseStack: PoseStack, f: Float) {
        poseStack.scale(entity.shape.scaleX, entity.shape.scaleY, entity.shape.scaleZ)
        super.scale(entity, poseStack, f)
    }

}