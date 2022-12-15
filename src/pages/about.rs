use leptos::*;
// use leptos_meta::*;

#[component]
pub fn About(cx: Scope) -> Element {
    log!("rendering Settings page");
    view! {
        cx,
        <h1>"About page"</h1>
    }
}
