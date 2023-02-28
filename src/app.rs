use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    view! {
        cx,

        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/leptos_start.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes>
                    <Route path="" view=|cx| view! { cx, <HomePage/> }/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage(cx: Scope) -> impl IntoView {
    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(cx, 0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    view! { cx,
        <div class="h-screen bg-stone-900 text-yellow-50">
            <h1 class="text-4xl font-extrabold">"Welcome to web-app-2!"</h1>
                <div class="max-w-sm m-auto text-center">
                    <img
                        src="me.jpg"
                        class="mb-4 w-64 rounded-full shadow-lg dark:shadow-black/30 mx-auto" />
                    <h5 class="mb-4 text-xl font-semibold">"Kyle Zheng"</h5>
                    <h6 class="mb-4 font-semibold">
                      "Societal Burden"
                    </h6>
                    <p class="mb-4 leading-loose">
                      <svg
                        xmlns="http://www.w3.org/2000/svg"
                        fill="currentColor"
                        class="inline-block h-7 w-7 pr-2 align-baseline"
                        viewBox="0 0 24 24">
                        <path
                          d="M13 14.725c0-5.141 3.892-10.519 10-11.725l.984 2.126c-2.215.835-4.163 3.742-4.38 5.746 2.491.392 4.396 2.547 4.396 5.149 0 3.182-2.584 4.979-5.199 4.979-3.015 0-5.801-2.305-5.801-6.275zm-13 0c0-5.141 3.892-10.519 10-11.725l.984 2.126c-2.215.835-4.163 3.742-4.38 5.746 2.491.392 4.396 2.547 4.396 5.149 0 3.182-2.584 4.979-5.199 4.979-3.015 0-5.801-2.305-5.801-6.275z" />
                      </svg>
                      "Fine-grained reactivity is the future of web development. Writing Rust and then compiling to WASM is almost as performant as native Javascript."
                    </p>
                    <button on:click=on_click>"Click Me: " {count}</button>
                </div>
        </div>
    }
}
