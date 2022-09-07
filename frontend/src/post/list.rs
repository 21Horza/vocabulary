use common::*;
use yew::prelude::*;
use super::api;
use yew_router::prelude::*;
use crate::route::AppRoute;
use gloo::dialogs::alert;
use gloo::utils::window;


pub enum Msg {
    MakeReq,
    Resp(Vec<PostResponse>),
    MakeDelReq(i32),
    DelResp(PostResponse)
}

pub struct List {
    posts: Option<Vec<PostResponse>>,
    id: Option<i32>,
    deleted_post: Option<PostResponse>
}


impl Component for List {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        _ctx.link().send_message(Msg::MakeReq);
        Self {
            id: None,
            posts: None,
            deleted_post: None
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::MakeReq => {
                self.posts = None;
                    _ctx.link().send_future(async {
                        let data = api::get_all_posts().await.unwrap();
                        Msg::Resp(data)
                    });
                true
            }
            Msg::Resp(data) => {
                self.posts = Some(data);
                true
            }
            Msg::MakeDelReq(id) => {
                self.id = Some(id);
                    _ctx.link().send_future(async move {
                    let data = api::delete_post(id).await.unwrap();
                        alert("Post Deleted");
                        Msg::DelResp(data)
                    });
                    window().location().reload().unwrap();
                    true
                }
                Msg::DelResp(data) => {
                self.deleted_post = Some(data);
                alert("Deleted");
                true
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let link = _ctx.link();
        
        if let Some(p) = &self.posts {
            html! {
                <div>
                    <h1 class="title"> {"List of words"} </h1>
                    { p.iter().map(|post| 
                        html! {
                                <div class="view_post">
                                    <div class="word">{"Word:"}<h3>{&post.word}</h3></div>
                                    <div class="def">{"Definition:"}<h3>{&post.def}</h3></div>
                                    <div class="btns">
                                        <button class="view_more">
                                        <Link <AppRoute> to={AppRoute::ViewPost { id: post.id }}> 
                                            { "View More" }
                                        </Link<AppRoute>>
                                        </button>
                                        <button class="delete" onclick={
                                            let id = post.id;
                                            link.callback(move |_| Msg::MakeDelReq(id))
                                        }>{"Delete"}</button>
                                    </div>
                                </div>
                        }
                    ).collect::<Html>() }
                </div>
            }
        } else {
            html! {
                <>
                    <div class="loader"/>
                    <div class="loading"><h2>{"List is loading..."}</h2></div>
                </>
            }
        }
    }
    
    fn changed(&mut self, _ctx: &Context<Self>) -> bool {
        true
    }
}