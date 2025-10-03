use std::cell::RefCell;

#[derive(Default, Clone, Copy)]
pub struct BraceStatus {
    pub unmatched_open: Option<usize>,
}

thread_local! {
    static TRACKER: RefCell<Option<TrackerState>> = const { RefCell::new(None) };
    static LAST_STATUS: RefCell<Option<BraceStatus>> = const { RefCell::new(None) };
}

struct TrackerState {
    total_len: usize,
    stack: Vec<usize>,
}

impl TrackerState {
    fn new(input: &str) -> Self {
        Self {
            total_len: input.len(),
            stack: Vec::new(),
        }
    }

    fn record_open(&mut self, input: &str) {
        self.stack.push(self.offset_from(input));
    }

    fn record_close(&mut self) {
        let _ = self.stack.pop();
    }

    fn offset_from(&self, slice: &str) -> usize {
        self.total_len.saturating_sub(slice.len())
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

pub fn install(input: &str) -> BraceTrackerGuard {
    TRACKER.with(|cell| {
        let mut guard = cell.borrow_mut();
        debug_assert!(guard.is_none(), "brace tracker already installed");
        *guard = Some(TrackerState::new(input));
    });
    LAST_STATUS.with(|cell| {
        *cell.borrow_mut() = None;
    });
    BraceTrackerGuard { active: true }
}

pub fn on_char(input: &str, matched: char) {
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
