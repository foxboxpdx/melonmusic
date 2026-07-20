//! home.rs
//! Defines a landing page at the root url
use crate::{authheader, Endpoint, get_apiurl};
use crate::components::{ContentHeader, NavBar, ErrorPage};
use yew::{Component, Context, html, Html};
use gloo_net::http::Request;

/// Struct for storing state
struct State {
    data: bool,
    data_error: Option<bool>,
    data_loaded: bool,
}

/// Struct defining the page component
pub struct LandingPage {
    state: State
}

/// Enum defining messages that can be sent to update the page
pub enum Msg {
    GetData,
    GetDataSuccess(bool),
    GetDataError(bool)
}

// Implement required types and functions to create a Struct Component
impl Component for LandingPage {
    type Message = Msg;
    type Properties = ();

    /// Called upon instantiation of the component
    fn create(_ctx: &Context<Self>) -> Self {
        Self { state: State { data: false, data_error: None, data_loaded: false }}
    }

    /// Determine whether to update the page and how to do so
    fn update(&mut self, ctx: &Context<Self>, message: Self::Message) -> bool {
        match message {
            Msg::GetData => {
                self.state.data_loaded = false;
                let ep = Endpoint::Status;
                ctx.link().send_future(async move {
                    let endpoint = format!("{}/{ep}", get_apiurl());
                    let response = Request::get(&endpoint).headers(authheader()).send();
                    let retval = match response.await {
                        Ok(x) => {
                            match x.json::<bool>().await {
                                Ok(y) => Msg::GetDataSuccess(y),
                                Err(_e) => Msg::GetDataError(true)//(e.into())
                            }
                        },
                        Err(_e) => Msg::GetDataError(true)//(e.into())
                    };
                    retval
                });
                true
            }
            Msg::GetDataSuccess(data) => {
                self.state.data = data;
                self.state.data_loaded = true;
                true
            }
            Msg::GetDataError(err) => {
                self.state.data_error = Some(err);
                self.state.data_loaded = true;
                true
            }
        }
    }

    /// Determine if page has been rendered yet
    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            ctx.link().send_message(Msg::GetData);
        }
    }

    /// How to actually render the page
    fn view(&self, _ctx: &Context<Self>) -> Html {
        if !self.state.data_loaded {
            html! { {"Loading..."} }
        } else if let Some(_e) = &self.state.data_error {
            // Error page
            html! { <ErrorPage caller={"Landing Page"} error={"Oh no"} /> }
        } else {
            html! {
                <div class="top_container">
                    <NavBar/>
                    <div class="top_column">
                        <div class="content_pane_wrapper">
                            <div class="content_pane">
                                <ContentHeader title={"Welcome"} />
                                <div class="content_body_wrapper">
                                    <div class="content_body">
                                        <div class="content_card_box">
                                            <div class="content_card">
                                                <div class="content_card_titlebar cctb_onecol">
                                                    <div class="content_card_titlebar_text">{"Welcome"}</div>
                                                </div>
                                                <div class="ccb_three">
                                                    <div class="content_card_header">{"Foo"}</div>
                                                    <div class="content_card_header">{"Bar"}</div>
                                                    <div class="content_card_header">{"Baz"}</div>
                                                </div>
                                            </div>
                                        </div>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            }
        }
    }
}
