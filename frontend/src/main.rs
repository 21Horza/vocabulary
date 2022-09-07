use yew_router::prelude::*;
use yew::prelude::*;
use yew::html::Scope;

mod post;
mod route;

use route::AppRoute;
use post::{detail::Detail, list::List, create::CreateForm};

pub enum Msg {}
pub enum Post {}

struct App {}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }

    fn changed(&mut self, _ctx: &Context<Self>) -> bool {
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
           <BrowserRouter>
            { self.view_nav(_ctx.link()) }
                <main>
                    <Switch<AppRoute> render={Switch::render(switch)} />
                </main>
           </BrowserRouter>
        }
    }
}

impl App {
    fn view_nav(&self, _link: &Scope<Self>) -> Html {
        html! {
            <div class="navbar">
                <h1> {"🇨🇳 - 🇺🇸 Vocabulary"} </h1>
                <button>
                    <Link<AppRoute> to={AppRoute::Home}>
                        { "Home" }
                    </Link<AppRoute>>
                </button>
                <button>
                    <Link<AppRoute> to={AppRoute::CreatePost}>
                        { "Create Post" }
                    </Link<AppRoute>>
                </button>
            </div>
        }
    }
}

fn switch(routes: &AppRoute) -> Html {
    match routes.clone() {
        AppRoute::Home => {
            html! { <List /> }
        },
        AppRoute::CreatePost => {
            html! { <CreateForm />}
        },
        AppRoute::ViewPost {id} => {
            html! { 
                <Detail post_id={id} /> 
            }
        },
    }
}

pub fn main() {
    yew::start_app::<App>();
}