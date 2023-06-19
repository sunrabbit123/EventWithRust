#[macro_use]
extern crate napi_derive;

use std::collections::HashMap;

use internal::js_function::call_event;
use napi::{Env, JsFunction, JsObject};

pub mod internal;

#[napi]
pub struct EventEmitter {
    pub using_domains: bool,
    pub default_max_listeners: i32,
    pub events: HashMap<String, Vec<JsFunction>>,
}

#[napi]
impl EventEmitter {
    #[napi(ts_args_type = "name : string, listener : (...args : any[]) => void")]
    pub fn add_listener(&mut self, name: &str, listener: JsFunction, prepend: Option<bool>) -> &mut EventEmitter {
        match self.events.get_mut(name) {
            Some(events) => {
                if let Some(true) = prepend {
                    events.insert(0, listener);
                } else {
                    events.push(listener);
                }
            }
            None => {
                self.events.insert(name.to_string(), vec![listener]);
            }
        }

        self
    }

    #[napi]
    pub fn emit(&self, name: &str, data: Vec<JsObject>) -> bool {
        let events_option = self.events.get(name);

        if let Some(events) = events_option {
            let _ = events.iter().map(|f| f.call(None, &data));
        }
        events_option.is_some()
    }

    #[napi]
    pub fn events_names(&self) -> Vec<&String> {
        self.events.keys().collect()
    }

    #[napi]
    pub fn listener_count(&self, name: &str) -> usize {
        self.events.get(name).map(|v| v.len()).unwrap_or_else(|| 0)
    }

    #[napi]
    pub fn set_max_listeners(&mut self, n: i32) {
        self.default_max_listeners = n;
    }

    #[napi]
    pub fn get_max_listeners(&self) -> i32 {
        self.default_max_listeners
    }

    #[napi]
    pub fn off(&mut self, js_env: Env, name: &String, listener: JsFunction) -> &mut EventEmitter {
        let event_name = js_env.create_string(name).unwrap();
        self.events.remove(name);
        if let Some(events) = self.events.get("removeListener") {
            events.iter().map(|f| call_event(&f, None, event_name, listener));
        }

        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn emit_data() {
        let mut e = EventEmitter::new();

        e.on("click", |data: &mut dyn Any| {
            let d = &mut data.downcast_mut::<Args>().unwrap();
            d.x = 10;
        });

        e.on("click", |data: &mut dyn Any| {
            let d = &mut data.downcast_mut::<Args>().unwrap();
            d.y = 20;
        });

        struct Args {
            pub x: usize,
            pub y: usize,
        }

        let args = &mut Args { x: 1, y: 2 };

        assert_eq!(args.x, 1);
        assert_eq!(args.y, 2);

        e.emit("click", args);

        assert_eq!(args.x, 10);
        assert_eq!(args.y, 20);
        assert_eq!(e.listeners("click").unwrap().len(), 2);
    }

    #[test]
    fn emit_inline() {
        let mut e = EventEmitter::new();

        e.on("click", |data: &mut dyn Any| {
            let d = &mut data.downcast_mut::<Args>().unwrap();
            assert_eq!(d.x, 1);
        });

        e.on("click", |data: &mut dyn Any| {
            let d = &mut data.downcast_mut::<Args>().unwrap();
            assert_eq!(d.y, 2);
        });

        struct Args {
            pub x: usize,
            pub y: usize,
        }

        e.emit("click", &mut Args { x: 1, y: 2 });
    }

    #[test]
    fn off() {
        let mut e = EventEmitter::new();

        e.on("click", |data: &mut dyn Any| {
            let d = &mut data.downcast_mut::<Args>().unwrap();
            d.x = d.x + 1;
        });

        struct Args {
            pub x: usize,
        }

        let args = &mut Args { x: 0 };

        e.emit("click", args);
        assert_eq!(args.x, 1);
        e.emit("click", args);
        assert_eq!(args.x, 2);
        e.off("click");
        e.emit("click", args);
        assert_eq!(args.x, 2);
    }
}
