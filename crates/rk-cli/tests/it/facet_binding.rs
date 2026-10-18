//! **`run` binds an `ordered` facet by its declared rank, not as a string** (s7d §7d.5, whose
//! `[facets]` binding names `with_facet_ordinal`; ADR-0048's `ordered = true`). *(Added
//! 2026-10-18, `s8j` build, defect D3 of the S8J-C4 lane proofs.)*
//!
//! Before D3, `build_item` bound every string value with `with_facet_str`, so a pack predicate such
//! as `criticality >= high` compared strings, and `"low" >= "high"` is true: a low-criticality
//! item selected the activities a high one owes.

use rk_kernel::predicate::Tri;
use rk_kernel::select::FacetValue;

use rig::{at_least_high, repo_with_items, resolve};

/// The fixture repository and the predicate probe, in a `#[cfg(test)]` module so the test
/// relaxations reach them (TST-14).
#[cfg(test)]
mod rig {
    use std::path::Path;

    use rk_kernel::predicate::{Tri, evaluate, parse};
    use rk_kernel::select::Item;

    /// The shipped `[facet.criticality]` table's shape: ordered over `low < med < high < unknown`.
    const CRITICALITY: &str = "\n[facet.criticality]\nordered = true\n\
                               values = [\"low\", \"med\", \"high\", \"unknown\"]\nsource = \"human\"\n";

    /// `pack-min` plus an ordered `criticality` facet, and one item per `(id, criticality)` pair.
    pub(super) fn repo_with_items(items: &[(&str, &str)]) -> tempfile::TempDir {
        let repo = crate::support::pack_min_repo(crate::support::PackVariant::Base);
        let rk = repo.path().join(".redkiln");
        let facets = rk.join("facets.toml");
        let mut src = std::fs::read_to_string(&facets).expect("facets.toml reads");
        src.push_str(CRITICALITY);
        std::fs::write(&facets, src).expect("facets.toml writes");
        for (id, criticality) in items {
            std::fs::write(
                rk.join("items").join(format!("{id}.toml")),
                format!(
                    "id    = \"{id}\"\nkind  = \"bug\"\nstage = \"implement\"\n\n[facets]\n\
                     has_repro   = true\ncriticality = \"{criticality}\"\n"
                ),
            )
            .expect("the item writes");
        }
        repo
    }

    /// `run`'s steps (1)-(7) over `item`, which is where `build_item` binds the record.
    pub(super) fn resolve(
        root: &Path,
        item: &str,
    ) -> Result<rk_cli::verbs::run::Resolved, rk_cli::exit::Exit> {
        let cli = rk_cli::cli::Cli {
            cwd: Some(root.to_path_buf()),
            command: rk_cli::cli::Command::Status,
        };
        let args = rk_cli::cli::RunArgs {
            item: item.to_owned(),
            lane: None,
        };
        rk_cli::verbs::run::through_step_seven(
            &cli,
            &args,
            &crate::support::clean_env(),
            rk_cli::verbs::Startup::real(),
            rk_git::reconciled::Entry::Run,
        )
    }

    /// `criticality >= high` over the item's context vector, as a pack row evaluates it.
    pub(super) fn at_least_high(item: &Item) -> Tri {
        let parsed = parse("criticality >= high").expect("the predicate parses");
        evaluate(&parsed.expr, &item.context())
            .expect("an ordinal criticality is orderable against `high`")
            .value
    }
}

/// A `low` item does not satisfy `criticality >= high` and a `high` one does, because both bind as
/// members of the declared domain.
#[test]
fn run_binds_an_ordered_facet_by_its_declared_rank() {
    let repo = repo_with_items(&[("crit-low", "low"), ("crit-high", "high")]);
    let domain: Vec<String> = ["low", "med", "high", "unknown"]
        .iter()
        .map(|v| (*v).to_owned())
        .collect();

    let low = resolve(repo.path(), "crit-low").expect("crit-low resolves");
    assert_eq!(
        low.item.facet("criticality"),
        Some(&FacetValue::Ordinal {
            domain: domain.clone(),
            index: 0,
        }),
        "`criticality = \"low\"` binds as the first member of its declared domain"
    );
    assert_eq!(
        at_least_high(&low.item),
        Tri::False,
        "a low-criticality item satisfies `criticality >= high` (string comparison)"
    );

    let high = resolve(repo.path(), "crit-high").expect("crit-high resolves");
    assert_eq!(
        high.item.facet("criticality"),
        Some(&FacetValue::Ordinal { domain, index: 2 }),
        "`criticality = \"high\"` binds as the third member of its declared domain"
    );
    assert_eq!(
        at_least_high(&high.item),
        Tri::True,
        "CONTROL: a high-criticality item does not satisfy `criticality >= high`"
    );
}

/// A record value outside the facet's declared domain is refused at step (7) as a configuration
/// error, never bound as a string and never a panic.
#[test]
fn run_refuses_an_ordered_facet_value_outside_its_domain() {
    let repo = repo_with_items(&[("crit-bad", "severe")]);
    assert_eq!(
        resolve(repo.path(), "crit-bad").err(),
        Some(rk_cli::exit::Exit::ConfigError),
        "`criticality = \"severe\"` is not a member of the declared domain"
    );
}
