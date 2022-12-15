use leptos::*;
// use leptos_meta::*;

#[component]
pub fn Home(cx: Scope) -> Element {
    log!("rendering Settings page");
    view! {
        cx,
        <h1>"Home page"</h1>
    }
}
