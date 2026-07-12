// Silences warnings from the compiler about Work.func and child_entry_point
// being unused when the target is not wasm.
#![cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]

use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use web_sys::{DedicatedWorkerGlobalScope, MessageEvent};
use web_sys::{ErrorEvent, Event, Worker};

struct Work {
    func: Box<dyn FnOnce() + Send>,
}

fn spawn_worker() -> Result<Worker, JsValue> {
    let worker = Worker::new("./worker.js")?;

    // When a worker is spun up send it the module and memory so it can start
    // instantiating the Wasm module.

    let array = js_sys::Array::new();
    array.push(&wasm_bindgen::module());
    array.push(&wasm_bindgen::memory());
    worker.post_message(&array)?;

    Ok(worker)
}

/// Spawns a single standalone worker and runs `f` on it.
pub fn spawn(f: impl FnOnce() + Send + 'static) -> Result<(), JsValue> {
    let worker = spawn_worker()?;

    let worker2 = worker.clone();
    let completion_slot = Rc::new(RefCell::new(None));
    let slot2 = completion_slot.clone();

    let completion = Closure::<dyn FnMut(_)>::new(move |event: Event| {
        if let Some(error) = event.dyn_ref::<ErrorEvent>() {
            console_log!("error in standalone worker: {}", error.message());
        } else if let Some(message) = event.dyn_ref::<MessageEvent>() {
            if kaja_web::try_handle_component_update_message(message) {
                return;
            }

            if message.data().is_undefined() {
                worker2.terminate();
                worker2.set_onmessage(None);
                worker2.set_onerror(None);
                *slot2.borrow_mut() = None;
                return;
            }

            console_log!("unhandled message from standalone worker");
            crate::logv(&message.data());
        } else {
            console_log!("unhandled event: {}", event.type_());
            crate::logv(&event);
        }

        worker2.terminate();
        worker2.set_onmessage(None);
        worker2.set_onerror(None);
        *slot2.borrow_mut() = None;
    });

    worker.set_onmessage(Some(completion.as_ref().unchecked_ref()));
    worker.set_onerror(Some(completion.as_ref().unchecked_ref()));
    *completion_slot.borrow_mut() = Some(completion);

    let work = Box::new(Work { func: Box::new(f) });
    let ptr = Box::into_raw(work);

    match worker.post_message(&JsValue::from(ptr as u32)) {
        Ok(()) => Ok(()),
        Err(e) => {
            unsafe {
                drop(Box::from_raw(ptr));
            }
            worker.terminate();
            worker.set_onmessage(None);
            worker.set_onerror(None);
            *completion_slot.borrow_mut() = None;
            Err(e)
        }
    }
}

/// Entry point invoked by `worker.js`
#[wasm_bindgen]
pub fn init_thread_runner(ptr: u32) -> Result<(), JsValue> {
    let ptr = unsafe { Box::from_raw(ptr as *mut Work) };
    let global = js_sys::global().unchecked_into::<DedicatedWorkerGlobalScope>();
    (ptr.func)();
    global.post_message(&JsValue::undefined())?;
    Ok(())
}
