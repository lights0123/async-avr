use core::cell::UnsafeCell;
use core::future::Future;
use core::ptr;
use core::sync::atomic::{AtomicBool, Ordering};
use core::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};
use pin_utils::pin_mut;

#[derive(Debug)]
#[repr(transparent)]
struct Volatile<T: Copy>(UnsafeCell<T>);

impl<T: Copy> Volatile<T> {
    pub fn new(value: T) -> Volatile<T> {
        Volatile(UnsafeCell::new(value))
    }

    pub fn read(&self) -> T {
        unsafe { ptr::read_volatile(self.0.get()) }
    }

    pub fn write(&self, value: T) {
        unsafe { ptr::write_volatile(self.0.get(), value) };
    }
}

// NOTE `*const ()` is &Volatile<bool>
static VTABLE: RawWakerVTable = {
    unsafe fn clone(p: *const ()) -> RawWaker {
        RawWaker::new(p, &VTABLE)
    }
    unsafe fn wake(p: *const ()) {
        wake_by_ref(p)
    }
    unsafe fn wake_by_ref(p: *const ()) {
        (*(p as *const Volatile<bool>)).write(true)
    }
    unsafe fn drop(_: *const ()) {
        // no-op
    }

    RawWakerVTable::new(clone, wake, wake_by_ref, drop)
};

/// Spawns a task and blocks until the future resolves, returning its result.
pub fn block_on<T>(task: impl Future<Output = T>) -> T {
    let ready = Volatile::new(true);
    let waker = unsafe { Waker::from_raw(RawWaker::new(&ready as *const _ as *const _, &VTABLE)) };
    let mut context = Context::from_waker(&waker);
    pin_mut!(task);
    let mut task = task;
    loop {
        while ready.read() {
            match task.as_mut().poll(&mut context) {
                Poll::Ready(val) => {
                    return val;
                }
                Poll::Pending => {
                    // ready.write(false);
                }
            }
        }
    }
}
