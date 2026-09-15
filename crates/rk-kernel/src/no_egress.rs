//! **AC-498** — reading the same resolved graph [`crate::adapter_edge`] reads, for the second
//! question D-2 puts to it: is a **networking** crate linked?
//!
//! AC-498: *"GIVEN the test suites / WHEN they run / THEN the `rk-kernel` and pack-validation test
//! suites complete with zero model dispatches and zero outbound network calls, using fakes for all
//! five ports / AND a test that reaches a real adapter fails the suite."*
//!
//! # 🛑 WHAT THIS MODULE CANNOT SEE, AND IT IS THE LARGER HALF OF THE CRITERION
//!
//! This is a reading of a **dependency graph**. It answers *"is a networking crate linked?"* It
//! does **not** answer *"can this process open a socket?"*, and the gap between those two questions
//! is `std`: `std::net::TcpStream::connect` needs no dependency, appears in no `Cargo.toml`, enters
//! no `Cargo.lock` and has no `resolve.nodes[]` entry. Every function here returns a clean verdict
//! over a crate that opens sockets in `std` on every line.
//!
//! **Nor is that hole closed by a lint.** Measured 2026-09-14 at the workspace root:
//! `grep -niE 'net|tcp|socket|reqwest|hyper|ureq|curl' clippy.toml deny.toml` → **0 lines**.
//! `clippy.toml` carries AC-001's clocks, AC-002's ids, AC-003's **two sleeps** and ADR-0083's env
//! setters, and **no network entry at all**. AC-003's network half is enforced by **nothing** and
//! stays **ARMED**.
//!
//! The instrument that does reach that class is the **`no-egress` CI job** — a container with
//! `--network none`, where the `connect` itself fails rather than the manifest that did not declare
//! it (D-2, 2026-09-14). Nothing in this module substitutes for it, and
//! `crates/rk-kernel/tests/it/no_egress.rs` asserts the job is **present** and never that it ran.
//!
//! # Why there is no second graph walker here
//!
//! D-2: *"reuse AC-484's machinery from the previous phase rather than writing a second graph
//! walker."* [`resolved_egress_surface`] calls [`crate::adapter_edge::resolved_reading`] **once**
//! and derives both classes from the one `reached` list, so the adapter answer and the networking
//! answer cannot be taken over two different readings of the graph — which is precisely the failure
//! [`crate::adapter_edge`]'s header records between `resolve` and `packages`. The only walk this
//! file adds is [`subject_dev_dependencies`], which is not a walk at all: it is the subject
//! package's own `dev` rows, one hop, with no closure, and its header says why that is all it is.
//!
//! # The matching rule, and the trap it is written around
//!
//! A `contains` rule over the token `hyper` flags `hyperlocal`, `hyperfine` and `hyperx` — three
//! packages that open no socket — and a rule that fires on innocent packages is the rule that gets
//! deleted rather than fixed (`kernel_denylist.rs`'s header records the same trap one level over,
//! where a naive `contains` fired on the crate's own documentation). [`networking_token`] therefore
//! matches a package name that **is** a token or that begins with the token followed by `-`, which
//! is how every sibling of these seven ships (`tokio-util`, `hyper-util`, `openssl-sys`,
//! `rustls-pki-types`, `curl-sys`).
//!
//! What that rule cannot do is recognise a networking crate whose name carries none of the seven
//! tokens. [`NETWORKING_CRATES`] is a **denylist**, fixed by D-2 from a measurement rather than
//! invented here, and `native-tls`, `ring`, `socket2` and `mio` are outside it. Widening it is a
//! decision and not a local edit.

use std::fmt;

use serde_json::Value;

use crate::adapter_edge::{AdapterEdge, GraphReadError, resolved_reading};

/// The networking crates D-2 names, in D-2's own order.
///
/// **Measured 2026-09-14**: `cargo tree -p rk-kernel -e normal` finds **0** of
/// `reqwest|hyper|tokio|ureq|curl|rustls|openssl`. The decision is that the set *stays* empty, so
/// this array is the measured list transcribed, not a list this module chose. It is `pub` because
/// the test asserts its contents: a token quietly removed here would make the assertion that reads
/// it green for the wrong reason.
pub const NETWORKING_CRATES: [&str; 7] = [
    "reqwest", "hyper", "tokio", "ureq", "curl", "rustls", "openssl",
];

/// The [`NETWORKING_CRATES`] token `package` matches, if any.
///
/// Exact, or the token followed by `-`. See this module's header for the `hyperlocal` trap that
/// forced the second half of that rule and for what the rule cannot reach.
#[must_use]
pub fn networking_token(package: &str) -> Option<&'static str> {
    NETWORKING_CRATES.into_iter().find(|token| {
        package == *token
            || (package.len() > token.len()
                && package.starts_with(token)
                && package.as_bytes()[token.len()] == b'-')
    })
}

/// One networking crate in the resolved graph, the token it matched, and the route to it.
///
/// The route is package names from the subject inclusive to the crate inclusive, for
/// [`AdapterEdge`]'s reason: a finding that named only the crate would leave a reader knowing a
/// socket is linked and not knowing which edge to cut.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NetworkingReach {
    /// The **package** name, hyphenated, as `Cargo.toml` spells it.
    pub package: String,
    /// Which of [`NETWORKING_CRATES`] it matched.
    pub token: &'static str,
    /// `[subject, …, package]`.
    pub route: Vec<String>,
}

impl fmt::Display for NetworkingReach {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} (matched `{}`) via {}",
            self.package,
            self.token,
            self.route.join(" -> ")
        )
    }
}

