use leptos::*;

#[derive(Clone, Copy, PartialEq)]
pub enum MessageRole {
    Human,
    Assistant,
    System,
}

#[derive(Clone)]
pub struct MessageData {
    pub id: String,
    pub role: MessageRole,
    pub content: String,
}

#[component]
pub fn Message(role: MessageRole, content: String) -> impl IntoView {
    // Function to convert markdown to HTML
    let markdown_to_html = move |markdown: String| {
        use pulldown_cmark::{html, Options, Parser};

        let mut options = Options::empty();
        options.insert(Options::ENABLE_STRIKETHROUGH);
        options.insert(Options::ENABLE_TABLES);

        let parser = Parser::new_ext(&markdown, options);
        let mut html_output = String::new();
        html::push_html(&mut html_output, parser);

        html_output
    };

    let role_class = match role {
        MessageRole::Human => "message-human",
        MessageRole::Assistant => "message-assistant",
        MessageRole::System => "message-system",
    };

    let role_name = match role {
        MessageRole::Human => "You",
        MessageRole::Assistant => "Assistant",
        MessageRole::System => "System",
    };

    view! {
        <div class=format!("message {}", role_class)>
            <div class="message-header">
                <div class="message-role">{role_name}</div>
            </div>
            <div class="message-content markdown-content"
                 inner_html=markdown_to_html(content)>
            </div>
        </div>
    }
}
