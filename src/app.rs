use crate::components::header::*;
use crate::pages::about::*;
use crate::pages::home::*;
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
                        <Route path="" element=|cx| view! {
                            cx,
                            <Home />
                        }/>
                        <Route path="about" element=|cx| view! {
                            cx,
                            <About />
                        }/>
                    </Routes>
                </main>
            </Router>
        </div>
    }
}
