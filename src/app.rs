use std::rc::Rc;
use std::cell::Cell;

use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::{JsFuture, spawn_local};
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;

use futures_signals::signal::{Signal, SignalExt, Mutable};
use dominator::{Dom, text_signal, html, clone, events, link, routing};

use js_sys::Error;
use web_sys::{window, Response, RequestInit, Headers, RequestMode};

use crate::routing::Route;
use crate::blog::Blog;
use crate::docs::Docs;

#[derive(Debug)]
pub struct App {
    message: Mutable<String>,
}

const BASE_URL: &str = "http://localhost:8185";

impl App {
    pub fn new() -> Rc<Self> {
        Rc::new(App {
            message: Mutable::new("".to_owned()),
        })
    }

    pub async fn deserialize() -> Rc<Self> {
        let app =  App::new();
        app
    }

    fn render_header(app: Rc<Self>) -> Dom {
        html!("header", {
            .class(["header","m-6","d-inline-flex","flex-row","width-auto","flex-justify-between","flex-wrap"])
            .children(&mut [
                html!("h1", {
                    .class(["h1","m-6","p-6","text-bold","text-center","my-lg-2","width-full"])
                    .text("Hi")
                }),

                html!("ul", {
                    .class(["filters","list-style-none"])
                    .children(&mut [
                        Self::render_button("Docs", Route::Docs),
                        Self::render_button("API", Route::API("api#".into())),
                        Self::render_button("Example", Route::Example("example#".into())),
                        Self::render_button("Blog", Route::Blog),
                    ])
                }),
            ])
        })
    }

    fn render_button(text: &str, route: Route) -> Dom {
        html!("li", {
            .children(&mut [
                link!(String::from(&route), {
                    .text(text)
                    .class_signal("selected", Route::signal().map(move |x| x == route))
                })
            ])
        })
    }

    fn render_footer() -> Dom {
        html!("footer", {
            .class(["footer","p-2","my-2","mx-6","d-inline-flex","flex-column"])
            // Hide if it doesn't have any todos.
        })
    }

    pub fn render(app: Rc<Self>) -> Dom {
        html!("section", {
            .class(["todoapp","bg-gray-light","height-full","d-inline-flex","flex-column","width-full"])
            .child(Self::render_header(app.clone()))
            .child_signal(
                Route::signal()
                    .map(move |route| {
                        let default = html!("section", {
                            .class(["main","my-2","mx-6","d-inline-flex","flex-column"])
                
                            .children(&mut [
                                html!("div", {
                                    .class(["d-inline-flex","flex-row","flex-items-center"])
                                    .text("Hello, world!")
                                }),
                                
                            ])
                        });
                        match route {
                            Route::Home => Some(default),
                            Route::Docs => Some(Docs::render(app.clone())),
                            Route::Blog => Some(Blog::render(app.clone())),
                            _ => Some(default),
                        }
                    })
            )
            .child(Self::render_footer())
        })
    }
}

#[inline]
pub fn trim(input: &str) -> Option<String> {
    let trimmed = input.trim();

    if trimmed.is_empty() {
        None

    } else {
        Some(trimmed.to_owned())
    }
}
