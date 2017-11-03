#[macro_use]
extern crate neon;

use neon::vm::{Call, JsResult, Lock};
use neon::js::binary::JsArrayBuffer;

fn hello(call: Call) -> JsResult<JsArrayBuffer> {
    let scope = call.scope;
    let mut buffer = JsArrayBuffer::new(scope, 8)?;
    buffer.grab(|mut slice| {
        slice[0] = 17;
        slice[1] = 42;
        slice[2] = 111;
    });
    Ok(buffer)
}

register_module!(m, {
    m.export("hello", hello)
});
