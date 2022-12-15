use leptos::*;
use leptos_meta::*;

#[component]
pub fn Home(cx: Scope) -> Element {
    let (count, set_count) = create_signal(cx, 0);

    log!("rendering About page");
    view! {
        cx,
        <div>
        <Stylesheet href="/static/styles/output.css"/>
            <h1>"Home page"</h1>
            <div class="flex gap-2">
                <button class="px-2 bg-slate-300 rounded" on:click=move |_| set_count(0)>"Clear"</button>
                <button class="px-2 bg-slate-300 rounded" on:click=move |_| set_count.update(|value| *value -= 1)>"-1"</button>
                <span>"Value: " {move || count().to_string()}</span>
                <button class="px-2 bg-slate-300 rounded" on:click=move |_| set_count.update(|value| *value += 1)>"+1"</button>
            </div>
        </div>
    }
}
