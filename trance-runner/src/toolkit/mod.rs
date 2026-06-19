//! Toolkit module: platform-specific helpers. Vendored from `runner::toolkit`.
//! Slim version: only the queries that screensaver-security actually uses are
//! preserved. The full library has many more (monitors, GPU, IPC, eBPF,
//! registry, etc.) that this project does not touch.

pub mod platform;
pub mod sys_info;
pub mod theme_query;
