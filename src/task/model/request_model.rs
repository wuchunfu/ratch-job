use crate::job::model::job::JobInfo;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct JobRunParam {
    pub job_id: u64,
    pub log_id: u64,
    pub executor_handler: Option<Arc<String>>,
    pub executor_params: Option<Arc<String>>,
    pub executor_block_strategy: Option<String>,
    pub executor_timeout: Option<i32>,
    pub log_date_time: Option<u64>,
    pub glue_type: Option<String>,
    pub glue_source: Option<String>,
    #[serde(rename(serialize = "glueUpdatetime", deserialize = "glueUpdatetime"))]
    pub glue_update_time: Option<u64>,
    pub broadcast_index: Option<u64>,
    pub broadcast_total: Option<u64>,
}

impl JobRunParam {
    pub fn from_job_info(log_id: u64, job_info: &Arc<JobInfo>) -> Self {
        Self {
            job_id: job_info.id,
            log_id,
            executor_handler: Some(job_info.handle_name.clone()),
            executor_params: Some(job_info.trigger_param.clone()),
            executor_block_strategy: Some(job_info.blocking_strategy.to_str().to_string()),
            executor_timeout: Some(job_info.timeout_second as i32),
            log_date_time: None,
            glue_type: Some(job_info.run_mode.to_str().to_string()),
            glue_source: None,
            glue_update_time: None,
            broadcast_index: Some(0),
            broadcast_total: Some(0),
        }
    }
}
