use crate::db::DbPool;
use crate::dto::error::ErrorDto;
use crate::dto::group::{CreateGroupRequest, GroupDto, GroupPublicDto, UpdateGroupRequest};
use crate::middleware::auth::strong::StrongAuthGuard;
use crate::middleware::auth::AuthGuard;
use crate::middleware::validation::ValidatedJson;
use actix_web::web::Json;
use actix_web::{delete, get, post, put, web, HttpResponse};

pub fn config_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(create_group);
    cfg.service(update_group);
    cfg.service(get_group);
    cfg.service(get_group_public);
}

#[post("/groups/create")]
async fn create_group(
    auth: AuthGuard,
    db_pool: web::Data<DbPool>,
    request: ValidatedJson<CreateGroupRequest>,
) -> Result<Json<GroupDto>, ErrorDto> {
    todo!()
}

#[put("/groups/{group_id}/update")]
async fn update_group(
    auth: AuthGuard,
    db_pool: web::Data<DbPool>,
    group_id: web::Path<i64>,
    request: ValidatedJson<UpdateGroupRequest>,
) -> Result<Json<GroupDto>, ErrorDto> {
    todo!()
}

#[delete("/groups/{group_id}/delete")]
async fn delete_group(
    auth: StrongAuthGuard,
    db_pool: web::Data<DbPool>,
    group_id: web::Path<i64>,
) -> Result<HttpResponse, ErrorDto> {
    todo!()
}

#[get("/groups/{group_id}")]
async fn get_group(
    auth: AuthGuard,
    db_pool: web::Data<DbPool>,
    group_id: web::Path<i64>,
) -> Result<Json<GroupDto>, ErrorDto> {
    todo!()
}

#[get("/groups/{group_id}/public")]
async fn get_group_public(
    auth: AuthGuard,
    db_pool: web::Data<DbPool>,
    group_id: web::Path<i64>,
) -> Result<Json<GroupPublicDto>, ErrorDto> {
    todo!()
}
