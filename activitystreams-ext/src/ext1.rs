use crate::Ext1;
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

impl<Inner, A> AsBase for Ext1<Inner, A>
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

impl<Inner, A> AsObject for Ext1<Inner, A>
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

impl<Inner, A> AsApObject for Ext1<Inner, A>
where
    Inner: AsApObject,
{
    type Inner = Inner::Inner;

    fn ap_object_ref(&self) -> &ApObject<Self::Inner> {
        self.inner.ap_object_ref()
    }

    fn ap_object_mut(&mut self) -> &mut ApObject<Self::Inner> {
        self.inner.ap_object_mut()
    }
}

impl<Inner, A> AsCollection for Ext1<Inner, A>
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

impl<Inner, A> AsCollectionPage for Ext1<Inner, A>
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

impl<Inner, A> AsApActor for Ext1<Inner, A>
where
    Inner: AsApActor,
{
    type Inner = Inner::Inner;

    fn ap_actor_ref(&self) -> &ApActor<Self::Inner> {
        self.inner.ap_actor_ref()
    }

    fn ap_actor_mut(&mut self) -> &mut ApActor<Self::Inner> {
        self.inner.ap_actor_mut()
    }
}

impl<Inner, A> AsActivity for Ext1<Inner, A>
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

impl<Inner, A> AsActivityActor for Ext1<Inner, A>
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

impl<Inner, A> AsActivityObject for Ext1<Inner, A>
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

impl<Inner, A> AsTarget for Ext1<Inner, A>
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

impl<Inner, A> AsOrigin for Ext1<Inner, A>
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

impl<Inner, A> AsOptTarget for Ext1<Inner, A>
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

impl<Inner, A> AsOptOrigin for Ext1<Inner, A>
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
