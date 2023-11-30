use std::sync::Arc;

use chrono::{Datelike, DateTime, Local, Timelike, TimeZone};
use sqlx::{MySqlPool, Row};

use libproto::{CreateTopicReply, CreateTopicRequest, EditTopicReply, EditTopicRequest, GetTopicReply, GetTopicRequest, ListTopicReply, ListTopicRequest, ToggleTopicReply, ToggleTopicRequest};
use libproto::topic_service_server::TopicService;

pub struct Topic {
    pool: Arc<MySqlPool>,
}

impl Topic {
    pub fn new(pool: Arc<MySqlPool>) -> Self {
        Self {
            pool,
        }
    }
}

#[tonic::async_trait]
impl TopicService for Topic {
    async fn create_topic(
        &self,
        request: tonic::Request<CreateTopicRequest>,
    ) -> Result<tonic::Response<CreateTopicReply>, tonic::Status> {
        let CreateTopicRequest {
            title,
            category_id,
            content,
            summary,
        } = request.into_inner();

        let summary = match summary {
            Some(summary) => summary,
            None => get_summary(&content),
        };
        let row_id = sqlx::query("INSERT INTO topics (title,category_id,content,summary) VALUES(?,?,?,?)")
            .bind(title)
            .bind(category_id)
            .bind(content)
            .bind(summary)
            .execute(&*self.pool)
            .await.map_err(|err| tonic::Status::internal(err.to_string()))?
            .last_insert_id();
        let reply = CreateTopicReply { id: row_id as i64 };
        Ok(tonic::Response::new(reply))
    }
    async fn edit_topic(
        &self,
        request: tonic::Request<EditTopicRequest>,
    ) -> Result<tonic::Response<EditTopicReply>, tonic::Status> {
        let r = request.into_inner();
        let summary = match r.summary {
            Some(s) => s,
            None => get_summary(&r.content),
        };
        let rows_affected = sqlx::query(
            "UPDATE topics SET title=?,content=?,summary=?,category_id=? WHERE id=?",
        )
            .bind(r.title)
            .bind(r.content)
            .bind(summary)
            .bind(r.category_id)
            .bind(r.id)
            .execute(&*self.pool)
            .await
            .map_err(|err| tonic::Status::internal(err.to_string()))?
            .rows_affected();
        Ok(tonic::Response::new(EditTopicReply {
            id: r.id,
            ok: rows_affected > 0,
        }))
    }
    async fn list_topic(
        &self,
        request: tonic::Request<ListTopicRequest>,
    ) -> Result<tonic::Response<ListTopicReply>, tonic::Status> {
        let ListTopicRequest {
            page,
            category_id,
            keyword,
            is_del,
            dateline_range,
        } = request.into_inner();
        let page = page.unwrap_or(0);
        let page_size = 30;
        let offset = page * page_size;
        let mut start = None;
        let mut end = None;
        if let Some(dr) = dateline_range {
            start = tm_cover(dr.start);
            end = tm_cover(dr.end);
        }

        let row = sqlx::query(
            r#"
            SELECT
                COUNT(*)
            FROM
            topics
            WHERE 1=1
                AND (? IS NULL OR category_id = ?)
                AND (? IS NULL OR title ILIKE CONCAT('%',?,'%'))
                AND (? IS NULL OR is_del = ?)
                AND (
                    (? IS NULL OR ? IS NULL)
                    OR
                    (dateline BETWEEN ? AND ?)
                )"#,
        )
            .bind(category_id)
            .bind(category_id)
            .bind(&keyword)
            .bind(&keyword)
            .bind(is_del)
            .bind(is_del)
            .bind(start)
            .bind(end)
            .bind(start)
            .bind(end)
            .fetch_one(&*self.pool)
            .await
            .map_err(|err| tonic::Status::internal(err.to_string()))?;

        dbg!(&row);

        let record_total: i64 = row.get(0);
        let page_totoal = f64::ceil(record_total as f64 / page_size as f64) as i64;

        let rows = sqlx::query(
            r#"
        SELECT
            id,title,content,summary,is_del,category_id,dateline,hit FROM topics
        WHERE 1=1
                AND (? IS NULL OR category_id = ?)
                AND (? IS NULL OR title ILIKE CONCAT('%',?,'%'))
                AND (? IS NULL OR is_del = ?)
                AND (
                    (? IS NULL OR ? IS NULL)
                    OR
                    (dateline BETWEEN ? AND ?)
                )
        ORDER BY id DESC
        LIMIT ? OFFSET ?
        "#,
        )
            .bind(category_id)
            .bind(category_id)
            .bind(&keyword)
            .bind(&keyword)
            .bind(is_del)
            .bind(is_del)
            .bind(start)
            .bind(end)
            .bind(start)
            .bind(end)
            .bind(page_size)
            .bind(offset)
            .fetch_all(&*self.pool)
            .await
            .map_err(|err| tonic::Status::internal(err.to_string()))?;

        let mut topics = Vec::with_capacity(rows.len());

        for row in rows {
            let dt: DateTime<Local> = row.get("dateline");
            let dateline = dt_conver(&dt);
            topics.push(libproto::Topic {
                id: row.get("id"),
                title: row.get("title"),
                category_id: row.get("category_id"),
                content: row.get("content"),
                summary: row.get("summary"),
                hit: row.get("hit"),
                is_del: row.get("is_del"),
                dateline,
            });
        }

        Ok(tonic::Response::new(ListTopicReply {
            page,
            page_size,
            topics,
            record_total,
            page_totoal,
        }))
    }
    async fn toggle_topic(
        &self,
        request: tonic::Request<ToggleTopicRequest>,
    ) -> Result<tonic::Response<ToggleTopicReply>, tonic::Status> {
        let ToggleTopicRequest { id } = request.into_inner();
        let row_count = sqlx::query("UPDATE topics SET is_del=(NOT is_del) WHERE id=?")
            .bind(id)
            .execute(&*self.pool)
            .await
            .map_err(|err| tonic::Status::internal(err.to_string()))?
            .rows_affected();
        if row_count > 0 {
            let request = tonic::Request::new(GetTopicRequest {
                id,
                is_del: None,
                inc_hit: None,
            });
            let reply = self.get_topic(request).await.unwrap();
            let reply = reply.into_inner().topic.unwrap();
            return Ok(tonic::Response::new(ToggleTopicReply {
                id,
                is_del: reply.is_del,
            }));
        }

        Err(tonic::Status::not_found("不存在的文章"))
    }
    async fn get_topic(
        &self,
        request: tonic::Request<GetTopicRequest>,
    ) -> Result<tonic::Response<GetTopicReply>, tonic::Status> {
        let GetTopicRequest {
            id,
            is_del,
            inc_hit,
        } = request.into_inner();

        let inc_hit = inc_hit.unwrap_or(false); // 增加点击量
        if inc_hit {
            sqlx::query("UPDATE topics SET hit=hit+1 WHERE id=?")
                .bind(id)
                .execute(&*self.pool)
                .await
                .map_err(|err| tonic::Status::internal(err.to_string()))?;
        }

        let query = match is_del {
            Some(is_del) => sqlx::query("SELECT id,title,content,summary,is_del,category_id,dateline,hit FROM topics WHERE id=? AND is_del=?")
                .bind(id).bind(is_del),
            None => sqlx::query("SELECT id,title,content,summary,is_del,category_id,dateline,hit FROM topics WHERE id=?")
                .bind(id),
        };
        let row = query
            .fetch_optional(&*self.pool)
            .await
            .map_err(|err| tonic::Status::internal(err.to_string()))?;
        if row.is_none() {
            return Err(tonic::Status::not_found("不存在的文章"));
        }
        let row = row.unwrap();
        let dt: DateTime<Local> = row.get("dateline");
        let dateline = dt_conver(&dt);

        Ok(tonic::Response::new(GetTopicReply {
            topic: Some(libproto::Topic {
                id: row.get("id"),
                title: row.get("title"),
                category_id: row.get("category_id"),
                content: row.get("content"),
                summary: row.get("summary"),
                hit: row.get("hit"),
                is_del: row.get("is_del"),
                dateline,
            }),
        }))
    }
}

fn get_summary(content: &str) -> String {
    if content.len() <= 255 {
        return String::from(content);
    }
    // 获取前255个字符
    content.chars().take(255).collect()
}

// 转换时间戳, 把DateTime<Local>, 转换为Option<prost_types::Timestamp>
fn dt_conver(dt: &DateTime<Local>) -> Option<prost_types::Timestamp> {
    if let Ok(dt) = prost_types::Timestamp::date_time(
        dt.year().into(),
        dt.month() as u8,
        dt.day() as u8,
        dt.hour() as u8,
        dt.minute() as u8,
        dt.second() as u8,
    ) {
        Some(dt)
    } else {
        None
    }
}

// 转换时间戳, 把Option<prost_types::Timestamp>, 转换为Option<DateTime<Local>>
fn tm_cover(tm: Option<prost_types::Timestamp>) -> Option<DateTime<Local>> {
    tm.and_then(|tm| Option::from({
        Local.timestamp_opt(tm.seconds, 0).unwrap()
    }))
}
