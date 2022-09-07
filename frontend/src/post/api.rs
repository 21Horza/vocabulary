use common::*;
use gloo_net::http::*;

pub async fn get_all_posts() -> Result<Vec<PostResponse>, anyhow::Error> {
    let url = format!("http://localhost:3000/posts");
    let res = Request::get(&url)
        .send()
        .await?
        .json::<Vec<PostResponse>>()
        .await?;

    Ok(res)
}

pub async fn get_one_post(id: i32) -> Result<PostResponse, anyhow::Error> {
    let url = format!("http://localhost:3000/posts/{}", id);
    let res = Request::get(&url)
        .send()
        .await?
        .json::<PostResponse>()
        .await?;

    Ok(res)
}

pub async fn delete_post(id: i32) -> Result<PostResponse, anyhow::Error> {
    let url = format!("http://localhost:3000/posts/{}", id);
    let res = Request::delete(&url)
        .send()
        .await?
        .json::<PostResponse>()
        .await?;

    Ok(res)
}

pub async fn create_post(dto: PostDto) -> Result<PostResponse, anyhow::Error> {
    let url = format!("http://localhost:3000/posts");
    let res = Request::post(&url)
        .json(&dto)
        .unwrap()
        .send()
        .await?
        .json::<PostResponse>()
        .await?;

    Ok(res)
}