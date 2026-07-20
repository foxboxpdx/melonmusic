//! components/mod.rs
//! This module contains reusable function components
use yew::{component, html, Html, Properties};
use yew_router::prelude::*;
use crate::app::Route;

// NavBar - Component defining the navigation sidebar and page links
// NOTE: {"\u{00a0}"} is a non-breaking space
#[component]
pub fn NavBar() -> Html {
    html! {
        <div class="top_column">
            <div class="navigation_sidebar">
                <div class="navigation_logo_wrapper">
                    {"logo.png"}
                </div>

                <div class="navigation_item_list">
                    <div class="navigation_item_wrapper">
                        <div class="navigation_item">
                            <Link<Route> to={Route::Home} classes="navigation_link">
                                {"Home icon"} {"\u{00a0}"} {"Home"}
                            </Link<Route>>
                        </div>
                    </div>
                    // Repeat for each link
                </div>
            </div>
        </div>
    }
}

/* Some SVG icons - Shout-out to mingcute-design */

/// Properties struct for supplying data to the icons
#[derive(Properties, Clone, PartialEq)]
pub struct IconProps {
    #[prop_or_default]
    pub title: String
}

// External link
#[component]
pub fn ExternalLinkIcon(props: &IconProps) -> Html {
    html! {
        <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24">
            <g fill="none">
                <path d="M24 0v24H0V0zM12.593 23.258l-.011.002-.071.035-.02.004-.014-.004-.071-.035c-.01-.004-.019-.001-.024.005l-.004.01-.017.428.005.02.01.013.104.074.015.004.012-.004.104-.074.012-.016.004-.017-.017-.427c-.002-.01-.009-.017-.017-.018m.265-.113-.013.002-.185.093-.01.01-.003.011.018.43.005.012.008.007.201.093c.012.004.023
                    0 .029-.008l.004-.014-.034-.614c-.003-.012-.01-.02-.02-.022m-.715.002a.023.023 0 0 0-.027.006l-.006.014-.034.614c0 .012.007.02.017.024l.015-.002.201-.093.01-.008.004-.011.017-.43-.003-.012-.01-.01z"/>
                <path fill="#09244B" d="M11 6a1 1 0 1 1 0 2H5v11h11v-6a1 1 0 1 1 2 0v6a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2zm9-3a1 1 0 0 1 1 1v5a1 1 0 1 1-2 0V6.414l-8.293 8.293a1 1 0 0 1-1.414-1.414L17.586 5H15a1 1 0 1 1 0-2Z"/>
                <title>{props.title.to_string()}</title>
            </g>
        </svg>
    }
}

// Foxtail icon
#[component]
pub fn FoxTailIcon() -> Html {
    html! {
        <svg width="24px" height="24px" viewBox="0 0 512 512" xmlns="http://www.w3.org/2000/svg">
            <path fill="#000000" d="M144.734 22.04c-5.61-.035-11.163.12-16.634.456-43.77 2.7-82.904 17.003-103.862 44.996-7.506 10.027-5.682 23.91 2.95 31.42 8.635 7.51 23.004 8.053 36.234-.52 22.84-14.805 47.933-9.572 64.27 6.172 16.34 15.745 23.736 41.453 7.54 71.145-19.17 35.143-32.716 96.153-20.146 156.526 12.57 60.374 52.968 119.76 139.728 145.772 33.476 10.036 78.825 16.75 121.645 7.666 44.507-8.788 95.85-34.758 106.892-63.11-9.25 8.885-19.44 15.14-30.202 19.79 18.306-20.92 31.735-49.732 36.79-88.174l2.53-19.24-16.322 10.496c-10.503 6.755-20.585 13.403-30.093 18.396 2.638-5.872 5.038-13.22 7.73-22.777-11.097 15.19-23.73 25.355-38.598 31.472-9.234-.503-18.353-4.867-29.21-16.097-11.358-11.747-18.12-32.095-22.463-57.666-4.344-25.572-6.46-55.927-10.668-86.877-8.42-61.902-25.912-127.873-89.74-161.035-36.955-19.2-79.092-28.577-118.372-28.813zm-.123 18.01c36.462.255 76.11 9.065 110.197 26.774 56.393 29.3 71.994 87.14 80.203 147.488 4.104 30.175 6.186 60.554 10.758 87.465 1.316 7.753 2.835 15.242 4.693 22.385-15.448.04-27.254-8.307-41.704-24.717 7.385 30.41 11.99 36.534 25.705 55.55-28.22-8.235-60.64-34.74-80.95-64.063-3.274 40.047 20.223 71.574 33.275 83.93-25.176-14.196-60.713-41.536-84.623-88.655-1.016 41.426 11.93 87.732 36.45 116.465-34.515-11.536-64.97-99.472-85.42-127.633-13.04 33.217-2.948 89.085 16.072 130.122-19.628-22.838-30.887-49.375-36.555-76.596-11.524-55.342 1.75-113.847 18.325-144.238 19.55-35.842 10.915-71.75-10.85-92.726-21.768-20.976-56.854-27.564-86.554-8.315-8.56 5.55-12.688 3.732-14.626 2.045-1.94-1.687-2.76-3.84-.356-7.053 16.106-21.514 50.135-35.324 90.56-37.817 5.052-.312 10.195-.45 15.403-.414z"/>
        </svg>
    }
}