/// One reading of one crate's resolved graph, for **both** of AC-498's linkage clauses.
#[derive(Debug, Clone)]
pub struct EgressSurface {
    /// The package the walk started at.
    pub subject: String,
    /// How many distinct packages the walk visited, the subject included.
    ///
    /// Carried for the reason [`crate::adapter_edge::AdapterEdgeReport::packages_walked`] is: a
    /// report with no size on it cannot be told apart from a walk that went nowhere, and both say
    /// "nothing linked".
    pub packages_walked: usize,
    /// Every package name the walk reached, sorted. The evidence behind the two empty lists below.
    pub reached: Vec<String>,
    /// AC-484's findings, **unchanged** — this is `resolved_reading`'s own list and not a second
    /// opinion about it.
    pub adapters: Vec<AdapterEdge>,
    /// Every reached package matching [`NETWORKING_CRATES`].
    pub networking: Vec<NetworkingReach>,
}

impl EgressSurface {
    /// AC-498's two linkage clauses as one verdict.
    ///
    /// # Errors
    /// [`EgressViolation`] if any adapter crate or any networking crate is reachable, carrying
    /// every finding and its route so the rendered message names the crate.
    pub fn verdict(&self) -> Result<(), EgressViolation> {
        if self.adapters.is_empty() && self.networking.is_empty() {
            Ok(())
        } else {
            Err(EgressViolation {
                subject: self.subject.clone(),
                adapters: self.adapters.clone(),
                networking: self.networking.clone(),
            })
        }
    }
}

/// AC-498's linkage failure, rendered so CI names what is linked and how it got there.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EgressViolation {
    pub subject: String,
    pub adapters: Vec<AdapterEdge>,
    pub networking: Vec<NetworkingReach>,
}

impl fmt::Display for EgressViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "AC-498: `{}`'s resolved graph carries {} adapter crate(s) and {} networking crate(s)",
            self.subject,
            self.adapters.len(),
            self.networking.len()
        )?;
        for finding in &self.adapters {
            write!(f, "\n  - {finding}")?;
        }
        for finding in &self.networking {
            write!(f, "\n  - {finding}")?;
        }
        Ok(())
    }
}

impl std::error::Error for EgressViolation {}

/// **The reading AC-498's linkage clauses ask for.** One walk of `resolve.nodes[]` from `subject`,
/// classified twice.
///
/// `resolve`, never `packages`, for the reason [`crate::adapter_edge`]'s header gives in full: the
/// manifest view lists an **optional** dependency whether or not its feature is on, and this
/// workspace permanently carries one (D-1's fixture edge), so an assertion over that view would be
/// red on a green tree.
///
/// # Errors
/// [`GraphReadError`] if the output does not parse, its shape changed, or `subject` is not in the
/// graph — exactly as [`resolved_reading`] returns it. A shape change must be **loud**, not read as
/// "nothing linked".
pub fn resolved_egress_surface(
    metadata_json: &str,
    subject: &str,
) -> Result<EgressSurface, GraphReadError> {
    let report = resolved_reading(metadata_json, subject)?;
    let networking = report
        .reached
        .iter()
        .filter_map(|(name, route)| {
            networking_token(name).map(|token| NetworkingReach {
                package: name.clone(),
                token,
                route: route.clone(),
            })
        })
        .collect();
    Ok(EgressSurface {
        subject: report.subject.clone(),
        packages_walked: report.packages_walked,
        reached: report.reached.iter().map(|(n, _)| n.clone()).collect(),
        adapters: report.findings.clone(),
        networking,
    })
}

/// The subject package's **direct** `dev` dependency names, from the manifest view.
///
/// # 🛑 This is one hop and it is not a closure, and the difference is the criterion's
///
/// [`resolved_egress_surface`] walks `kind: null` edges only, because it re-uses AC-484's walk and
/// AC-484 is about what a **consumer** of `rk-kernel` links. AC-498 is about what the **test
/// binary** links, and a test binary links the dev-dependencies too — so a networking crate
/// arriving as a dev-dependency is inside AC-498's subject and outside that walk.
///
/// This function closes the **direct** rows of that hole and nothing more: `dev` edges have no
/// entry in `resolve.nodes[]` distinguishable per kind from here, so a networking crate reached
/// transitively **through** a dev-dependency is not returned and is **not claimed**. The reading is
/// deliberately the manifest view, which is the only place the subject's own `dev` rows are named.
///
/// # Errors
/// [`GraphReadError`] if the output does not parse, its shape changed, or `subject` is not in the
/// graph.
pub fn subject_dev_dependencies(
    metadata_json: &str,
    subject: &str,
) -> Result<Vec<String>, GraphReadError> {
    let root: Value =
        serde_json::from_str(metadata_json).map_err(|e| GraphReadError::Json(e.to_string()))?;
    let packages = root
        .get("packages")
        .and_then(Value::as_array)
        .ok_or(GraphReadError::Shape("packages"))?;
    let package = packages
        .iter()
        .find(|p| p.get("name").and_then(Value::as_str) == Some(subject))
        .ok_or_else(|| GraphReadError::NoSuchPackage(subject.to_owned()))?;
    let deps = package
        .get("dependencies")
        .and_then(Value::as_array)
        .ok_or(GraphReadError::Shape("packages[].dependencies"))?;
    let mut names = Vec::new();
    for dep in deps {
        if dep.get("kind").and_then(Value::as_str) != Some("dev") {
            continue;
        }
        let name = dep
            .get("name")
            .and_then(Value::as_str)
            .ok_or(GraphReadError::Shape("packages[].dependencies[].name"))?;
        names.push(name.to_owned());
    }
    names.sort();
    names.dedup();
    Ok(names)
}
