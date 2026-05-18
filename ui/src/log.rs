use crate::api::TryNodeAction;

pub(crate) fn _log(msg: impl AsRef<str>) {
    let msg = msg.as_ref();
    #[cfg(target_family = "wasm")]
    {
        web_sys::console::info_1(&serde_wasm_bindgen::to_value(&msg).unwrap());
    }
    let _ = msg;
}

/// Info-level log that goes to the browser console (wasm) and to
/// `tracing` on native builds. Use for one-shot startup banners or
/// build-time diagnostics that should always appear.
pub(crate) fn info(msg: impl AsRef<str>) {
    let msg = msg.as_ref();
    tracing::info!(%msg);
    #[cfg(target_family = "wasm")]
    {
        web_sys::console::info_1(&serde_wasm_bindgen::to_value(&msg).unwrap());
    }
}

macro_rules! debug {
    ($($msg:tt)*) => {{
        #[cfg(debug_assertions)]
        {
            let msg = format!($($msg)*);
            crate::log::__debug_internal(msg)
        }
    }};
}

pub(crate) use debug;

pub(crate) fn __debug_internal(msg: impl AsRef<str>) {
    let msg = msg.as_ref();
    #[cfg(target_family = "wasm")]
    {
        web_sys::console::debug_1(&serde_wasm_bindgen::to_value(&msg).unwrap());
    }
    let _ = msg;
}

/// Log an async local-state write failure AND surface it to the user via an
/// error toast. The optimistic in-memory write made the UI look correct;
/// without this surfacing the persistent write failure is silent and the row
/// silently un-mutates on the next reload (#231, #232).
///
/// `op` is the user-facing verb ("save draft", "delete draft", "mark read",
/// "archive", "save sent", "update delivery state"). The toast reads
/// "Couldn't <op> — changes may not persist after reload."
pub(crate) fn local_state_failure(op: &str, err: impl std::fmt::Display) {
    let detail = format!("{op} failed: {err}");
    error(detail, None);
    let user_msg = format!("Couldn't {op} — changes may not persist after reload.");
    crate::toast::push_toast(user_msg, crate::toast::ToastLevel::Error);
}

pub(crate) fn error(msg: impl AsRef<str>, action: Option<TryNodeAction>) {
    let error = msg.as_ref();
    if let Some(action) = action {
        tracing::error!(%error, %action);
        #[cfg(target_family = "wasm")]
        {
            let error = format!("error while `{action}`: {error}");
            web_sys::console::error_1(&serde_wasm_bindgen::to_value(&error).unwrap());
        }
    } else {
        tracing::error!(%error);
        #[cfg(target_family = "wasm")]
        {
            web_sys::console::error_1(&serde_wasm_bindgen::to_value(&error).unwrap());
        }
    }
}