// Music note icon
#[component]
pub fn NotesIcon() -> Html {
    html! {
        <svg width="24px" height="24px" viewBox="0 0 24 24" class="loading_spinner">
            <g id="music_fill" fill="none" fill-rule="evenodd">
                <path d="M24 0v24H0V0zM12.593 23.258l-.011.002-.071.035-.02.004-.014-.004-.071-.035c-.01-.004-.019-.001-.024.005l-.004.01-.017.428.005.02.01.013.104.074.015.004.012-.004.104-.074.012-.016.004-.017-.017-.427c-.002-.01-.009-.017-.017-.018m.265-.113-.013.002-.185.093-.01.01-.003.011.018.43.005.012.008.007.201.093c.012.004.023 0 .029-.008l.004-.014-.034-.614c-.003-.012-.01-.02-.02-.022m-.715.002a.023.023 0 0 0-.027.006l-.006.014-.034.614c0.012.007.02.017.024l.015-.002.201-.093.01-.008.004-.011.017-.43-.003-.012-.01-.01z"/>
                <path fill="#09244BFF" d="M18.671 3.208A2 2 0 0 1 21 5.18V17a4 4 0 1 1-2-3.465V9.18L9 10.847V18c0 .06-.005.117-.015.174A3.5 3.5 0 1 1 7 15.337v-8.49a2 2 0 0 1 1.671-1.973zM9 8.82l10-1.667V5.18L9 6.847z"/>
            </g>
        </svg>
    }
}


// Loading spinner component
#[component]
pub fn Loading() -> Html {
    html! {
        <div class="loading_spinner_container">
            <NotesIcon/>
            <div class="loading_spinner_text">{"Loading..."}</div>
        </div>
    }
}

// ContentHeader - Reusable component for the content pane's header and title
#[component]
pub fn ContentHeader(props: &IconProps) -> Html {
    let v = format!("v{}", env!("CARGO_PKG_VERSION"));
    html! {
        <div class="content_titlebar_container">
            <div class="content_titlebar">
                <h6>{"MelonMusic Tunesharing System"}</h6>
                <h6>{v.to_string()}</h6>
                {"icon"}
            </div>
            <div class="content_titlebar_title">
                {props.title.to_string()}
            </div>
        </div>
    }
}

// ErrorPage - Component for displaying error messages
#[component]
pub fn ErrorPage(props: &ErrorPageProps) -> Html {
    html! {
        <div class="top_container">
            <NavBar/>
            <div class="content_pane_wrapper">
                <div class="content_pane">
                    <ContentHeader title={"Error!".to_string()}/>
                    <div class="content_body_wrapper">
                        <div class="content_body">
                            <div class="content_card_box">
                                <div class="content_card">
                                    <div class="content_card_titlebar_error">
                                        <div class="content_card_titlebar_text">{"MelonMusic has encountered an error"}</div>
                                    </div>
                                    <div class="content_card_body_twocols">
                                        <div class="ccd_two">{"Calling page:"}</div>
                                        <div class="ccd_two">{props.caller.to_string()}</div>
                                        <div class="ccd_two">{"Error message:"}</div>
                                        <div class="ccd_two">{props.error.to_string()}</div>
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


/// Properties for ErrorPage
#[derive(Properties, Clone, PartialEq)]
pub struct ErrorPageProps {
    pub caller: String,
    pub error: String
}
