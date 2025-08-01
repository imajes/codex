//! Root of the `codex-core` library.

// Prevent accidental direct writes to stdout/stderr in library code. All
// user-visible output must go through the appropriate abstraction (e.g.,
// the TUI or the tracing stack).
#![deny(clippy::print_stdout, clippy::print_stderr)]

mod apply_patch;
mod bash;
mod chat_completions;
mod client;
mod client_common;
pub mod codex;
pub use codex::Codex;
pub use codex::CodexSpawnOk;
pub mod codex_wrapper;
pub mod config;
pub mod config_profile;
pub mod config_types;
mod conversation_history;
pub mod error;
pub mod exec;
pub mod exec_env;
mod flags;
pub mod git_info;
mod is_safe_command;
mod mcp_connection_manager;
mod mcp_tool_call;
mod message_history;
mod model_provider_info;
pub use model_provider_info::ModelProviderInfo;
pub use model_provider_info::WireApi;
pub use model_provider_info::built_in_model_providers;
mod models;
mod openai_model_info;
mod openai_tools;
pub mod plan_tool;
mod project_doc;
pub mod protocol;
mod rollout;
mod safety;
pub mod shell;
mod user_notification;
pub mod util;

pub use apply_patch::CODEX_APPLY_PATCH_ARG1;
pub use client_common::model_supports_reasoning_summaries;
