use crate::EventEmitter;

impl Default for EventEmitter {
    #[inline]
    fn default() -> Self {
        EventEmitter {
            using_domains: false,
            default_max_listeners: 10,
        }
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
