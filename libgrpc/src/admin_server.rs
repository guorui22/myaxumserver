use std::sync::Arc;

use sqlx::{MySqlPool, Row};

use libauth::password;
use libproto::{
    admin_service_server::AdminService, AdminExistsReply, AdminExistsRequest, CreateAdminReply,
    CreateAdminRequest, EditAdminReply, EditAdminRequest, GetAdminReply, GetAdminRequest,
    ListAdminReply, ListAdminRequest, ToggleAdminReply, ToggleAdminRequest,
};

pub struct Admin {
    pub pool: Arc<MySqlPool>,
}

impl Admin {
    pub fn new(pool: Arc<MySqlPool>) -> Self {
        Self {
            pool,
        }
    }
}

#[tonic::async_trait]
impl AdminService for Admin {
    async fn create_admin(
        &self,
        request: tonic::Request<CreateAdminRequest>,
    ) -> Result<tonic::Response<CreateAdminReply>, tonic::Status> {
        let request = request.into_inner();
        let AdminExistsReply { exists } = self
            .admin_exists(tonic::Request::new(AdminExistsRequest {
                condition: Some(libproto::admin_exists_request::Condition::Email(
                    request.email.clone(),
                )),
            }))
            .await?
            .into_inner();
        if exists {
            return Err(tonic::Status::already_exists("管理员已存在"));
        }
        let pwd = password::hash(&request.password).map_err(tonic::Status::internal)?;
        let row_id = sqlx::query("INSERT INTO admins (email,password) VALUES (?,?)")
            .bind(request.email)
            .bind(pwd)
            .execute(&*self.pool)
            .await
            .map_err(|err| tonic::Status::internal(err.to_string()))?
            .last_insert_id();
        Ok(tonic::Response::new(CreateAdminReply { id: row_id as i32 }))
    }
    async fn list_admin(
        &self,
        request: tonic::Request<ListAdminRequest>,
    ) -> Result<tonic::Response<ListAdminReply>, tonic::Status> {
        let ListAdminRequest { email, is_del } = request.into_inner();
        let rows = sqlx::query(
            r#"
            SELECT
                id,email,is_del
            FROM
                admins
            WHERE 1=1
                AND (? IS NULL OR email ILIKE CONCAT('%',?,'%'))
                AND (? IS NULL OR is_del=?)
        "#,
        )
            .bind(&email)
            .bind(&email)
            .bind(is_del)
            .bind(is_del)
            .fetch_all(&*self.pool)
            .await
            .map_err(|err| tonic::Status::internal(err.to_string()))?;
        let mut admins = Vec::with_capacity(rows.len());
        for row in rows {
            let a = libproto::Admin {
                id: row.get("id"),
                email: row.get("email"),
                is_del: row.get("is_del"),
                password: None,
            };
            admins.push(a);
        }
        Ok(tonic::Response::new(ListAdminReply { admins }))
    }
    async fn edit_admin(
        &self,
        request: tonic::Request<EditAdminRequest>,
    ) -> Result<tonic::Response<EditAdminReply>, tonic::Status> {
        let EditAdminRequest {
            id,
            email,
            password,
            new_password,
        } = request.into_inner();
        let new_password = match new_password {
            Some(n) => n,
            None => return Err(tonic::Status::invalid_argument("请设定新密码")),
        };
        let row = sqlx::query("SELECT password FROM admins WHERE id=? AND email=?")
            .bind(id)
            .bind(&email)
            .fetch_optional(&*self.pool)
            .await
            .map_err(|err| tonic::Status::internal(err.to_string()))?;
        let pwd_in_db: String = match row {
            Some(row) => row.get("password"),
            None => return Err(tonic::Status::not_found("不存在的用户")),
        };
        let is_verify = password::verify(&password, &pwd_in_db).map_err(tonic::Status::internal)?;
        if !is_verify {
            return Err(tonic::Status::invalid_argument("密码错误"));
        }
        let hashed_new_pwd = password::hash(&new_password).map_err(tonic::Status::internal)?;
        let rows_affected = sqlx::query("UPDATE admins SET password=? WHERE id=? AND email=?")
            .bind(hashed_new_pwd)
            .bind(id)
            .bind(&email)
            .execute(&*self.pool)
            .await
            .map_err(|err| tonic::Status::internal(err.to_string()))?
            .rows_affected();
        Ok(tonic::Response::new(EditAdminReply {
            id,
            ok: rows_affected > 0,
        }))
    }
    async fn toggle_admin(
        &self,
        request: tonic::Request<ToggleAdminRequest>,
    ) -> Result<tonic::Response<ToggleAdminReply>, tonic::Status> {
        let ToggleAdminRequest { id } = request.into_inner();
        let row_count = sqlx::query("UPDATE admins SET is_del=(NOT is_del) WHERE id=?")
            .bind(id)
            .execute(&*self.pool)
            .await
            .map_err(|err| tonic::Status::internal(err.to_string()))?
            .rows_affected();
        if row_count > 0 {
            if let Ok(response) = self.get_admin(tonic::Request::new(GetAdminRequest {
                condition: Some(libproto::get_admin_request::Condition::ById(
                    libproto::get_admin_request::ById {
                        id,
                        is_del: None,
                    },
                )),
            })).await {
                if let Some(admin) = response.into_inner().admin {
                    return Ok(tonic::Response::new(ToggleAdminReply {
                        id,
                        is_del: admin.is_del,
                    }));
                };
            }
        }
        Err(tonic::Status::not_found("不存在的用户"))
    }
    async fn admin_exists(
        &self,
        request: tonic::Request<AdminExistsRequest>,
    ) -> Result<tonic::Response<AdminExistsReply>, tonic::Status> {
        let AdminExistsRequest { condition } = request.into_inner();
        let condition = match condition {
            Some(condition) => condition,
            None => return Err(tonic::Status::invalid_argument("请指定条件")),
        };
        let row = match condition {
            libproto::admin_exists_request::Condition::Email(email) => {
                sqlx::query("SELECT COUNT(*) FROM admins WHERE email=?").bind(email)
            }
            libproto::admin_exists_request::Condition::Id(id) => {
                sqlx::query("SELECT COUNT(*) FROM admins WHERE id=?").bind(id)
            }
        }
            .fetch_one(&*self.pool)
            .await
            .map_err(|err| tonic::Status::internal(err.to_string()))?;
        let count: i64 = row.get(0);
        Ok(tonic::Response::new(AdminExistsReply { exists: count > 0 }))
    }
    async fn get_admin(
        &self,
        request: tonic::Request<GetAdminRequest>,
    ) -> Result<tonic::Response<GetAdminReply>, tonic::Status> {
        let GetAdminRequest { condition } = request.into_inner();
        let condition = match condition {
            Some(condition) => condition,
            None => return Err(tonic::Status::invalid_argument("请指定条件")),
        };
        let reply = match condition {
            libproto::get_admin_request::Condition::ByAuth(ba) => {
                let row = sqlx::query("SELECT id,email,is_del,password FROM admins WHERE email=?")
                    .bind(ba.email)
                    .fetch_optional(&*self.pool)
                    .await
                    .map_err(|err| tonic::Status::internal(err.to_string()))?;
                if let Some(row) = row {
                    let hashed_pwd: String = row.get("password");
                    let is_verify = password::verify(&ba.password, &hashed_pwd)
                        .map_err(tonic::Status::internal)?;
                    if !is_verify {
                        return Err(tonic::Status::invalid_argument("用户名/密码错误2"));
                    } else {
                        GetAdminReply {
                            admin: Some(libproto::Admin {
                                id: row.get("id"),
                                email: row.get("email"),
                                password: None,
                                is_del: row.get("is_del"),
                            }),
                        }
                    }
                } else {
                    return Err(tonic::Status::invalid_argument("用户名/密码错误"));
                }
            }
            libproto::get_admin_request::Condition::ById(bi) => {
                let row = match bi.is_del {
                    Some(is_del) => {
                        sqlx::query("SELECT id,email,is_del FROM admins WHERE id=? AND is_del=?")
                            .bind(bi.id)
                            .bind(is_del)
                    }
                    None => {
                        sqlx::query("SELECT id,email,is_del FROM admins WHERE id=?").bind(bi.id)
                    }
                }
                    .fetch_optional(&*self.pool)
                    .await
                    .map_err(|err| tonic::Status::internal(err.to_string()))?;
                if let Some(row) = row {
                    GetAdminReply {
                        admin: Some(libproto::Admin {
                            id: row.get("id"),
                            email: row.get("email"),
                            password: None,
                            is_del: row.get("is_del"),
                        }),
                    }
                } else {
                    return Err(tonic::Status::not_found("不存在的用户"));
                }
            }
        };
        Ok(tonic::Response::new(reply))
    }
}
