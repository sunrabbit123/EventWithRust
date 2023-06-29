use std::ptr;

use napi::{
    check_pending_exception, sys, Env, Error, JsFunction, JsObject, JsString, JsUnknown, NapiRaw, NapiValue, Result, Status, ValueType,
};

#[derive(Clone, Copy)]
struct InternalValue {
    pub env: sys::napi_env,
    pub value: sys::napi_value,
    pub value_type: ValueType,
}

struct InternalJsFunction(pub InternalValue);

pub fn call_event(src: &JsFunction, this: Option<&JsObject>, args1: JsString, args2: JsFunction) -> Result<JsUnknown> {
    let f: InternalJsFunction = unsafe { std::mem::transmute_copy(src) };

    let raw_this = this
        .map(|v| unsafe { v.raw() })
        .or_else(|| unsafe { Env::from_raw(f.0.env) }.get_undefined().ok().map(|u| unsafe { u.raw() }))
        .ok_or_else(|| Error::new(Status::GenericFailure, "Get raw this failed".to_owned()))?;

    let raw_args = vec![unsafe { args1.raw() }, unsafe { args2.raw() }];
    let mut return_value = ptr::null_mut();
    check_pending_exception!(f.0.env, unsafe {
        sys::napi_call_function(f.0.env, raw_this, f.0.value, 2, raw_args.as_ptr(), &mut return_value)
    })?;
    unsafe { JsUnknown::from_raw(f.0.env, return_value) }
}
