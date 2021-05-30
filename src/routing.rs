use wasm_bindgen::prelude::*;
use web_sys::Url;
use futures_signals::signal::{Signal, SignalExt};
use dominator::routing;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Route {
    NotFound,
    Home,
    Docs,
    Blog,
    API(InnerRoute),
    Example(InnerRoute)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InnerRoute {
    base_path: String,
    hash_path: String,
}

impl Route {
    pub fn signal() -> impl Signal<Item = Self> {
        routing::url()
            .signal_ref(|url| Url::new(&url).unwrap_throw())
            .map(|url| {
                match url.hash().as_str() {
                    "/" => Route::Home,
                    "/docs" => Route::Docs,
                    "/blog" => Route::Blog,
                    a => Route::NotFound,
                }
            })
    }
    
    pub fn from_url(url:&str) -> Self {
        let url = Url::new(&url).unwrap();
        let paths = url.pathname();
        let paths = paths.split("/").into_iter().skip(1).collect::<Vec<_>>();
        let paths = paths.as_slice();
        
        match paths {
            [""] => Route::Home,
            ["docs"] => Route::Docs,
            ["blog"] => Route::Blog,
            a => Route::NotFound,
        }
    }
}

impl From<Route> for String {
	fn from(route:Route) -> Self {
		(&route).into()
	}
}

impl From<&Route> for String {
    fn from(route:&Route) -> Self {
        match route {
            Route::NotFound => "/404".to_string(),
            Route::Home => "/".to_string(),
            Route::Docs => "/docs".to_string(),
            Route::Blog => "/blog".to_string(),
            Route::API(ir) => String::from(ir),
            Route::Example(ir) => String::from(ir)
        }
    }
}

impl From<&InnerRoute> for Route {
    fn from(route:&InnerRoute) -> Self {
        let base = &route.base_path;
        let hash = &route.hash_path;
        let inner = InnerRoute {
            base_path: base.to_string(),
            hash_path: hash.to_string(),
        };
        match base.as_str() {
            "/api" => Route::API(inner),
            "/example" => Route::Example(inner),
            _ => Route::NotFound
        }
    }
}

impl InnerRoute {
    pub fn inner_base(&self) -> String {
        return self.base_path.to_string()
    }
}

impl From<&InnerRoute> for String {
    fn from(route:&InnerRoute) -> Self {
        format!("/{}#{}", route.base_path, route.hash_path)
    }
}

impl From<&str> for InnerRoute {
    fn from(route:&str) -> Self {
        if let Some((base, hash)) = route.split_once('#') {
            return InnerRoute {
                base_path: String::from(base),
                hash_path: String::from(hash)
            }
        }
        InnerRoute {
            base_path: "".to_string(),
            hash_path: "".to_string()
        }
    }
}
