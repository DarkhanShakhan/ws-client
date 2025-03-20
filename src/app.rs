use crate::components::chat_view::ChatView;
use leptos::*;
use leptos_meta::*;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Title text="Cybersecurirty AI Tutor"/>
        <Meta name="viewport" content="width=device-width, initial-scale=1.0"/>

        <div class="app-container">
            <header class="app-header">
                <h1>Cybersecurirty AI Tutor</h1>
            </header>
            <main class="app-content">
                <ChatView />
            </main>
            <footer class="app-footer">
                <p>Built with Leptos | &copy; 2025</p>
            </footer>
        </div>
    }
}
