use crate::{
    config::MAX_APP_NUM,
    debug, info,
    loader::{get_num_app, trap_init},
    sbi::shutdown,
    task::context::TaskContext,
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
}

pub struct TaskManager {
    pub num_app: usize,
    inner: Mutex<TaskManagerInner>,
}

impl TaskManager {
    pub fn show_debugging_info(&self) {
        let inner = self.inner.lock();
        // for index in 0..self.num_app {
        //     let t = &inner.tasks[index];
        //     debug!("No.{} of TCB starts at {:#x} - {:#x} - {:#x}",
        //         index,
        //         t as *const TaskControlBlock as usize,
        //         &t.task_status as *const TaskStatus as usize,
        //         &t.task_cx as *const TaskContext as usize,
        //     );
        // }
        debug!(
            "Task control block starts at {:#x}",
            &inner.tasks[0] as *const TaskControlBlock as usize
        );
    }
}

pub struct TaskManagerInner {
    tasks: [TaskControlBlock; MAX_APP_NUM],
    current_task: usize,
}

lazy_static! {
    #[no_mangle]
    pub static ref TASK_MANAGER: TaskManager = {
        let num_app = get_num_app();
        let mut tasks = [TaskControlBlock {
            task_cx: TaskContext::zero_init(),
            task_status: TaskStatus::UnInit,
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
}

impl TaskManager {
    pub fn mark_current_exited(&self) {
        let mut inner = self.inner.lock();
        let current = inner.current_task;
        inner.tasks[current].task_status = TaskStatus::Exited;
    }

    pub fn mark_current_suspended(&self) {
        let mut inner = self.inner.lock();
        let current = inner.current_task;
        inner.tasks[current].task_status = TaskStatus::Ready;
    }

    pub fn run_next_app(&self) {
        if let Some(next) = self.find_next_app() {
            let mut inner = self.inner.lock();

            let mut current = inner.tasks[inner.current_task].task_cx;
            inner.current_task = next;
            inner.tasks[next].task_status = TaskStatus::Running;
            let mut next = inner.tasks[next].task_cx;

            drop(inner);
            unsafe { __switch(&mut current, &mut next) }
        } else {
            info!("all app finished");
            shutdown()
        }
    }

    pub fn find_next_app(&self) -> Option<usize> {
        let inner = self.inner.lock();
        let current = inner.current_task;

        (current + 1..current + self.num_app + 1)
            .map(|i| i % self.num_app)
            .find(|&i| inner.tasks[i].task_status == TaskStatus::Ready)
    }

    pub fn run_first_app(&self) -> ! {
        let mut inner = self.inner.lock();
        inner.current_task = 0;
        inner.tasks[0].task_status = TaskStatus::Running;

        let mut first = inner.tasks[0].task_cx;
        let mut dummy = TaskContext::zero_init();

        drop(inner);

        unsafe { __switch(&mut dummy, &mut first) };

        panic!("unreachable")
    }
}

pub fn suspend_and_run_next() {
    TASK_MANAGER.mark_current_suspended();
    TASK_MANAGER.run_next_app();
}

pub fn exit_and_run_next() {
    TASK_MANAGER.mark_current_exited();
    TASK_MANAGER.run_next_app();
}
