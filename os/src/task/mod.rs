use crate::{
    config::MAX_APP_NUM,
    debug, info,
    loader::{get_num_app, trap_init},
    sbi::shutdown,
    task::context::TaskContext,
    timer::{ticks_to_us, StopWatch},
};

use self::switch::__switch;

use lazy_static::lazy_static;
use spin::Mutex;

mod context;
mod switch;

#[derive(Clone, Copy, PartialEq)]
pub enum TaskStatus {
    UnInit,
    Ready,
    Running,
    Exited,
}

#[derive(Clone, Copy)]
pub struct TaskControlBlock {
    pub task_status: TaskStatus,
    pub task_cx: TaskContext,
    /// 用户态计时
    pub stopwatch_user: StopWatch,
    /// 内核计时
    pub stopwatch_kernel: StopWatch,
    /// 总用时
    pub stopwatch_total: StopWatch,
}

pub struct TaskManager {
    pub num_app: usize,
    inner: Mutex<TaskManagerInner>,
    // inner2: UPSafeCell<TaskManagerInner>,
}

impl TaskManager {
    pub fn show_debugging_info(&self) {
        let inner = self.inner.lock();
        inner.show_debugging_info(self.num_app)
    }
}

pub struct TaskManagerInner {
    tasks: [TaskControlBlock; MAX_APP_NUM],
    current_task: usize,
}

impl TaskManagerInner {
    #[inline]
    pub fn current_mut(&mut self) -> &mut TaskControlBlock {
        &mut self.tasks[self.current_task]
    }

    #[inline]
    pub fn set_current(&mut self, task: usize) -> &mut TaskControlBlock {
        self.current_task = task;
        self.current_mut()
    }

    pub fn show_debugging_info(&self, num_app: usize) {
        debug!(
            "Task control block starts at {:#x}",
            &self.tasks[0] as *const TaskControlBlock as usize
        );
        for index in 0..num_app {
            let t = &self.tasks[index];
            debug!("No.{} of TCB: {:?}",
                index,
                t.task_cx,
            );
        }
    }
}

lazy_static! {
    pub static ref TASK_MANAGER: TaskManager = {
        let num_app = get_num_app();
        let mut tasks = [TaskControlBlock {
            task_cx: TaskContext::zero_init(),
            task_status: TaskStatus::UnInit,
            stopwatch_user: StopWatch::init(),
            stopwatch_kernel: StopWatch::init(),
            stopwatch_total: StopWatch::init(),
        }; MAX_APP_NUM];

        debug!("TCBs on stack: {:#x}", &tasks[0] as *const TaskControlBlock as usize);

        // task context 的初始值是一个 trap context 的地址(sp)和 __restore 的地址(ra)
        // 所以第一次启动 __switch 的时候是跳到 __restore
        // __restore 的参数，即是 trap context 的地址
        for i in 0..num_app {
            tasks[i].task_cx = TaskContext::init(trap_init(i));
            tasks[i].task_status = TaskStatus::Ready;
        }

        TaskManager {
            num_app,
            inner:
                Mutex::new(TaskManagerInner {
                    tasks,
                    current_task: 0,
                })
            ,
        }
    };

    pub static ref UPTIME: Mutex<StopWatch> = Mutex::new(StopWatch::init()) ;
}

impl TaskManager {
    /// 会尝试lock。
    ///
    /// 暂停user计时并且启动内核计时
    #[inline]
    pub fn enter_trap(&self) {
        let mut inner = self.inner.lock();
        let current = inner.current_mut();
        current.stopwatch_user.stop();
        current.stopwatch_kernel.start();
    }
    /// 会尝试lock
    ///
    /// 暂停内核计时并且启动user计时
    #[inline]
    pub fn leave_trap(&self) {
        let mut inner = self.inner.lock();
        let current = inner.current_mut();
        current.stopwatch_kernel.stop();
        current.stopwatch_user.start();
    }
    /// 会尝试lock
    /// 暂停app计时
    #[inline]
    pub fn mark_current_exited(&self) {
        let mut inner = self.inner.lock();
        let current = inner.current_mut();
        current.task_status = TaskStatus::Exited;
        current.stopwatch_kernel.stop();
        current.stopwatch_total.stop();
    }
    /// 会尝试lock
    /// 暂停app计时
    #[inline]
    pub fn mark_current_suspended(&self) {
        let mut inner = self.inner.lock();
        let current = inner.current_mut();
        current.task_status = TaskStatus::Ready;
        current.stopwatch_kernel.stop();
    }
    /// 会尝试lock
    pub fn run_next_app(&self) {
        if let Some(next) = self.find_next_app() {
            let mut inner = self.inner.lock();
            let current = &mut inner.current_mut().task_cx as *mut TaskContext;
            let next = inner.set_current(next);
            next.task_status = TaskStatus::Running;
            if next.stopwatch_total.untouched() {
                next.stopwatch_total.start();
            }
            // 启动下一个app计时。在此之前，该app的计时器因为进入上一次trap肯定已经被停掉了
            next.stopwatch_user.start();
            let next = &next.task_cx as *const TaskContext;

            drop(inner);
            unsafe { __switch(current, next) }
            // debug!("returned from switched context");
        } else {
            let total_kernel = {
                let mut uptime = UPTIME.lock();
                uptime.stop();
                uptime.acc()
            };
            let total_user = self.inner.lock().tasks
                .iter()
                .take(self.num_app)
                .enumerate()
                .fold(0usize, |acc, (i, t)| {
                    info!(
                        "app_{} time - user: {}us\tsys: {}us\ttotal: {}us",
                        i,
                        ticks_to_us(t.stopwatch_user.acc()),
                        ticks_to_us(t.stopwatch_kernel.acc()),
                        ticks_to_us(t.stopwatch_total.acc()),
                    );
                    acc + t.stopwatch_user.acc() + t.stopwatch_kernel.acc()
                });

            info!(
                "finished - user: {}us\tsys: {}us",
                ticks_to_us(total_user),
                ticks_to_us(total_kernel),
            );
            shutdown()
        }
    }

    /// 会尝试lock
    pub fn find_next_app(&self) -> Option<usize> {
        let inner = self.inner.lock();
        let current = inner.current_task;

        (current + 1..current + self.num_app + 1)
            .map(|i| i % self.num_app)
            .find(|&i| inner.tasks[i].task_status == TaskStatus::Ready)
    }

    pub fn run_first_app(&self) -> ! {
        let mut inner = self.inner.lock();
        let mut current = inner.set_current(0);

        current.task_status = TaskStatus::Running;

        let context = &mut current.task_cx as *const TaskContext;

        UPTIME.lock().start();
        current.stopwatch_total.start();
        current.stopwatch_user.start();

        drop(inner);
        unsafe { __switch(&mut TaskContext::zero_init(), context) };

        panic!("unreachable")
    }
}

#[inline]
pub fn suspend_and_run_next() {
    TASK_MANAGER.mark_current_suspended();
    TASK_MANAGER.run_next_app();
}

#[inline]
pub fn exit_and_run_next() {
    TASK_MANAGER.mark_current_exited();
    TASK_MANAGER.run_next_app();
}
