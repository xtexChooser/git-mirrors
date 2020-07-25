use crate::Ext2;
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

impl<Inner, A, B> markers::Base for Ext2<Inner, A, B> where Inner: markers::Base {}
impl<Inner, A, B> markers::Object for Ext2<Inner, A, B> where Inner: markers::Object {}
impl<Inner, A, B> markers::Collection for Ext2<Inner, A, B> where Inner: markers::Collection {}
impl<Inner, A, B> markers::CollectionPage for Ext2<Inner, A, B> where Inner: markers::CollectionPage {}
impl<Inner, A, B> markers::Actor for Ext2<Inner, A, B> where Inner: markers::Actor {}
impl<Inner, A, B> markers::Activity for Ext2<Inner, A, B> where Inner: markers::Activity {}
impl<Inner, A, B> markers::IntransitiveActivity for Ext2<Inner, A, B> where
    Inner: markers::IntransitiveActivity
{
}

impl<Inner, A, B, Kind> AsBase<Kind> for Ext2<Inner, A, B>
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

impl<Inner, A, B, Kind> AsObject<Kind> for Ext2<Inner, A, B>
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

impl<Inner, A, B, ApInner> AsApObject<ApInner> for Ext2<Inner, A, B>
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

impl<Inner, A, B, Kind> AsCollection<Kind> for Ext2<Inner, A, B>
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

impl<Inner, A, B, Kind> AsCollectionPage<Kind> for Ext2<Inner, A, B>
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

impl<Inner, A, B, ApInner> AsApActor<ApInner> for Ext2<Inner, A, B>
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

impl<Inner, A, B, Kind> AsActivity<Kind> for Ext2<Inner, A, B>
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

impl<Inner, A, B> ActorAndObjectRef for Ext2<Inner, A, B>
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

impl<Inner, A, B> TargetRef for Ext2<Inner, A, B>
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

impl<Inner, A, B> OriginRef for Ext2<Inner, A, B>
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

impl<Inner, A, B> OptTargetRef for Ext2<Inner, A, B>
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

impl<Inner, A, B> OptOriginRef for Ext2<Inner, A, B>
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

impl<Inner, A, B> AsQuestion for Ext2<Inner, A, B>
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
