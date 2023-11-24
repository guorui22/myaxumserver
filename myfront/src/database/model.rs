use serde::Deserialize;

/// 数据库批量查询参数
#[derive(Debug, Clone, Deserialize)]
pub struct DbBatchQueryArgs {
    pub str_sql_array: Vec<String>,
    pub str_node_env: String,
}