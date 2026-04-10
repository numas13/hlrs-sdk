use xash3d_shared::{export::UnsyncGlobal, ffi};

use crate::engine::ServerEngine;

/// Initialize the global [ServerEngine] and [crate::globals::ServerGlobals] instances.
///
/// # Safety
///
/// Must be called only once.
pub unsafe fn init_engine(
    funcs: &ffi::server::enginefuncs_s,
    globals: *mut ffi::server::globalvars_t,
) {
    let engine = ServerEngine::new(funcs, globals);
    unsafe {
        (*ServerEngine::global_as_mut_ptr()).write(engine);
    }
}
