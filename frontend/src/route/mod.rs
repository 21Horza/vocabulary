use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum AppRoute {
    #[at("/create")]
    CreatePost,
    #[at("/:id")]
    ViewPost {id: i32},
    #[at("/")]
    Home,
}