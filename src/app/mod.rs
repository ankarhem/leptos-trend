use leptos::*;
use leptos_meta::*;

#[component]
pub fn App(cx: Scope) -> Element {
    provide_context(cx, MetaContext::default());

    view! {
        cx,
        <div class="container mx-auto">
            <h1>"Hi from your Leptos WASM!"</h1>
        </div>
    }
}
