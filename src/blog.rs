use std::rc::Rc;

use futures_signals::signal::{Signal, SignalExt, Mutable};
use futures_signals::signal_vec::{SignalVec, SignalVecExt, MutableVec};
use dominator::{Dom, text_signal, html, clone, events, link};

use js_sys::Error;
use web_sys::{window, Response, RequestInit, Headers, RequestMode};

use crate::app::App;
use crate::routing::Route;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Blog {
    message: Mutable<String>,
}

impl Blog {
    pub fn render(app: Rc<App>) -> Dom {
        html!("section", {
            .class(["main","my-2","mx-6","d-inline-flex","flex-column"])
            .children(&mut [
                html!("div", {
                    .class(["d-inline-flex","flex-row","flex-items-center"])
                    .text("Hello, Blog")
                }),
            ])
        })
    }
}
