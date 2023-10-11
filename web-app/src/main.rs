mod components;

use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::default::Default;
use crate::components::tdm::TDM;
use crate::components::team::Team;
use crate::components::login::Login;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Default,
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
        Route::TDM => html!{ <TDM/> },
        Route::Team => html!{ <Team/> },
        Route::Login => html!{ <Login/> },
        Route::NotFound => html!{ <img src={"https://http.cat/404"}/> }
    }
}

#[function_component]
fn App() -> Html {
    html! {
        <BrowserRouter>
            <header>
                <nav>
                    <h1>{"RANBOW"}</h1>
                    <h3><a href="https://www.ubisoft.com/en-gb/game/rainbow-six/siege/game-info/operators">{"MORE UP TO DATE THAN UBISOFT ITSELF!"}</a></h3>
                    <br/>
                    <nav>
                        <Link<Route> to={Route::Default}>{"Default"}</Link<Route>>
                        {" / "}
                        <Link<Route> to={Route::TDM}>{"TDM"}</Link<Route>>
                        {" / "}
                        <Link<Route> to={Route::Team}>{"Team"}</Link<Route>>
                    </nav>
                </nav>
            </header>
            <Switch<Route> render={switch}/>
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
