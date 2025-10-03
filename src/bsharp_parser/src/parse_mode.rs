use std::cell::Cell;

thread_local! {
    static STRICT_MODE: Cell<bool> = const { Cell::new(false) };
}

pub fn set_strict(enabled: bool) {
    STRICT_MODE.with(|c| c.set(enabled));
}

pub fn is_strict() -> bool {
    STRICT_MODE.with(|c| c.get())
}
