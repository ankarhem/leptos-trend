use leptos::*;

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
              <li class="hover:text-blue-400">
                <a class="" href="/news">"New arrivals"</a>
              </li>
              <li class="hover:text-blue-400">
                <a class="" href="/campaigns">"Campaigns"</a>
              </li>
              <li class="hover:text-blue-400">
                <a class="" href="/bikes">"Bikes"</a>
              </li>
              <li class="hover:text-blue-400">
                <a class="" href="/accessories">"Accessories"</a>
              </li>
              <li class="hover:text-blue-400">
                <a class="" href="/furniture">"Furniture"</a>
              </li><li class="hover:text-blue-400">
                <a class="" href="/brands">"Brands"</a>
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
