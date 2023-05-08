use riscv::register::time;

use crate::{config::CLOCK_FREQ, sbi::set_timer};

#[inline]
/// 可能返回的是 tick 数
pub fn get_time() -> usize {
    time::read()
}

// 产生时钟中断的频率
const TIMER_FREQ: usize = 100;

/// 产生时钟中断的间隔，是OS控制的？
pub fn set_next_trigger() {
    // 1s有多少个tick / 1s产生多少个时钟中断=时钟中断的间隔，用tick衡量
    // 这个间隔大概是 TIMER_FREQ/CLOCK_FREQ=8
    set_timer(get_time() + CLOCK_FREQ / TIMER_FREQ)
}

const MICRO_PER_SEC: usize = 1_000_000;

// #[inline]
// pub fn get_time_us() -> usize {
//     ticks_to_us(get_time())
// }

pub fn ticks_to_us(ticks: usize) -> usize {
    ticks / (CLOCK_FREQ / MICRO_PER_SEC)
}

#[derive(Clone, Copy)]
pub struct StopWatch(usize);

impl StopWatch {
    #[inline]
    pub fn start(&mut self) {
        self.0 = get_time();
    }

    /// 跟 [crate::timer::get_time] 的单位一样
    #[inline]
    pub fn lap(&mut self) -> usize {
        let current = get_time();
        if self.untouched() {
            panic!("cannot stop an untouched stopwatch")
        }
        let ret = current - self.0;
        self.0 = current;
        ret
    }

    #[inline]
    pub fn untouched(&self) -> bool {
        self.0 == 0
    }

    #[inline]
    pub fn init() -> Self {
        StopWatch(0)
    }
}
