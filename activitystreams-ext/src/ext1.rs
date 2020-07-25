use crate::Ext1;
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

impl<Inner, A> markers::Base for Ext1<Inner, A> where Inner: markers::Base {}
impl<Inner, A> markers::Object for Ext1<Inner, A> where Inner: markers::Object {}
impl<Inner, A> markers::Collection for Ext1<Inner, A> where Inner: markers::Collection {}
impl<Inner, A> markers::CollectionPage for Ext1<Inner, A> where Inner: markers::CollectionPage {}
impl<Inner, A> markers::Actor for Ext1<Inner, A> where Inner: markers::Actor {}
impl<Inner, A> markers::Activity for Ext1<Inner, A> where Inner: markers::Activity {}
impl<Inner, A> markers::IntransitiveActivity for Ext1<Inner, A> where
    Inner: markers::IntransitiveActivity
{
}

impl<Inner, A, Kind> AsBase<Kind> for Ext1<Inner, A>
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

impl<Inner, A, Kind> AsObject<Kind> for Ext1<Inner, A>
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

impl<Inner, A, ApInner> AsApObject<ApInner> for Ext1<Inner, A>
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

impl<Inner, A, Kind> AsCollection<Kind> for Ext1<Inner, A>
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

impl<Inner, A, Kind> AsCollectionPage<Kind> for Ext1<Inner, A>
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

impl<Inner, A, ApInner> AsApActor<ApInner> for Ext1<Inner, A>
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

impl<Inner, A, Kind> AsActivity<Kind> for Ext1<Inner, A>
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

impl<Inner, A> ActorAndObjectRef for Ext1<Inner, A>
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

impl<Inner, A> TargetRef for Ext1<Inner, A>
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

impl<Inner, A> OriginRef for Ext1<Inner, A>
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

impl<Inner, A> OptTargetRef for Ext1<Inner, A>
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

impl<Inner, A> OptOriginRef for Ext1<Inner, A>
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

impl<Inner, A> AsQuestion for Ext1<Inner, A>
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
