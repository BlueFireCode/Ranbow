mod components;
mod logic;

use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::default::Default;
use crate::components::full_game::FullGame;
use crate::components::tdm::TDM;
use crate::components::team::Team;
use crate::components::login::Login;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Default,
    #[at("/FullGame")]
    FullGame,
    #[at("/TDM")]
    TDM,
    #[at("/Team")]
    Team,
    #[at("/Login")]
    Login,
    #[not_found]
    #[at("/404")]
    NotFound
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Default => html!{ <Default/> },
        Route::FullGame => html!{ <FullGame/> },
        Route::TDM => html!{ <TDM/> },
        Route::Team => html!{ <Team/> },
        Route::Login => html!{ <Login/> },
        Route::NotFound => html!{ <img src={"https://http.cat/404"}/> }
    }
}

#[function_component]
fn App() -> Html {
    wasm_logger::init(wasm_logger::Config::default());

    let body = gloo_utils::document().get_element_by_id("main_body").expect("Expected to find main_body element");
    let node = create_portal(html! {
            <script src="https://cdn.jsdelivr.net/npm/bootstrap@5.3.2/dist/js/bootstrap.bundle.min.js" integrity="sha384-C6RzsynM9kWDrMNeT87bh95OGNyZPhcTNXj1NW7RuBCsyN/o0jlpcV8Qyq46cDfL" crossorigin="anonymous"></script>
        }, body);

    html! {
        <div>
            <BrowserRouter>
                <nav class="navbar navbar-expand-lg border-bottom bd-body-tertiary" style="background-color:var(--bs-purple)">
                    <div class="container-fluid">
                        <div class="navbar-brand">
                            <Link<Route> classes={"nav-link mt-1"} to={Route::Default}>
                                <img class="mx-3 mb-1" src="/bluefire-d111e6a6dd18c618.png" heigth="50px" width="50px"/>
                                {"Ranbow"}
                            </Link<Route>>
                        </div>
                        <button class="navbar-toggler" type="button" data-bs-toggle="collapse" data-bs-target="#navbarSupportedContent" aria-controls="navbarSupportedContent" aria-expanded="false" aria-label="Toggle navigation">
                            <span class="navbar-toggler-icon"></span>
                        </button>
                        <div class="collapse navbar-collapse" id="navbarSupportedContent">
                            <ul class="navbar-nav me-auto mb-2 mb-lg-0">
                                <li class="nav-item">
                                    <Link<Route> classes={"nav-link"} to={Route::FullGame}>{"Full Game"}</Link<Route>>
                                </li>
                                <li class="nav-item">
                                    <Link<Route> classes={"nav-link"} to={Route::TDM}>{"TDM"}</Link<Route>>
                                </li>
                                <li class="nav-item">
                                    <Link<Route> classes={"nav-link"} to={Route::Team}>{"Team"}</Link<Route>>
                                </li>
                            </ul>
                            <div class="d-flex">
                                <Link<Route> classes={"btn btn-outline-light"} to={Route::Login}>{"Login"}</Link<Route>>
                            </div>
                        </div>
                    </div>
                </nav>
                <Switch<Route> render={switch}/>
            </BrowserRouter>
            { node }
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
