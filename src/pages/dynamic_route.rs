use cfg_if::cfg_if;
use leptos::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Route {
    id: u16,
    name: String,
}

cfg_if! {
    if #[cfg(feature = "ssr")] {
        pub fn register_server_functions() {
            // Silence clippy with the _
            _ = GetRoute::register();
        }
    }
}

#[server(GetRoute, "/api")]
pub async fn get_route(path: String) -> Result<Route, ServerFnError> {
    Ok(Route { id: 1, name: path })
}

#[component]
pub fn DynamicRoute(cx: Scope) -> Element {
    let location = use_location(cx);
    println!("path: {}", location.pathname.get());
    // fetch route data
    let route_data = create_resource(
        cx,
        move || location.pathname.get(),
        move |_| get_route(location.pathname.get()),
    );

    view! { cx,
        <div>
        <Suspense fallback=view!{ cx, <p>"loading"</p> }>
            {move || {
                route_data.read().map(|route| {
                    match route {
                        Err(_) => view!{ cx, <p>"error"</p> },
                        Ok(route) => {
                            view! { cx,
                                <div>
                                    <p>{format!("Route id: {}", route.id)}</p>
                                    <p>{format!("Route name: {}", route.name)}</p>
                                </div>
                            }
                        }
                    }
                })
            }}
        </Suspense>
        </div>
    }
}
