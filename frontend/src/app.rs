//! app.rs
//! Define the routes that the program will accept
use yew::prelude::*;
use yew_router::prelude::*;
use yew_router::Routable;
use crate::pages::*;

/// Enum describing routes in order of NavBar apperance (more or less)
#[derive(PartialEq, Clone, Routable)]
pub enum Route {
    #[at("/")]
    /// Landing page
    Home,
}

/// Main application component
/// The value of 'render' should be a function that returns an Html type
#[component]
pub fn App() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

/// Matcher to pick which route to bring up
fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <LandingPage/> }
    }
}
