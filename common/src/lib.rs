use serde::{Deserialize, Serialize};

#[derive(Deserialize, Clone, PartialEq, Debug)]
pub struct Post {
    pub id: i32,
    pub word: String,
    pub pinyin: Option<String>,
    pub def: String,
    pub example: Option<String>
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct PostDto {
    pub word: String,
    pub pinyin: Option<String>,
    pub def: String,
    pub example: Option<String>
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct PostResponse {
    pub id: i32,
    pub word: String,
    pub pinyin: Option<String>,
    pub def: String,
    pub example: Option<String>
}

impl PostResponse {
    pub fn get(post: Post) -> PostResponse {
        PostResponse {
            id: post.id,
            word: post.word,
            pinyin: post.pinyin,
            def: post.def,
            example: post.example
        }
    }
}