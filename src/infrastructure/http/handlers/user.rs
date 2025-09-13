use crate::{
    application::use_cases::user::{
        crud::use_case_user_create_persist, execute::use_case_user_execute,
    },
    domain::entities::user::{Query, user},
    infrastructure::cross_cutting::InjectedServices,
    presentation::html::{
        section::main::presentation_html_section_main,
        user::get::presentation_html_user_get_user_input,
    },
};
use axum::{
    Form,
    extract::{Path, State},
    response::Html,
};
use std::collections::HashMap;

pub async fn get_user_handler() -> Html<String> {
    Html(presentation_html_user_get_user_input().to_string())
}

pub async fn post_user_handler(
    State(services): State<InjectedServices>,
    Form(user): Form<user>,
) -> Html<String> {
    Html(use_case_user_execute(services, user.user).await)
}

pub async fn handler_user_create(
    State(services): State<InjectedServices>,
    Path(table): Path<String>,
    Form(data): Form<HashMap<String, String>>,
) -> Html<String> {
    Html(use_case_user_create_persist(services, table, data).await)
}

pub async fn handler_user_execute_query(
    State(services): State<InjectedServices>,
    Form(data): Form<Query>,
) -> Html<String> {
    let _ = services.providers.query.execute(&data.query).await;
    Html(presentation_html_section_main(services).await)
}
