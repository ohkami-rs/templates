mod utils;
mod fetch;
mod components;

use fetch::Client;
use utils::{set_state, report_error};
use components::{};

use crate::models::{};
use yew::prelude::*;
use yew::suspense::{use_future, Suspense};
use std::rc::Rc;


#[function_component]
pub fn App() -> Html {
    html! (
    )
}
