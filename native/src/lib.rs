#[macro_use]
extern crate neon;

extern crate cslice;

use neon::vm::{Call, JsResult, Lock};
use neon::js::binary::JsArrayBuffer;
use cslice::CMutSlice;
use std::mem;

fn hello(call: Call) -> JsResult<JsArrayBuffer> {
    let scope = call.scope;
    let mut buffer = JsArrayBuffer::new(scope, 8)?;
    buffer.grab(|mut slice| {
        let len = slice.len();
        let raw = slice.as_mut_ptr();
        let mut slice: CMutSlice<f32> = unsafe { CMutSlice::new(mem::transmute(raw), len / 4) };
        slice[0] = 1.8;
        slice[1] = 13.4;
    });
    Ok(buffer)
}

register_module!(m, {
    m.export("hello", hello)
});
