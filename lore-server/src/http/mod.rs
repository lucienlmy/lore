// SPDX-FileCopyrightText: 2026 Epic Games, Inc.
// SPDX-License-Identifier: MIT
pub mod health_check;
pub mod presign_token;
pub mod presigned;
pub mod repositories;
pub mod server;
pub mod tracing;

use lore_transport::grpc::CORRELATION_ID_HEADER;
pub use server::LoreHttpServer;

/// Extracts correlation IDs from `http::Request` headers
pub fn extract_correlation_id<B>(req: &http::Request<B>) -> Option<String> {
    match req.headers().get(CORRELATION_ID_HEADER) {
        Some(val) => val.to_str().map(|s| s.to_string()).ok(),
        None => None,
    }
}
