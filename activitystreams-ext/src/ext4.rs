use crate::Ext4;
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

impl<Inner, A, B, C, D> AsBase for Ext4<Inner, A, B, C, D>
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

impl<Inner, A, B, C, D> AsObject for Ext4<Inner, A, B, C, D>
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

impl<Inner, A, B, C, D> AsApObject for Ext4<Inner, A, B, C, D>
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

impl<Inner, A, B, C, D> AsCollection for Ext4<Inner, A, B, C, D>
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

impl<Inner, A, B, C, D> AsCollectionPage for Ext4<Inner, A, B, C, D>
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

impl<Inner, A, B, C, D> AsApActor for Ext4<Inner, A, B, C, D>
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

impl<Inner, A, B, C, D> AsActivity for Ext4<Inner, A, B, C, D>
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

impl<Inner, A, B, C, D> AsActivityActor for Ext4<Inner, A, B, C, D>
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

impl<Inner, A, B, C, D> AsActivityObject for Ext4<Inner, A, B, C, D>
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

impl<Inner, A, B, C, D> AsTarget for Ext4<Inner, A, B, C, D>
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

impl<Inner, A, B, C, D> AsOrigin for Ext4<Inner, A, B, C, D>
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

impl<Inner, A, B, C, D> AsOptTarget for Ext4<Inner, A, B, C, D>
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

impl<Inner, A, B, C, D> AsOptOrigin for Ext4<Inner, A, B, C, D>
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
