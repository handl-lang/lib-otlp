//! `otlp` — `.hnl` contract: conn-as-value telemetry ops over `ffi.call`.
//! Impls live in `lib-otlp-c` (native `otlp_c.*` FFI backend).
#![allow(dead_code)]

pub const OTLP_HNL: &str = include_str!("index.hnl");
pub const OTLP_FUNCS: &[&str] = &[
    "configure","start_span","end_span","set_attribute","add_event",
    "record_log","record_metric","flush","shutdown",
];
