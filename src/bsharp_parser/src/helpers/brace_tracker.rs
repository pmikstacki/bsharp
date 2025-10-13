use std::cell::RefCell;
use crate::syntax::span::Span;

#[derive(Default, Clone, Copy)]
pub struct BraceStatus {
    pub unmatched_open: Option<usize>,
}

thread_local! {
    static TRACKER: RefCell<Option<TrackerState>> = const { RefCell::new(None) };
    static LAST_STATUS: RefCell<Option<BraceStatus>> = const { RefCell::new(None) };
}

struct TrackerState {
    stack: Vec<usize>,
}

impl TrackerState {
    fn new() -> Self {
        Self { stack: Vec::new() }
    }

    fn record_open(&mut self, at: Span<'_>) {
        self.stack.push(at.location_offset());
    }

    fn record_close(&mut self) {
        let _ = self.stack.pop();
    }

    fn into_status(self) -> BraceStatus {
        BraceStatus {
            unmatched_open: self.stack.last().copied(),
        }
    }
}

fn flush_tracker() -> BraceStatus {
    TRACKER.with(|cell| {
        let mut guard = cell.borrow_mut();
        guard
            .take()
            .map(|state| state.into_status())
            .unwrap_or_default()
    })
}

pub fn store_status(status: BraceStatus) {
    LAST_STATUS.with(|cell| {
        *cell.borrow_mut() = Some(status);
    });
}

pub fn take_status() -> Option<BraceStatus> {
    LAST_STATUS.with(|cell| cell.borrow_mut().take())
}

pub struct BraceTrackerGuard {
    active: bool,
}

impl BraceTrackerGuard {
    pub fn finish(mut self) -> BraceStatus {
        let status = flush_tracker();
        store_status(status);
        self.active = false;
        status
    }
}

impl Drop for BraceTrackerGuard {
    fn drop(&mut self) {
        if self.active {
            let status = flush_tracker();
            store_status(status);
        }
    }
}

pub fn install(_input: Span<'_>) -> BraceTrackerGuard {
    TRACKER.with(|cell| {
        let mut guard = cell.borrow_mut();
        debug_assert!(guard.is_none(), "brace tracker already installed");
        *guard = Some(TrackerState::new());
    });
    LAST_STATUS.with(|cell| {
        *cell.borrow_mut() = None;
    });
    BraceTrackerGuard { active: true }
}

pub fn on_char(input: Span<'_>, matched: char) {
    TRACKER.with(|cell| {
        if let Some(state) = cell.borrow_mut().as_mut() {
            match matched {
                '{' => state.record_open(input),
                '}' => state.record_close(),
                _ => {}
            }
        }
    });
}
