use std::sync::Arc;
use sqlx::{MySqlPool, Row};

use libproto::category_exists_request::Condition;
use libproto::category_service_server::CategoryService;
use libproto::{
    CategoryExistsReply, CategoryExistsRequest, CreateCategoryReply, CreateCategoryRequest,
    EditCategoryReply, EditCategoryRequest, GetCategoryReply, GetCategoryRequest,
    ListCategoryReply, ListCategoryRequest, ToggleCategoryReply, ToggleCategoryRequest,
};

pub struct Category {
    pool: Arc<MySqlPool>,
}

impl Category {
    pub fn new(pool: Arc<MySqlPool>) -> Self {
        Self { pool }
    }
}

#[tonic::async_trait]
impl CategoryService for Category {
    async fn create_category(
        &self,
        request: tonic::Request<CreateCategoryRequest>,
    ) -> Result<tonic::Response<CreateCategoryReply>, tonic::Status> {
        let CreateCategoryRequest { name } = request.into_inner();
        let exists_request = tonic::Request::new(CategoryExistsRequest {
            condition: Some(Condition::Name(name.clone())),
        });
        let exists_reply = self.category_exists(exists_request).await?.into_inner();
        if exists_reply.exists {
            return Err(tonic::Status::already_exists("分类已存在"));
        }
        let res = sqlx::query("INSERT INTO categories (name) VALUES (?)")
            .bind(name)
            .execute(&*self.pool)
            .await
            .map_err(|err| tonic::Status::internal(err.to_string()))?
            .last_insert_id();
        let reply = CreateCategoryReply { id: res as i32 };
        Ok(tonic::Response::new(reply))
    }
    async fn edit_category(
        &self,
        request: tonic::Request<EditCategoryRequest>,
    ) -> Result<tonic::Response<EditCategoryReply>, tonic::Status> {
        let EditCategoryRequest { id, name } = request.into_inner();
        let row = sqlx::query("SELECT COUNT(*) FROM categories WHERE name=? AND id<>?")
            .bind(&name)
            .bind(id)
            .fetch_one(&*self.pool)
            .await
            .map_err(|err| tonic::Status::internal(err.to_string()))?;
        let count: i64 = row.get(0);
        if count > 0i64 {
            return Err(tonic::Status::already_exists("分类已存在"));
        }
        let rows_affected = sqlx::query("UPDATE categories SET name=? WHERE id=?")
            .bind(&name)
            .bind(id)
            .execute(&*self.pool)
            .await
            .map_err(|err| tonic::Status::internal(err.to_string()))?
            .rows_affected();
        let reply = EditCategoryReply {
            id,
            ok: rows_affected > 0,
        };
        Ok(tonic::Response::new(reply))
    }
    async fn list_category(
        &self,
        request: tonic::Request<ListCategoryRequest>,
    ) -> Result<tonic::Response<ListCategoryReply>, tonic::Status> {
        let ListCategoryRequest { name, is_del } = request.into_inner();
        let query = match name {
            Some(name) => {
                let name = format!("%{}%", name);
                match is_del {
                    Some(is_del) => {
                        sqlx::query("SELECT id,name,is_del FROM categories WHERE name ILIKE ? AND is_del=? ORDER BY id")
                            .bind(name.clone())
                            .bind(is_del)
                    }
                    None => sqlx::query("SELECT id,name,is_del FROM categories WHERE name ILIKE ?  ORDER BY id")
                        .bind(name),
                }
            }
            None => match is_del {
                Some(is_del) => {
                    sqlx::query("SELECT id,name,is_del FROM categories WHERE is_del=? ORDER BY id")
                        .bind(is_del)
                }
                None => sqlx::query("SELECT id,name,is_del FROM categories ORDER BY id"),
            },
        };
        let rows = query
            .fetch_all(&*self.pool)
            .await
            .map_err(|err| tonic::Status::internal(err.to_string()))?;
        if rows.is_empty() {
            return Err(tonic::Status::not_found("没有符合条件的分类"));
        }
        let mut categories = Vec::with_capacity(rows.len());
        for row in rows {
            categories.push(libproto::Category {
                id: row.get("id"),
                name: row.get("name"),
                is_del: row.get("is_del"),
            });
        }
        let reply = ListCategoryReply { categories };
        Ok(tonic::Response::new(reply))
    }
    async fn toggle_category(
        &self,
        request: tonic::Request<ToggleCategoryRequest>,
    ) -> Result<tonic::Response<ToggleCategoryReply>, tonic::Status> {
        let ToggleCategoryRequest { id } = request.into_inner();
        let row_count = sqlx::query("UPDATE categories SET is_del=(NOT is_del) WHERE id=?")
            .bind(id)
            .execute(&*self.pool)
            .await
            .map_err(|err| tonic::Status::internal(err.to_string()))?
            .rows_affected();
        if row_count > 0 {
            let request = tonic::Request::new(GetCategoryRequest { id, is_del: None });
            let reply = self.get_category(request).await.unwrap();
            let reply = reply.into_inner().category.unwrap();
            return Ok(tonic::Response::new(ToggleCategoryReply {
                id,
                is_del: reply.is_del,
            }));
        }
        Err(tonic::Status::not_found("不存在的分类"))
    }
    async fn category_exists(
        &self,
        request: tonic::Request<CategoryExistsRequest>,
    ) -> Result<tonic::Response<CategoryExistsReply>, tonic::Status> {
        let request = request.into_inner();
        let condition = request
            .condition
            .ok_or(tonic::Status::invalid_argument("参数错误"))?;
        let query = match condition {
            Condition::Name(name) => {
                sqlx::query("SELECT COUNT(*) FROM categories WHERE name=?").bind(name)
            }
            Condition::Id(id) => sqlx::query("SELECT COUNT(*) FROM categories WHERE id=?").bind(id),
        };
        let row = query
            .fetch_one(&*self.pool)
            .await
            .map_err(|err| tonic::Status::internal(err.to_string()))?;
        let count: i64 = row.get(0);
        let reply = CategoryExistsReply { exists: count > 0 };
        Ok(tonic::Response::new(reply))
    }
    async fn get_category(
        &self,
        request: tonic::Request<GetCategoryRequest>,
    ) -> Result<tonic::Response<GetCategoryReply>, tonic::Status> {
        let GetCategoryRequest { id, is_del } = request.into_inner();
        let query = match is_del {
            Some(is_del) => {
                sqlx::query("SELECT id,name,is_del FROM categories WHERE id=? AND is_del=?")
                    .bind(id)
                    .bind(is_del)
            }
            None => sqlx::query("SELECT id,name,is_del FROM categories WHERE id=?").bind(id),
        };
        let row = query
            .fetch_optional(&*self.pool)
            .await
            .map_err(|err| tonic::Status::internal(err.to_string()))?;
        let reply = match row {
            Some(row) => GetCategoryReply {
                category: Some(libproto::Category {
                    id: row.get("id"),
                    name: row.get("name"),
                    is_del: row.get("is_del"),
                }),
            },
            None => GetCategoryReply { category: None },
        };
        Ok(tonic::Response::new(reply))
    }
}
