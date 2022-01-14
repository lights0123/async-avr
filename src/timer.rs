use core::cell::Ref;
use core::task::{Context, Poll};

pub trait TimeReader {
    fn ticks(&self) -> u16;
}

pub struct Timer<'a, T> where T: TimeReader {
    delay_in_ticks: u16,
    start_tick: u16,
    hw_timer: Ref<'a, T>,
}

impl<'a, T: TimeReader> Timer<'a, T> {
    pub fn sleep_in_millis(delay_in_millis: u16, hw_timer: Ref<'a, T>) -> Timer<'a, T> {
        Timer {
            // this assumes a 16MHz timer with a 1024 prescalar
            delay_in_ticks: delay_in_millis * 10 / 64u16 * 100,
            start_tick: hw_timer.ticks(),
            hw_timer: hw_timer
        }
    }
}

impl<'a, T: TimeReader> crate::Future for Timer<'a, T> {
    type Output = bool;

    fn poll(self: crate::Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        let tick = self.hw_timer.ticks();

        if tick - self.start_tick >= self.delay_in_ticks {
            return Poll::Ready(true);
        } else {
            return Poll::Pending;
        }
    }
}
