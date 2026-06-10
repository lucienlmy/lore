// SPDX-FileCopyrightText: 2026 Epic Games, Inc.
// SPDX-License-Identifier: MIT
use axum::extract::MatchedPath;
use axum::extract::Request;
use axum::middleware::Next;
use axum::response::Response;
use lore_telemetry::tracing::fields::CORRELATION_ID;
use lore_telemetry::tracing::fields::REPOSITORY_ID;
use lore_telemetry::tracing::fields::USER_AGENT;
use lore_telemetry::tracing::fields::USER_ID;
use lore_transport::grpc::CORRELATION_ID_HEADER;
use tracing::Instrument;

/*
 * Axum middleware that creates a root tracing span for each HTTP request.
 *
 * Because this runs as an axum layer (after routing), MatchedPath is available
 * and used for http.path, avoiding high cardinality from raw URI values.
 *
 * REPOSITORY_ID is recorded later by the repository and presigned trace middlewares.
 * USER_ID is recorded later by the JWT middleware.
 */
pub async fn lore_http_tracing(
    matched_path: Option<MatchedPath>,
    request: Request,
    next: Next,
) -> Response {
    let method = request.method().as_str();
    let path = matched_path
        .as_ref()
        .map_or("<unknown>", MatchedPath::as_str);
    let correlation_id = request
        .headers()
        .get(CORRELATION_ID_HEADER)
        .and_then(|v| v.to_str().ok())
        .unwrap_or("<no_correlation_id>");
    let user_agent = request
        .headers()
        .get("user-agent")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("<no_user_agent>");

    let span = tracing::info_span!(
        parent: None,
        "lore_http_tracing",
        "http.method" = method,
        "http.path" = path,
        { CORRELATION_ID } = correlation_id,
        { USER_AGENT } = user_agent,
        { REPOSITORY_ID } = tracing::field::Empty,
        { USER_ID } = tracing::field::Empty,
    );

    next.run(request).instrument(span).await
}
