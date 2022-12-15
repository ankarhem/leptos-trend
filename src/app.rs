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
                <header class="px-4 py-2">
                    <nav>
                        <ul class="flex items-center gap-2">
                            <li><a href="/">"Home"</a></li>
                            <li><a href="/about">"About"</a></li>
                        </ul>
                    </nav>
                </header>
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
