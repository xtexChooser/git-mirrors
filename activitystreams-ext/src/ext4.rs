use crate::Ext4;
use activitystreams::{
    activity::{
        Activity, ActorAndObjectRef, AsActivity, AsQuestion, OptOriginRef, OptTargetRef, OriginRef,
        Question, TargetRef,
    },
    actor::{ApActor, AsApActor},
    base::{AnyBase, AsBase, Base},
    collection::{AsCollection, AsCollectionPage, Collection, CollectionPage},
    markers,
    object::{ApObject, AsApObject, AsObject, Object},
    primitives::OneOrMany,
};

impl<Inner, A, B, C, D> markers::Base for Ext4<Inner, A, B, C, D> where Inner: markers::Base {}
impl<Inner, A, B, C, D> markers::Object for Ext4<Inner, A, B, C, D> where Inner: markers::Object {}
impl<Inner, A, B, C, D> markers::Collection for Ext4<Inner, A, B, C, D> where
    Inner: markers::Collection
{
}
impl<Inner, A, B, C, D> markers::CollectionPage for Ext4<Inner, A, B, C, D> where
    Inner: markers::CollectionPage
{
}
impl<Inner, A, B, C, D> markers::Actor for Ext4<Inner, A, B, C, D> where Inner: markers::Actor {}
impl<Inner, A, B, C, D> markers::Activity for Ext4<Inner, A, B, C, D> where Inner: markers::Activity {}
impl<Inner, A, B, C, D> markers::IntransitiveActivity for Ext4<Inner, A, B, C, D> where
    Inner: markers::IntransitiveActivity
{
}

impl<Inner, A, B, C, D, Kind> AsBase<Kind> for Ext4<Inner, A, B, C, D>
where
    Inner: AsBase<Kind>,
{
    fn base_ref(&self) -> &Base<Kind> {
        self.inner.base_ref()
    }

    fn base_mut(&mut self) -> &mut Base<Kind> {
        self.inner.base_mut()
    }
}

impl<Inner, A, B, C, D, Kind> AsObject<Kind> for Ext4<Inner, A, B, C, D>
where
    Inner: AsObject<Kind>,
{
    fn object_ref(&self) -> &Object<Kind> {
        self.inner.object_ref()
    }

    fn object_mut(&mut self) -> &mut Object<Kind> {
        self.inner.object_mut()
    }
}

impl<Inner, A, B, C, D, ApInner> AsApObject<ApInner> for Ext4<Inner, A, B, C, D>
where
    Inner: AsApObject<ApInner>,
{
    fn ap_object_ref(&self) -> &ApObject<ApInner> {
        self.inner.ap_object_ref()
    }

    fn ap_object_mut(&mut self) -> &mut ApObject<ApInner> {
        self.inner.ap_object_mut()
    }
}

impl<Inner, A, B, C, D, Kind> AsCollection<Kind> for Ext4<Inner, A, B, C, D>
where
    Inner: AsCollection<Kind>,
{
    fn collection_ref(&self) -> &Collection<Kind> {
        self.inner.collection_ref()
    }

    fn collection_mut(&mut self) -> &mut Collection<Kind> {
        self.inner.collection_mut()
    }
}

impl<Inner, A, B, C, D, Kind> AsCollectionPage<Kind> for Ext4<Inner, A, B, C, D>
where
    Inner: AsCollectionPage<Kind>,
{
    fn collection_page_ref(&self) -> &CollectionPage<Kind> {
        self.inner.collection_page_ref()
    }

    fn collection_page_mut(&mut self) -> &mut CollectionPage<Kind> {
        self.inner.collection_page_mut()
    }
}

impl<Inner, A, B, C, D, ApInner> AsApActor<ApInner> for Ext4<Inner, A, B, C, D>
where
    Inner: AsApActor<ApInner>,
{
    fn ap_actor_ref(&self) -> &ApActor<ApInner> {
        self.inner.ap_actor_ref()
    }

    fn ap_actor_mut(&mut self) -> &mut ApActor<ApInner> {
        self.inner.ap_actor_mut()
    }
}

impl<Inner, A, B, C, D, Kind> AsActivity<Kind> for Ext4<Inner, A, B, C, D>
where
    Inner: AsActivity<Kind>,
{
    fn activity_ref(&self) -> &Activity<Kind> {
        self.inner.activity_ref()
    }

    fn activity_mut(&mut self) -> &mut Activity<Kind> {
        self.inner.activity_mut()
    }
}

impl<Inner, A, B, C, D> ActorAndObjectRef for Ext4<Inner, A, B, C, D>
where
    Inner: ActorAndObjectRef,
{
    fn actor_field_ref(&self) -> &OneOrMany<AnyBase> {
        self.inner.actor_field_ref()
    }

    fn actor_field_mut(&mut self) -> &mut OneOrMany<AnyBase> {
        self.inner.actor_field_mut()
    }

    fn object_field_ref(&self) -> &OneOrMany<AnyBase> {
        self.inner.object_field_ref()
    }

    fn object_field_mut(&mut self) -> &mut OneOrMany<AnyBase> {
        self.inner.object_field_mut()
    }
}

impl<Inner, A, B, C, D> TargetRef for Ext4<Inner, A, B, C, D>
where
    Inner: TargetRef,
{
    fn target_field_ref(&self) -> &OneOrMany<AnyBase> {
        self.inner.target_field_ref()
    }

    fn target_field_mut(&mut self) -> &mut OneOrMany<AnyBase> {
        self.inner.target_field_mut()
    }
}

impl<Inner, A, B, C, D> OriginRef for Ext4<Inner, A, B, C, D>
where
    Inner: OriginRef,
{
    fn origin_field_ref(&self) -> &OneOrMany<AnyBase> {
        self.inner.origin_field_ref()
    }

    fn origin_field_mut(&mut self) -> &mut OneOrMany<AnyBase> {
        self.inner.origin_field_mut()
    }
}

impl<Inner, A, B, C, D> OptTargetRef for Ext4<Inner, A, B, C, D>
where
    Inner: OptTargetRef,
{
    fn target_field_ref(&self) -> &Option<OneOrMany<AnyBase>> {
        self.inner.target_field_ref()
    }

    fn target_field_mut(&mut self) -> &mut Option<OneOrMany<AnyBase>> {
        self.inner.target_field_mut()
    }
}

impl<Inner, A, B, C, D> OptOriginRef for Ext4<Inner, A, B, C, D>
where
    Inner: OptOriginRef,
{
    fn origin_field_ref(&self) -> &Option<OneOrMany<AnyBase>> {
        self.inner.origin_field_ref()
    }

    fn origin_field_mut(&mut self) -> &mut Option<OneOrMany<AnyBase>> {
        self.inner.origin_field_mut()
    }
}

impl<Inner, A, B, C, D> AsQuestion for Ext4<Inner, A, B, C, D>
where
    Inner: AsQuestion,
{
    fn question_ref(&self) -> &Question {
        self.inner.question_ref()
    }

    fn question_mut(&mut self) -> &mut Question {
        self.inner.question_mut()
    }
}
