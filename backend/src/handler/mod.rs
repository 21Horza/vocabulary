use crate::{db, DBPool, Result};
use common::*;
use warp::{http::StatusCode, reject, reply::json, Reply};

pub async fn get_all_posts(db_pool: DBPool) -> Result<impl Reply> {
    let posts = db::post::get_all(&db_pool)
        .await
        .map_err(reject::custom)?;

    Ok(json::<Vec<_>>(
        &posts.into_iter().map(PostResponse::get).collect(),
    ))
}

pub async fn get_one_post(id: i32, db_pool: DBPool) -> Result<impl Reply> {
    let post = db::post::get_one(&db_pool, id)
            .await
            .map_err(reject::custom)?;

    Ok(json(&PostResponse::get(post)))
}

pub async fn create_post(post_dto: PostDto, db_pool: DBPool) -> Result<impl Reply> {
    Ok(json(&PostResponse::get(
        db::post::create(&db_pool, post_dto)
            .await
            .map_err(reject::custom)?
    )))
}

pub async fn delete_post(id: i32, db_pool: DBPool) -> Result<impl Reply> {
    db::post::delete(&db_pool, id)
        .await
        .map_err(reject::custom)?;
    Ok(StatusCode::OK)
}