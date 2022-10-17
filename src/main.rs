#[macro_use]
extern crate lazy_static;

use yew::prelude::*;
use yew_router::prelude::*;

mod components;
mod content;
mod pages;
mod randomizer;

pub const TRANSPARENT_B64: &str = "data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAQAAAC1HAwCAAAAC0lEQVR42mNkYAAAAAYAAjCB0C8AAAAASUVORK5CYII=";

#[derive(Routable, PartialEq, Eq, Clone, Debug)]
pub enum Route {
    #[at("/")]
    Home,
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[function_component]
pub fn App() -> Html {
    html! {
        <BrowserRouter>
            <components::Nav />

            <main>
                <Switch<Route> render={switch} />
            </main>

            <footer class="footer">
                <div class="content has-text-centered">
                    { "Powered by " }
                    <a href="https://yew.rs">{ "Yew ♥" }</a>
                </div>
            </footer>
        </BrowserRouter>
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => {
            html! { <pages::RandomLoadout /> }
        }
        Route::NotFound => {
            html! { <pages::PageNotFound /> }
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Info));
    yew::Renderer::<App>::new().render();
}
