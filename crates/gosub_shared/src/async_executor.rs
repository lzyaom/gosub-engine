use std::future::Future;

#[cfg(not(target_arch = "wasm32"))]
pub trait WasmNotSend: Send {}

#[cfg(not(target_arch = "wasm32"))]
impl<T: Send> WasmNotSend for T {}

#[cfg(target_arch = "wasm32")]
pub trait WasmNotSend {}

#[cfg(target_arch = "wasm32")]
impl<T> WasmNotSend for T {}

#[cfg(not(target_arch = "wasm32"))]
pub trait WasmNotSync: Sync {}

#[cfg(not(target_arch = "wasm32"))]
impl<T: Sync> WasmNotSync for T {}

#[cfg(target_arch = "wasm32")]
pub trait WasmNotSync {}

#[cfg(target_arch = "wasm32")]
impl<T> WasmNotSync for T {}

pub trait WasmNotSendSync: WasmNotSend + WasmNotSync {}

impl<T: WasmNotSync + WasmNotSend> WasmNotSendSync for T {}

pub fn spawn<F: Future<Output = ()> + WasmNotSend + 'static>(f: F) {
    #[cfg(target_arch = "wasm32")]
    {
        wasm_bindgen_futures::spawn_local(f);
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        //TODO: this should be done with a thread pool
        std::thread::spawn(|| {
            futures::executor::block_on(f);
        });
    }
}

pub fn spawn_from<F: Future<Output = ()> + 'static>(f: impl FnOnce() -> F + 'static + WasmNotSend) {
    #[cfg(target_arch = "wasm32")]
    {
        let fut = f(ctx);
        wasm_bindgen_futures::spawn_local(f);
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        std::thread::spawn(|| {
            let fut = f();
            futures::executor::block_on(fut);
        });
    }
}
