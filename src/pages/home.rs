use leptos::*;

#[component]
pub fn Home(cx: Scope) -> Element {
    let (count, set_count) = create_signal(cx, 0);

    log!("rendering About page");
    view! {
        cx,
        <div>
        // <Stylesheet href="/assets/styles/output.css"/>
            <h1>"Home page"</h1>
            <div class="flex gap-2">
                <button class="px-2 bg-neutral rounded" on:click=move |_| set_count(0)>"Clear"</button>
                <button class="px-2 bg-neutral rounded" on:click=move |_| set_count.update(|value| *value -= 1)>"-1"</button>
                <span>"Value: " {move || count().to_string()}</span>
                <button class="px-2 bg-neutral rounded" on:click=move |_| set_count.update(|value| *value += 1)>"+1"</button>
            </div>
        </div>
    }
}
