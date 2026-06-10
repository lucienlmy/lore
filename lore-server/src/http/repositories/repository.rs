// SPDX-FileCopyrightText: 2026 Epic Games, Inc.
// SPDX-License-Identifier: MIT
pub mod contents;

use axum::Router;
use axum::extract::Path;
use axum::extract::Request;
use axum::middleware;
use axum::middleware::Next;
use axum::response::Response;
use lore_telemetry::tracing::fields::REPOSITORY_ID;
use serde::Deserialize;
use tracing::Span;

use crate::http::server::ServerState;

#[derive(Deserialize)]
struct TracePath {
    repository_id: String,
}

async fn trace(Path(params): Path<TracePath>, request: Request, next: Next) -> Response {
    Span::current().record(REPOSITORY_ID, &params.repository_id);
    next.run(request).await
}

pub fn create_router<S>(shared_state: ServerState) -> Router<S> {
    let contents_router = contents::create_router(shared_state.clone());

    Router::new()
        .nest("/content", contents_router)
        .layer(middleware::from_fn(trace))
        .with_state(shared_state)
}
