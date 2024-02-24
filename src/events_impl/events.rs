use std::hash::{DefaultHasher, Hash, Hasher};

/// Type-safe value representing a unique ID for events. Defaults to the hash of the event type's
/// name, but can be overwritten for cases like enum events where each variant is a separate type.
#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub struct EventKind(u64);

/// A trait for types that are used as events to behavior tree nodes.
/// For the sake of testing, the unit type () is considered an event type.
pub trait Event {
    /// A unique string describing the type of the event.
    fn event_name(&self) -> &str;
}

pub trait EventExt {
    fn event_type(&self) -> EventKind;
}

impl<E: Event + ?Sized> EventExt for E {
    #[inline]
    fn event_type(&self) -> EventKind {
        let mut hasher = DefaultHasher::new();
        self.event_name().hash(&mut hasher);
        EventKind(hasher.finish())
    }
}

/// A event type whose unique ID is not dependent on the event's value (ie. non-enum events).
/// A type implementing [`EventType`] automatically implements [`Event`].
pub trait EventType {}

pub trait EventTypeExt {
    /// A unique value describing the type of the event.
    #[inline]
    fn static_event_name() -> &'static str {
        std::any::type_name::<Self>()
    }

    #[inline]
    fn static_event_type() -> EventKind {
        let mut hasher = DefaultHasher::new();
        std::any::type_name::<Self>().hash(&mut hasher);
        EventKind(hasher.finish())
    }
}

// just in case you don't want your own event type
impl EventType for () {}

/// An iterator that yields () infinitely, useful for easy testing behavior nodes.
#[derive(Clone)]
pub struct UnitEventPump;

impl Iterator for UnitEventPump {
    type Item = &'static dyn Event;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        Some(&())
    }
}

impl<E: EventType> EventTypeExt for E {}

impl<E: EventType> Event for E {
    #[inline]
    fn event_name(&self) -> &str {
        E::static_event_name()
    }
}