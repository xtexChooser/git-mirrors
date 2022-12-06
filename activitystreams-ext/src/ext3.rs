use crate::Ext3;
use activitystreams::{
    activity::{
        Activity, ActivityActor, ActivityObject, AsActivity, AsActivityActor, AsActivityObject,
        AsOptOrigin, AsOptTarget, AsOrigin, AsQuestion, AsTarget, OptOrigin, OptTarget, Origin,
        Question, Target,
    },
    actor::{ApActor, AsApActor},
    base::{AsBase, Base},
    collection::{AsCollection, AsCollectionPage, Collection, CollectionPage},
    markers,
    object::{ApObject, AsApObject, AsObject, Object},
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

impl<Inner, A, B, C> AsBase for Ext3<Inner, A, B, C>
where
    Inner: AsBase,
{
    type Kind = Inner::Kind;

    fn base_ref(&self) -> &Base<Self::Kind> {
        self.inner.base_ref()
    }

    fn base_mut(&mut self) -> &mut Base<Self::Kind> {
        self.inner.base_mut()
    }
}

impl<Inner, A, B, C> AsObject for Ext3<Inner, A, B, C>
where
    Inner: AsObject,
{
    type Kind = Inner::Kind;

    fn object_ref(&self) -> &Object<Self::Kind> {
        self.inner.object_ref()
    }

    fn object_mut(&mut self) -> &mut Object<Self::Kind> {
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

impl<Inner, A, B, C> AsCollection for Ext3<Inner, A, B, C>
where
    Inner: AsCollection,
{
    type Kind = Inner::Kind;

    fn collection_ref(&self) -> &Collection<Self::Kind> {
        self.inner.collection_ref()
    }

    fn collection_mut(&mut self) -> &mut Collection<Self::Kind> {
        self.inner.collection_mut()
    }
}

impl<Inner, A, B, C> AsCollectionPage for Ext3<Inner, A, B, C>
where
    Inner: AsCollectionPage,
{
    type Kind = Inner::Kind;

    fn collection_page_ref(&self) -> &CollectionPage<Self::Kind> {
        self.inner.collection_page_ref()
    }

    fn collection_page_mut(&mut self) -> &mut CollectionPage<Self::Kind> {
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

impl<Inner, A, B, C> AsActivity for Ext3<Inner, A, B, C>
where
    Inner: AsActivity,
{
    type Kind = Inner::Kind;

    fn activity_ref(&self) -> &Activity<Self::Kind> {
        self.inner.activity_ref()
    }

    fn activity_mut(&mut self) -> &mut Activity<Self::Kind> {
        self.inner.activity_mut()
    }
}

impl<Inner, A, B, C> AsActivityActor for Ext3<Inner, A, B, C>
where
    Inner: AsActivityActor,
{
    type Inner = Inner::Inner;

    fn activity_actor_ref(&self) -> &ActivityActor<Self::Inner> {
        self.inner.activity_actor_ref()
    }

    fn activity_actor_mut(&mut self) -> &mut ActivityActor<Self::Inner> {
        self.inner.activity_actor_mut()
    }
}

impl<Inner, A, B, C> AsActivityObject for Ext3<Inner, A, B, C>
where
    Inner: AsActivityObject,
{
    type Inner = Inner::Inner;

    fn activity_object_ref(&self) -> &ActivityObject<Self::Inner> {
        self.inner.activity_object_ref()
    }

    fn activity_object_mut(&mut self) -> &mut ActivityObject<Self::Inner> {
        self.inner.activity_object_mut()
    }
}

impl<Inner, A, B, C> AsTarget for Ext3<Inner, A, B, C>
where
    Inner: AsTarget,
{
    type Inner = Inner::Inner;

    fn target_ref(&self) -> &Target<Self::Inner> {
        self.inner.target_ref()
    }

    fn target_mut(&mut self) -> &mut Target<Self::Inner> {
        self.inner.target_mut()
    }
}

impl<Inner, A, B, C> AsOrigin for Ext3<Inner, A, B, C>
where
    Inner: AsOrigin,
{
    type Inner = Inner::Inner;

    fn origin_ref(&self) -> &Origin<Self::Inner> {
        self.inner.origin_ref()
    }

    fn origin_mut(&mut self) -> &mut Origin<Self::Inner> {
        self.inner.origin_mut()
    }
}

impl<Inner, A, B, C> AsOptTarget for Ext3<Inner, A, B, C>
where
    Inner: AsOptTarget,
{
    type Inner = Inner::Inner;

    fn opt_target_ref(&self) -> &OptTarget<Self::Inner> {
        self.inner.opt_target_ref()
    }

    fn opt_target_mut(&mut self) -> &mut OptTarget<Self::Inner> {
        self.inner.opt_target_mut()
    }
}

impl<Inner, A, B, C> AsOptOrigin for Ext3<Inner, A, B, C>
where
    Inner: AsOptOrigin,
{
    type Inner = Inner::Inner;

    fn opt_origin_ref(&self) -> &OptOrigin<Self::Inner> {
        self.inner.opt_origin_ref()
    }

    fn opt_origin_mut(&mut self) -> &mut OptOrigin<Self::Inner> {
        self.inner.opt_origin_mut()
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
