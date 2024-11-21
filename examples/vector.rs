use rtsan_standalone_rs::*;

fn main() {
    unsafe { __rtsan_ensure_initialized() };

    let mut my_vec = Vec::new();

    unsafe { __rtsan_realtime_enter() };

    my_vec.push(1.0);

    unsafe { __rtsan_realtime_exit() };
}
