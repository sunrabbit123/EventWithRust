use std::collections::HashMap;

use crate::EventEmitter;

impl Default for EventEmitter {
    #[inline]
    fn default() -> Self {
        EventEmitter {
            using_domains: false,
            default_max_listeners: 10,
            events: Default::default(),
        }
    }
}

impl Clone for EventEmitter {
    fn clone(&self) -> Self {
        let mut events = HashMap::new();
        events.clear();
        events.extend(self.events.into_iter());
        Self { using_domains: self.using_domains.clone(), default_max_listeners: self.default_max_listeners.clone(), events }
    }
}

impl EventEmitter {
    pub fn event_emitter(self) -> Self {
        self
    }

    pub fn init(opts: Option<bool>) -> Self {
        let mut emitter: Self = Default::default();

        if let Some(is_using_domain) = opts {
            emitter.using_domains = is_using_domain;
        }

        emitter
    }
}
