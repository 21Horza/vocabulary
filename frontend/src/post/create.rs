use common::*;
use yew::prelude::*;
use super::api;
use wasm_bindgen::JsCast;
use web_sys::{HtmlInputElement, EventTarget};

pub struct CreateForm {
    pub word: String,
    pub pinyin: Option<String>,
    pub def: String,
    pub example: Option<String>,
    pub response: Option<PostResponse>
}

pub enum Msg {
    MakeReq,
    Resp(PostResponse),
    EditWord(String),
    EditPinYin(Option<String>),
    EditDef(String),
    EditExample(Option<String>),
}

impl Component for CreateForm {
    type Properties = ();
    type Message = Msg;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            word: String::new(),
            pinyin: None,
            example: None,
            def: String::new(),
            response: None
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let link = _ctx.link();
        let change_word = link.callback(|e: Event| {
            let target: EventTarget = e
                .target().unwrap();
            Msg::EditWord(target.unchecked_into::<HtmlInputElement>().value())
        });
        let change_pinyin = link.callback(|e: Event| {
            let target: EventTarget = e
                .target().unwrap();
            Msg::EditPinYin(Some(target.unchecked_into::<HtmlInputElement>().value()))
        });
        let change_example = link.callback(|e: Event| {
            let target: EventTarget = e
                .target().unwrap();
            Msg::EditExample(Some(target.unchecked_into::<HtmlInputElement>().value()))
        });
        let change_def = link.callback(|e: Event| {
            let target: EventTarget = e
                .target().unwrap();
            Msg::EditDef(target.unchecked_into::<HtmlInputElement>().value())
        });
        html! {
            <>
            <form type="submit">
                    <h1 class="title">{"Add a new word"}</h1>
                    <input type="text" value={self.word.clone()} placeholder="Word..." onchange={change_word}/>
                    <input type="text" value={self.def.clone()} placeholder="Definition..." onchange={change_def}/>
                    <input type="text" value={self.pinyin.clone()} placeholder="PinYin..." onchange={change_pinyin}/>
                    <input type="text" value={self.example.clone()} placeholder="Example..." onchange={change_example}/>
                    <button onclick = {link.callback(move |_| Msg::MakeReq)}>{"Submit"}</button>
                </form>
            </>
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::MakeReq => {
                let dto = PostDto {
                    word: self.word.clone(),
                    pinyin: self.pinyin.clone(),
                    def: self.def.clone(),
                    example: self.example.clone(),
                };
                _ctx.link().send_future(async {
                    let data = api::create_post(dto).await.unwrap();
                    Msg::Resp(data)
                });
                true
            }
            Msg::EditWord(input) => {
                self.word = input;
                true
            }
            Msg::EditDef(input) => {
                self.def = input;
                true
            }
            Msg::EditExample(input) => {
                self.example = input;
                true
            }
            Msg::EditPinYin(input) => {
                self.pinyin = input;
                true
            }
            Msg::Resp(data) => {
                self.response = Some(data);
                true
            }
        }
    }

    fn changed(&mut self, _ctx: &Context<Self>) -> bool {
        true
    }
}