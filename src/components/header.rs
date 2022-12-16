use leptos::*;
use leptos_router::*;

#[component]
pub fn Header(cx: Scope) -> Element {
    view! { cx,
      <div>
        <div class="flex justify-center gap-2 bg-secondary/80 py-0.5 text-xs text-secondary-content uppercase">
          <span>
            "Cyber sale starts now: "
            <a href="/campaigns" class="px-1 underline">
              "Check out our deals"
            </a>
          </span>
        </div>
        <div class="flex items-center h-12 px-8 shadow-md bg-blue-50 lg:pl-2">
          <nav class="hidden px-4 lg:block">
            <ul class="flex gap-4">
              <li class="hover:underline underline-offset-2">
                <A href="/news">"New arrivals"</A>
              </li>
              <li class="hover:underline underline-offset-2">
                <A href="/campaigns">"Campaigns"</A>
              </li>
              <li class="hover:underline underline-offset-2">
                <A href="/bikes">"Bikes"</A>
              </li>
              <li class="hover:underline underline-offset-2">
                <A class="".to_string() href="/accessories">"Accessories"</A>
              </li>
              <li class="hover:underline underline-offset-2">
                <A href="/furniture">"Furniture"</A>
              </li><li class="hover:underline underline-offset-2">
                <A href="/brands">"Brands"</A>
              </li>
            </ul>
          </nav>

          <div class="flex items-center gap-4 ml-auto">
            <form method="get" action="/search" class="px-4 sm:px-8 max-w-md mx-auto">
              <input
                type="search"
                name="term"
                class="w-full px-4 py-1 rounded-full border border-neutral/60 focus-within:ring-1 ring-neutral/60 outline-none"
                placeholder="Search..."
                autocomplete="off" />
            </form>
            <button class="relative" aria-label="Open cart">
              <svg fill="none" viewBox="0 0 24 24" class="w-6 h-6"><path d="M6 2L3 6v14a2 2 0 002 2h14a2 2 0 002-2V6l-3-4zM3 6h18m-5 4a4 4 0 11-8 0" stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5"></path></svg>
            </button>
          </div>
        </div>
      </div>
    }
}
