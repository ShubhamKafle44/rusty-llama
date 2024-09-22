use crate::model::conversation::Conversation;
use leptos::server;
#[server(Converse, "./api")]
pub async fn converse(prompt: Conversation) -> Result<(), ServetFnError> {
    println!("ehraekjf");
}
