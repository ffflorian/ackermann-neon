#[macro_use]
extern crate neon;
extern crate ackermann;

use neon::vm::{Call, JsResult};
use neon::js::{JsNumber};

fn ack(mut call: Call) -> JsResult<JsNumber> {
	let m_neon = call.arguments.require(call.scope, 0)?.check::<JsNumber>()?;
  let n_neon = call.arguments.require(call.scope, 1)?.check::<JsNumber>()?;

	let m = m_neon.value() as u64;
	let n = n_neon.value() as u64;

	let result = JsNumber::new(call.scope, ackermann::ack(m, n) as f64);
	Ok(result)
}

register_module!(m, {
	m.export("ack", ack)?;
	Ok(())
});
