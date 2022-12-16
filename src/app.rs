use crate::components::header::*;
use crate::pages::dynamic_route::*;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App(cx: Scope) -> Element {
    provide_context(cx, MetaContext::default());

    view! {
        cx,
        <div>
            <Router>
                <Header/>
                <main class="container mx-auto">
                    <Routes>
                        <Route path="*path" element=|cx| view! {
                            cx,
                            <DynamicRoute />
                        }/>
                    </Routes>
                </main>
            </Router>
        </div>
    }
}
