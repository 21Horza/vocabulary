use common::*;
use yew::prelude::*;
use super::api;
use crate::route::AppRoute;
use yew_router::prelude::*;
use gloo::utils::window;

pub enum Msg {
    MakeReq(i32),
    Resp(PostResponse),
    MakeDelReq(i32),
    DelResp(PostResponse)
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub post_id: i32,
}

pub struct Detail {
    id: i32,
    post: Option<PostResponse>,
    deleted_post: Option<PostResponse>,
}

impl Component for Detail {
    type Properties = Props;
    type Message = Msg;

    fn create(_ctx: &Context<Self>) -> Self {
        _ctx.link().callback(Msg::MakeReq);
        Self {
            id: _ctx.props().post_id,
            post: None,
            deleted_post: None
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::MakeReq(id) => {
                self.id = id;
                    _ctx.link().send_future(async move {
                        let data = api::get_one_post(id).await.unwrap();
                        Msg::Resp(data)
                    });
                true
            }
            Msg::Resp(data) => {
                self.post = Some(data);
                true
            }
            Msg::MakeDelReq(id) => {
                self.id = id;
                    _ctx.link().send_future(async move {
                        let data = api::delete_post(id).await.unwrap();
                        Msg::DelResp(data)
                    });
                    window().location().replace("/").unwrap();
                    true
            }
            Msg::DelResp(data) => {
                self.deleted_post = Some(data);
                true
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let post = &self.post;
        let link = _ctx.link().clone();
        let history = link.history().unwrap();
        let id = self.id;
        
        let onclick_back = Callback::once(move |_| history.push(AppRoute::Home));
        let onclick_delete = link.callback(move |_| Msg::MakeDelReq(id));

        match post {
            Some(p) => {
                html! {
                    <div class="view_details">
                        <div class="id">{"ID:"}<h3>{&p.id}</h3></div>
                        <div class="word">{"Word:"}<h3>{&p.word}</h3></div>
                        <div class="def">{"Definition:"}<h3>{&p.def}</h3></div>
                        <div class="pinyin">{"Pinyin:"}<h3>{&p.pinyin.as_deref().unwrap_or_default()}</h3></div>
                        <div class="example">{"Example:"}<h3>{&p.example.as_deref().unwrap_or_default()}</h3></div>
                        <div class="btns">
                            <button onclick={onclick_back} class="back" >
                                    { "Go back" }
                            </button>
                            <button onclick={onclick_delete} class="delete">{"Delete"}</button>
                        </div>
                    </div>
                }
            }
            None => {
                html! {
                    <>
                        <div class="loader"></div>
                        <div class="loading"><h1>{"Post is loading..."}</h1></div>
                    </>
                }
            }
        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, first_render: bool) {
        if first_render {
            let id = _ctx.props().post_id;
            _ctx.link().send_future(async move {
            let data = api::get_one_post(id).await.unwrap();
            Msg::Resp(data)
        });
        }
    }
}