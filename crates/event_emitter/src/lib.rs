pub struct EventEmitter{
  pub using_domains : bool,
  pub default_max_listeners : i32,
}

impl Default for EventEmitter {
  fn default() -> Self { EventEmitter { using_domains: false, default_max_listeners: 10 }}
}

impl EventEmitter {
  pub fn event_emitter(mut self) -> Self {
    self
  }

  pub fn init(opts : Option<bool>) -> Self {
    let mut emitter : Self = Default::default();
    opts.map(|v| emitter.using_domains = v);

    emitter
  }
}

#[cfg(test)]
mod event_emitter_test {
  #[test]
  fn it_works() {
    let result = 2 + 2;
    assert_eq!(result, 4);
  }
}
