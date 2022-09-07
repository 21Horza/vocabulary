use mobc::{Connection, Pool};
use mobc_postgres::{tokio_postgres, PgConnectionManager};
use std::convert::Infallible;
use tokio_postgres::NoTls;
use warp::{
    http::{header, Method},
    Filter, Rejection,
};

mod db;
mod error;
mod handler;

type Result<T> = std::result::Result<T, Rejection>;
type DBCon = Connection<PgConnectionManager<NoTls>>;
type DBPool = Pool<PgConnectionManager<NoTls>>;

#[tokio::main]
async fn main() {
    let db_pool = db::create_pool().expect("DB can be created");

    db::init_db(&db_pool)
        .await
        .expect("DB can be initialized");

    let posts = warp::path!("posts");
    let post_param = warp::path!("posts" / i32);
    
    let all_posts = posts
        .and(warp::get())
        .and(with_db(db_pool.clone()))
        .and_then(handler::get_all_posts)
    .or(posts
        .and(warp::post())
        .and(warp::body::json())
        .and(with_db(db_pool.clone()))
        .and_then(handler::create_post))
    .or(post_param
        .and(warp::get())
        .and(with_db(db_pool.clone()))
        .and_then(handler::get_one_post))
    .or(post_param
        .and(warp::delete())
        .and(with_db(db_pool.clone()))
        .and_then(handler::delete_post))
    .recover(error::rejection_handler)
        .with(
            warp::cors()
                .allow_credentials(true)
                .allow_methods(&[
                    Method::OPTIONS,
                    Method::GET,
                    Method::POST,
                    Method::DELETE,
                    Method::PUT,
                ])
                .allow_headers(vec![header::CONTENT_TYPE, header::ACCEPT])
                .expose_headers(vec![header::LINK])
                .max_age(300)
                .allow_any_origin(),
        ); 

    warp::serve(all_posts).run(([127, 0, 0, 1], 3000)).await;
}

fn with_db(db_pool: DBPool) -> impl Filter<Extract = (DBPool,), Error = Infallible> + Clone {
    warp::any().map(move || db_pool.clone())
}