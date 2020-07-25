use crate::Ext3;
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

impl<Inner, A, B, C> markers::Base for Ext3<Inner, A, B, C> where Inner: markers::Base {}
impl<Inner, A, B, C> markers::Object for Ext3<Inner, A, B, C> where Inner: markers::Object {}
impl<Inner, A, B, C> markers::Collection for Ext3<Inner, A, B, C> where Inner: markers::Collection {}
impl<Inner, A, B, C> markers::CollectionPage for Ext3<Inner, A, B, C> where
    Inner: markers::CollectionPage
{
}
impl<Inner, A, B, C> markers::Actor for Ext3<Inner, A, B, C> where Inner: markers::Actor {}
impl<Inner, A, B, C> markers::Activity for Ext3<Inner, A, B, C> where Inner: markers::Activity {}
impl<Inner, A, B, C> markers::IntransitiveActivity for Ext3<Inner, A, B, C> where
    Inner: markers::IntransitiveActivity
{
}

impl<Inner, A, B, C, Kind> AsBase<Kind> for Ext3<Inner, A, B, C>
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

impl<Inner, A, B, C, Kind> AsObject<Kind> for Ext3<Inner, A, B, C>
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

impl<Inner, A, B, C, ApInner> AsApObject<ApInner> for Ext3<Inner, A, B, C>
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

impl<Inner, A, B, C, Kind> AsCollection<Kind> for Ext3<Inner, A, B, C>
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

impl<Inner, A, B, C, Kind> AsCollectionPage<Kind> for Ext3<Inner, A, B, C>
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

impl<Inner, A, B, C, ApInner> AsApActor<ApInner> for Ext3<Inner, A, B, C>
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

impl<Inner, A, B, C, Kind> AsActivity<Kind> for Ext3<Inner, A, B, C>
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

impl<Inner, A, B, C> ActorAndObjectRef for Ext3<Inner, A, B, C>
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

impl<Inner, A, B, C> TargetRef for Ext3<Inner, A, B, C>
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

impl<Inner, A, B, C> OriginRef for Ext3<Inner, A, B, C>
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

impl<Inner, A, B, C> OptTargetRef for Ext3<Inner, A, B, C>
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

impl<Inner, A, B, C> OptOriginRef for Ext3<Inner, A, B, C>
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

impl<Inner, A, B, C> AsQuestion for Ext3<Inner, A, B, C>
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
