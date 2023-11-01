//! Types related to task management

use super::TaskContext;
use crate::config::MAX_SYSCALL_NUM;

/// The task control block (TCB) of a task.
#[derive(Copy, Clone)]
pub struct TaskControlBlock {
    /// The task status in it's lifecycle
    pub task_status: TaskStatus,
    /// The task context
    pub task_cx: TaskContext,
/// rummer status
    pub runner_status: TaskRunnerStatus
}
/// runner status
#[derive(Copy, Clone)]
pub struct TaskRunnerStatus {
    /// times about syscall
    pub syscall_times: [u32;MAX_SYSCALL_NUM],
    /// start time
    pub start_time: usize
}

/// default
impl Default for TaskRunnerStatus {
    fn default() -> Self {
        Self {
            start_time: 0,
            syscall_times: [0; MAX_SYSCALL_NUM]
        }
    }
}

/// The status of a task
#[derive(Copy, Clone, PartialEq)]
pub enum TaskStatus {
    /// uninitialized
    UnInit,
    /// ready to run
    Ready,
    /// running
    Running,
    /// exited
    Exited,
}

/// Task information
#[allow(dead_code)]
pub struct TaskInfo {
    /// Task status in it's life cycle
    pub status: TaskStatus,
    /// The numbers of syscall called by task
    pub syscall_times: [u32; MAX_SYSCALL_NUM],
    /// Total running time of task
    pub time: usize,
}