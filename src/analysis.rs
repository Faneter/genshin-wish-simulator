use std::sync::atomic::{AtomicI32, Ordering};

static LEN: AtomicI32 = AtomicI32::new(0);
static COUNTS: AtomicI32 = AtomicI32::new(0);
static UP: AtomicI32 = AtomicI32::new(0);

pub fn add(count: i32, up: bool) {
    LEN.fetch_add(1, Ordering::Relaxed);
    COUNTS.fetch_add(count, Ordering::Relaxed);
    if up {
        UP.fetch_add(1, Ordering::Relaxed);
    }
}

pub fn clear() {
    LEN.store(0, Ordering::Relaxed);
    COUNTS.store(0, Ordering::Relaxed);
    UP.store(0, Ordering::Relaxed);
}

pub fn print_results() {
    println!(
        "平均{}抽出金，{}抽UP，不歪率为{}。",
        COUNTS.load(Ordering::Relaxed) / LEN.load(Ordering::Relaxed),
        COUNTS.load(Ordering::Relaxed) / UP.load(Ordering::Relaxed),
        UP.load(Ordering::Relaxed) as f32 / LEN.load(Ordering::Relaxed) as f32
    )
}
