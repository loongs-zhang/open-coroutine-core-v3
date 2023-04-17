cfg_if::cfg_if! {
    if #[cfg(feature = "setjmp")] {
        mod setjmp;
    } else if #[cfg(feature = "boost")] {
        mod boost;
    } else if #[cfg(feature = "korosensei")] {
        use corosensei::ScopedCoroutine;
    } else {
        compile_error!("enable at least one coroutine");
    }
}
