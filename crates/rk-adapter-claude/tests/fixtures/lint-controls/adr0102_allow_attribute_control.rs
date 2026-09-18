//! POSITIVE CONTROL for Checkpoint 7c's `#!?\[allow\(` ban (ADR-0102 d3; s7c §7c.17, §7c.21). Not compiled.
//! Carries BOTH attribute forms — the inner `#![allow(` a crate root would carry and the outer `#[allow(` — on
//! `dead_code` only, so the workspace-wide exactly-one count of the unsafe-code allow (whose token this file
//! must not spell even in prose: the block greps it without a comment filter) is undisturbed by its own control.
#![allow(dead_code)]

#[allow(dead_code)]
pub struct Control;
