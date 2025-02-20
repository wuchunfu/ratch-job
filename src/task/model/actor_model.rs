use crate::app::model::AppKey;
use crate::job::model::job::{JobInfo, JobTaskLogQueryParam};
use crate::task::model::task::JobTaskInfo;
use actix::Message;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct TaskCallBackParam {
    pub task_id: u64,
    pub task_date_time: i64,
    pub success: bool,
    pub handle_msg: Option<String>,
}

#[derive(Debug, Message)]
#[rtype(result = "anyhow::Result<TaskManagerResult>")]
pub enum TaskManagerReq {
    AddAppInstance(AppKey, Arc<String>),
    RemoveAppInstance(AppKey, Arc<String>),
    TriggerTask(u32, Arc<JobInfo>),
    TaskCallBacks(Vec<TaskCallBackParam>),
}

pub enum TaskManagerResult {
    None,
}

#[derive(Debug, Message)]
#[rtype(result = "anyhow::Result<TaskHistoryManagerResult>")]
pub enum TaskHistoryManagerReq {
    UpdateTask(Arc<JobTaskInfo>),
    QueryJobTaskLog(JobTaskLogQueryParam),
}

pub enum TaskHistoryManagerResult {
    JobTaskLogPageInfo(usize, Vec<Arc<JobTaskInfo>>),
    None,
}
