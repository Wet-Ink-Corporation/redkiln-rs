//! **The memo** — AC-014's nine cases, AC-015's edit, AC-016's third assertion, AC-017's two
//! iterations, AC-018's three fixtures and AC-019's two halves.
//!
//! # 🛑 WHAT IS MEASURED HERE AND WHAT IS NOT
//!
//! | criterion | measured here | **not** here |
//! | --- | --- | --- |
//! | **AC-014** | the eight components, one perturbation each plus the must-hit; the `u32-length \|\| bytes` framing; component 4's canonicalisation **by `rk_store::canon`** and its declaration order; component 6's sortedness; and — as of 2026-09-14 — component 8 being `ActivitySpec.schema`'s **whole id**, with the same-version/different-kind pin the rejected `vN` parse fails | component 5's **tool branch** — `Toolchain::detect()` returns a `bool` and there is no adapter at 7b (residual 5b) — which still arrives as a caller-supplied value and is still **ARMED**. Component 8's source is no longer among them; what `schema_version` formally **is** stays a record **OWED**, so the pin below carries no AC id |
//! | **AC-015** | a `memo_miss` **then** a re-execution, in that order, over the projection; the resolved-atom-set reading; the unchanged-activation **control** | the real append. Nothing in `rk-kernel::workflow` appends; Slice 4h's boundary does. **ARMED** |
//! | **AC-016** | all three instruments, the third one first: the activity key is **unchanged** when a step's resolved inputs change | nothing. This is the one criterion in the file whose every named instrument is here |
//! | **AC-017** | two iterations, byte-identical inputs, two keys **and** two executions, with the same-iteration control | nothing beyond the projection caveat AC-015 carries |
//! | **AC-018** | the two clauses, the three fixtures (**two must-fail, one must-load**) and the shipped catalog's **nineteen** live violations | the CLI surface (`rk-cli`, Step 7d) and the repairs (Step 7e). The rule is **red against `spikes/p9/catalog/` by design** and this file asserts that by number |
//! | **AC-019** (1/2) | the nine JSON members of `memo_hit`'s payload and every value | 🛑 the register's *"event-schema test asserting the field set"*. `Event.payload` is an untyped `serde_json::Value` and the typed-payload record is **OWED** (`crates/rk-types/src/event.rs`:46-59); the assertion is over **JSON keys** and nothing stronger. **ARMED** (residual 10j) |
//! | **AC-019** (2/2) | the identity `executed steps + memo_hit events == step activations`, over a graph whose activations outnumber its static steps for **both** of the register's reasons, with four adverse journals proving it can fail | the identity over a **dispatched** run. The journal is a projection; a run with real dispatches needs Slice 4h |
//! | **AC-043** | nothing | its `Evicted` decision is not built. ADR-0119 clause 7 owes the register a dated amendment for the third state and `00-preamble.md`:55 forbids a stage taking it |
//! | **AC-045** | nothing | the selection memo key is `rk-kernel::select`'s. **ARMED** under ADR-0119 clause 8 |
//!
//! # What these three names add beyond `rk-store`'s own tests, and what they re-run
//!
//! `crates/rk-store/tests/it/canon_and_memo.rs` already carries
//! `every_component_participates_in_the_key` (:89), `the_key_is_stable` (:164) and
//! `the_activity_key_is_not_the_step_key` (:174), and honesty about the overlap is owed
//! (residual 5a). What is **re-run** here: that each of the eight fields moves the digest.
//! What is **new**, and what `rk-store` structurally cannot assert because it has no
//! `StepContext`:
//!
//! 1. **The assembly.** `rk-store` perturbs its own struct. This file perturbs the **runtime fact**
//!    and goes through [`StepContext::components`], so a build that dropped a fact on the floor —
//!    read the rigor and never wrote it to component 2, say — is red here and green there.
//! 2. **Component 4's canonicalisation.** Two inputs that are the same object with keys inserted in
//!    a different order must give the **same** key. `serde_json` carries `preserve_order`
//!    workspace-wide (root `Cargo.toml`:44), so an assembly using `to_string` passes `rk-store`'s
//!    tests and fails this one.
//! 3. **AC-016's third assertion and its lint.** `rk_store::memo::activity_key` accepts a
//!    one-element slice happily; [`activity_key_rule`] does not, and the invariance of the activity
//!    key to a step's resolved inputs is the assertion a derive-one-from-the-other build fails.
//! 4. **AC-017's two executions**, AC-015's ordering, AC-018 and AC-019 in whole: `rk-store` has no
//!    memo pass, no journal, no catalog and no verb registry.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use rk_kernel::catalog::{Catalog, StepKind};
use rk_kernel::memo::{
    ActivationTally, CardOrTool, MEMO_HIT_PAYLOAD_KEYS, MemoDecision, MemoError, MemoLedger,
    MemoLintError, MemoViolation, StepContext, VerbRegistry, VerbRegistryError, activity_key_rule,
    consult, fold, live_verb_memo_rule, memo_hit_payload, recorded_residue, tally_activations,
    verb_of,
};
use rk_kernel::rigor::Rigor;
use rk_store::memo::StepKeyComponents;
use rk_types::activity::ActivitySpec;
use rk_types::digest::{Digest, sha256};
use rk_types::event::Event;
use rk_types::event_kind::EventKind;
use rk_types::typed::SchemaId;
use serde_json::json;

/// The shipped catalog, relative to the workspace root. On a non-comment line deliberately, for
/// `tests/it/rigor.rs`:41-43's reason: *"passes on the catalog as shipped"* asserted over a fixture
/// of the implementer's choosing is not that claim.
const SHIPPED_CATALOG: &str = "spikes/p9/catalog";

const FIXTURE_VERBS: &str = include_str!("../fixtures/registries/verbs/verbs-live.toml");
const FIXTURE_VERBS_NO_LIVE: &str =
    include_str!("../fixtures/registries/verbs/verbs-no-live-flag.toml");
const FIXTURE_LIVE_MEMOISED: &str = include_str!("../fixtures/catalog/live-verb-memoised.toml");
const FIXTURE_MEMO_FALSE: &str = include_str!("../fixtures/catalog/step-declares-memo-false.toml");
const FIXTURE_NON_LIVE: &str = include_str!("../fixtures/catalog/non-live-verb-memoises.toml");
const FIXTURE_ACTIVATIONS: &str = include_str!("../fixtures/catalog/memo-activations.toml");

fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .canonicalize()
        .expect("the workspace root resolves from this crate's manifest directory")
}

/// A digest that is a real SHA-256 of a named seed, so two fixtures never accidentally share one
/// and a reader can see where each came from.
fn d(seed: &str) -> Digest {
    sha256(seed.as_bytes())
}

fn catalog(name: &'static str, src: &'static str) -> Catalog {
    Catalog::from_sources(&[(name, src)])
        .unwrap_or_else(|e| panic!("the {name} fixture loads: {e}"))
}

fn verbs() -> VerbRegistry {
    VerbRegistry::from_toml(FIXTURE_VERBS).expect("the verb-registry fixture loads")
}

/// The baseline eight runtime facts. **Every component non-empty and every one distinct**, so a
/// perturbation of one cannot be mistaken for a perturbation of another and an assembly that
/// crossed two wires is red.
fn baseline() -> StepContext {
    StepContext {
        activity_id_at_version: "memo-activations@2".to_owned(),
        rigor: Rigor::Recommended,
        step_id: "draft".to_owned(),
        resolved_inputs: vec![json!({"spec": "INC-1", "shape": "text"}), json!([1, 2, 3])],
        card_or_tool: CardOrTool::Card(d("card/implementer@v4")),
        kb_atoms: vec![d("kb/atom-a@v1"), d("kb/atom-b@v1")],
        loop_iteration: 0,
        output_schema: SchemaId::new("RecordedResult/v1"),
    }
}

fn key_of(ctx: &StepContext) -> Digest {
    ctx.key()
        .unwrap_or_else(|e| panic!("the {} context assembles: {e}", ctx.step_id))
}

/// An [`ActivitySpec`] differing from its siblings **only in `schema`**, so a key that moved
/// moved because component 8 did. The other four fields are ADR-0099:61's, filled with the shape
/// `tests/it/degradation.rs`:60 already uses.
fn spec_with_schema(id: &str) -> ActivitySpec {
    ActivitySpec {
        card_digest: d("card/implementer@v4"),
        tools: vec!["Read".to_owned()],
        schema: SchemaId::new(id),
        timeout_secs: 900,
        inputs: vec![("item".to_owned(), json!("issue-81"))],
    }
}

/// [`baseline`]'s seven other runtime facts, with **component 8 taken from the spec** and from
/// nowhere else — the decision of 2026-09-14 written as a call rather than as a comment.
fn ctx_from_spec(spec: &ActivitySpec) -> StepContext {
    StepContext {
        output_schema: spec.schema.clone(),
        ..baseline()
    }
}

/// The assembled components with **component 8 blanked**: everything a premise of the form *"and
/// nothing else moved"* has to hold fixed, compared at `rk-store`'s boundary rather than at this
/// file's, so a wire crossed during assembly is caught by the premise and not hidden by it.
fn held_fixed(ctx: &StepContext) -> StepKeyComponents {
    let mut c = ctx
        .components()
        .unwrap_or_else(|e| panic!("the {} context assembles: {e}", ctx.step_id));
    c.schema_version = String::new();
    c
}

fn kinds(journal: &[Event]) -> Vec<EventKind> {
    journal.iter().map(|e| e.kind).collect()
}

fn tally(journal: &[Event]) -> ActivationTally {
    tally_activations(journal).unwrap_or_else(|e| panic!("the journal tallies: {e}"))
}

// ---- AC-014 -------------------------------------------------------------------------------------

/// **AC-014.** *"GIVEN the step memo key — `SHA-256` over a length-prefixed (`u32-length || bytes`),
/// ordered tuple of exactly these eight components … WHEN any one component changes and all others
/// are held fixed THEN the key changes and the memo lookup misses."*
///
/// *Verified by:* *"a property test … parameterised over each key component (**eight** cases, one
/// per component), each asserting a miss; and a companion test asserting a hit when all eight are
/// identical."* — **nine cases**, and they are the first nine assertions below.
///
/// The perturbation is applied to the **runtime fact** ([`StepContext`]) and not to
/// `StepKeyComponents`, which is what makes this more than a re-run of `rk-store`'s own
/// `every_component_participates_in_the_key`: an assembly that read a fact and never wrote it to
/// its component is green there and red here. Four further assertions follow the nine, each for a
/// clause of the GIVEN that a bare eight-way perturbation does not reach: the **framing**, the
/// **canonicalisation** of component 4, its **declaration order**, and component 6's **sortedness**.
#[test]
fn memo_key_eight_components() {
    // The record's count and the struct's count are the same number, or "eight" is a comment.
    assert_eq!(
        StepKeyComponents::COMPONENT_COUNT,
        8,
        "ADR-0097 decision 2 fixes eight components; rk-store's own constant is what this file \
         counts against"
    );

    let base = baseline();
    let base_key = key_of(&base);

    // ---- the eight must-miss cases, one per component, all others held fixed ------------------
    let mut c1 = base.clone();
    c1.activity_id_at_version = "memo-activations@3".to_owned();
    let mut c2 = base.clone();
    c2.rigor = Rigor::Mandatory;
    let mut c3 = base.clone();
    c3.step_id = "settle".to_owned();
    let mut c4 = base.clone();
    c4.resolved_inputs = vec![json!({"spec": "INC-2", "shape": "text"}), json!([1, 2, 3])];
    let mut c5 = base.clone();
    c5.card_or_tool = CardOrTool::Card(d("card/implementer@v5"));
    let mut c6 = base.clone();
    c6.kb_atoms = vec![d("kb/atom-a@v2"), d("kb/atom-b@v1")];
    let mut c7 = base.clone();
    c7.loop_iteration = 1;
    let mut c8 = base.clone();
    // A VERSION bump, which is the move AC-014 names: `RecordedResult/v1` -> `/v2`. Component 8
    // is the spec's whole schema id as of 2026-09-14 (see the module header), so the case that
    // perturbs it perturbs an id and not a bare number.
    c8.output_schema = SchemaId::new("RecordedResult/v2");

    let cases: [(&str, &StepContext); 8] = [
        ("1 activity_id@version", &c1),
        ("2 rigor", &c2),
        ("3 step_id", &c3),
        ("4 resolved inputs", &c4),
        ("5 card digest", &c5),
        ("6 kb atoms", &c6),
        ("7 loop iteration", &c7),
        ("8 output schema id", &c8),
    ];
    let mut seen: BTreeSet<Digest> = BTreeSet::new();
    seen.insert(base_key.clone());
    for (name, ctx) in cases {
        let k = key_of(ctx);
        assert_ne!(
            k, base_key,
            "component {name} changed and the key did not — the memo lookup would HIT on a \
             changed step, which is AC-014's THEN inverted"
        );
        assert!(
            seen.insert(k),
            "component {name}'s perturbed key collides with an earlier one: eight components that \
             share a key are not eight components"
        );
    }
    assert_eq!(
        seen.len(),
        9,
        "the baseline and its eight perturbations are nine distinct keys"
    );

    // ---- the ninth case: all eight identical must HIT -----------------------------------------
    let twin = baseline();
    assert_eq!(
        key_of(&twin),
        base_key,
        "two contexts carrying the same eight components must give the same key, or the memo never \
         hits and every assertion above is satisfied by a random number"
    );

    // ---- the GIVEN's framing clause: `u32-length || bytes`, ordered ---------------------------
    // A character moved ACROSS the boundary between components 1 and 3, with the other six held
    // fixed. Under an unframed concatenation these two are the same byte string and the same key.
    let mut left = base.clone();
    left.activity_id_at_version = "memo-activations@2d".to_owned();
    left.step_id = "raft".to_owned();
    let mut right = base.clone();
    right.activity_id_at_version = "memo-activations@2".to_owned();
    right.step_id = "draft".to_owned();
    assert_eq!(
        key_of(&right),
        base_key,
        "premise: the right-hand context IS the baseline"
    );
    assert_ne!(
        key_of(&left),
        key_of(&right),
        "the framing is `u32-length || bytes`: moving one character across a component boundary \
         must move the key. An unframed concatenation gives these two the same bytes"
    );

    // ---- the GIVEN's canonicalisation clause: component 4 goes through rk_store::canon --------
    // `serde_json` carries `preserve_order` WORKSPACE-WIDE (root Cargo.toml:44), so an assembly
    // that used `to_string` leaks insertion order into the key and fails here while passing every
    // perturbation above.
    let mut reordered_keys = base.clone();
    reordered_keys.resolved_inputs =
        vec![json!({"shape": "text", "spec": "INC-1"}), json!([1, 2, 3])];
    assert_ne!(
        serde_json::to_string(&reordered_keys.resolved_inputs[0]).expect("serialises"),
        serde_json::to_string(&base.resolved_inputs[0]).expect("serialises"),
        "premise: with `preserve_order` on, the two objects DO serialise differently — without this \
         the assertion below is vacuous"
    );
    assert_eq!(
        key_of(&reordered_keys),
        base_key,
        "component 4 is *\"canonicalised by rk-store's canonical serialiser (sorted keys, fixed \
         float format, LF)\"*: the same object with its keys inserted in another order is the same \
         input and must be the same key"
    );

    // ---- ... and it is "in declaration order", so SWAPPING two different inputs moves the key --
    let mut swapped = base.clone();
    swapped.resolved_inputs = vec![json!([1, 2, 3]), json!({"spec": "INC-1", "shape": "text"})];
    assert_ne!(
        key_of(&swapped),
        base_key,
        "component 4 is the digest of each resolved input IN DECLARATION ORDER; a set would make \
         these two the same key"
    );

    // ---- component 6 is "sorted", and the sort is rk-store's ---------------------------------
    let mut atoms_reordered = base.clone();
    atoms_reordered.kb_atoms = vec![d("kb/atom-b@v1"), d("kb/atom-a@v1")];
    assert_ne!(
        atoms_reordered.kb_atoms, base.kb_atoms,
        "premise: the two atom lists really are in different orders"
    );
    assert_eq!(
        key_of(&atoms_reordered),
        base_key,
        "component 6 is *\"the digests of the resolved KB atoms, SORTED\"* — a caller that passed \
         them in resolution order must not get a different key for the same set"
    );

    // ---- component 5's two branches cannot collide -------------------------------------------
    let card = d("card/implementer@v4");
    let mut as_tool = base.clone();
    as_tool.card_or_tool = CardOrTool::Tool {
        detected_version: card.as_str().to_owned(),
        command: String::new(),
    };
    assert_ne!(
        key_of(&as_tool),
        base_key,
        "a tool whose detected version happens to be 64 lowercase hex must not share a key with \
         the card of that digest — see the module header's stated reading on the branch tag"
    );

    // ---- the two refusals: neither component is allowed a silent default ----------------------
    let mut no_schema = base.clone();
    no_schema.output_schema = SchemaId::new("");
    assert_eq!(
        no_schema.components().unwrap_err().as_str(),
        "schema_version_not_supplied",
        "component 8 has a source as of 2026-09-14 \u{2014} the spec's schema id \u{2014} and still no code \
         default: an empty id is refused rather than hashed, because `\"\"` is a default wearing \
         a string's clothes"
    );
    let mut floaty = base.clone();
    floaty.resolved_inputs = vec![json!({"ratio": 0.5})];
    assert_eq!(
        floaty.components().unwrap_err().as_str(),
        "canon",
        "rk_store::canon refuses a float rather than choosing a format, and component 4 inherits \
         that refusal rather than routing around it"
    );

    // The assembly IS rk-store's derivation and not a second one.
    let comps = base.components().expect("the baseline assembles");
    assert_eq!(
        base_key,
        comps.digest(),
        "ADR-0119 clause 1: `StepKeyComponents` does not move, and this module ASSEMBLES it. A key \
         computed here rather than by rk-store is the accidental equality the two crates' headers \
         exist to prevent"
    );
}

/// **Component 8 is `ActivitySpec.schema`'s WHOLE id** — the operator decision of 2026-09-14
/// (D-10), and the pin on the reading that was rejected with it.
///
/// **The name carries no AC id on purpose.** AC-014 calls component 8 *"the output type's
/// `schema_version`"*, and **no `schema_version` field exists anywhere in `rk-types` or
/// `rk-ports`** [measured 2026-09-14: `grep -rn schema_version crates/rk-types/src
/// crates/rk-ports/src` → 0]. Design §3.5 names a field that does not exist, so **a dated note
/// saying what `schema_version` formally is remains OWED**. Until it lands, a test named for a
/// criterion would claim a settlement no record has made —
/// `tests/it/budget.rs`'s `appetite_raise_from_an_activity_result_is_refused` is the precedent, and
/// `tests/it/fsm.rs`'s expiry test the second.
///
/// **What the decision chose**: the spec's schema id, whole and unparsed
/// ([`rk_types::activity::ActivitySpec::schema`]). It is the only value in the tree carrying a
/// version, it is already on the spec where the key is built, and AC-014's property holds —
/// `v1` → `v2` moves the key.
///
/// **What it rejected, and what assertion 3 below is the pin for**: parsing the `vN` suffix. That
/// invents a convention no record states, has no defined behaviour on an id without a suffix, and
/// makes **every output type at `/v1` share one component 8** — so two different output types
/// collide there, and the nine-case property `memo_key_eight_components` exists to pin is weakened
/// to a version check. Assertion 3 is red under that reading and green under this one.
#[test]
fn component_eight_is_the_whole_schema_id_and_never_a_parsed_version() {
    // ---- 1. the WHOLE id reaches rk-store's component 8, character for character ---------------
    let recorded = spec_with_schema("RecordedResult/v1");
    let base = ctx_from_spec(&recorded);
    let comps = base.components().expect("the baseline assembles");
    assert_eq!(
        comps.schema_version,
        recorded.schema.as_str(),
        "component 8 IS `ActivitySpec.schema`'s id — assembled from the spec, not re-spelled by a \
         literal in this test and not derived from anything else"
    );
    assert_eq!(
        comps.schema_version, "RecordedResult/v1",
        "and the whole id is what lands: the kind AND the version, in one string"
    );
    for parsed in ["v1", "1", "RecordedResult"] {
        assert_ne!(
            comps.schema_version, parsed,
            "component 8 must not be a PART of the id. `{parsed}` is what a suffix (or prefix) \
             parse would leave, and the decision of 2026-09-14 refused exactly that"
        );
    }

    // ---- 2. AC-014's property over the version: v1 -> v2 moves the key -------------------------
    let bumped = spec_with_schema("RecordedResult/v2");
    let after_bump = ctx_from_spec(&bumped);
    assert_eq!(
        held_fixed(&base),
        held_fixed(&after_bump),
        "premise: the bump moved component 8 and nothing else, so the miss below is attributable \
         to component 8 alone"
    );
    assert_ne!(
        key_of(&base),
        key_of(&after_bump),
        "*\"(8) the output type's `schema_version`\"*: a schema bump must miss the memo. A \
         component 8 that stood still across v1 -> v2 would serve the OLD shape's memoised result \
         for the NEW shape, which inverts AC-014's THEN"
    );

    // ---- 3. THE PIN ON THE REJECTED READING: same version, different output KIND --------------
    // Both ids end in the SAME version. Under the rejected `vN` parse both components 8 are `v1`
    // and these two keys are EQUAL — one output type's memoised result served to another.
    let other_kind = spec_with_schema("ImplReport/v1");
    assert_ne!(
        other_kind.schema.as_str(),
        recorded.schema.as_str(),
        "premise: two different schema ids"
    );
    assert!(
        other_kind.schema.as_str().ends_with("/v1") && recorded.schema.as_str().ends_with("/v1"),
        "premise: and they agree on the VERSION — without this the assertion below is satisfied by \
         the version parse it exists to refuse, and is vacuous"
    );
    let other = ctx_from_spec(&other_kind);
    assert_eq!(
        held_fixed(&base),
        held_fixed(&other),
        "premise: the other seven components are identical across the two kinds"
    );
    assert_ne!(
        key_of(&base),
        key_of(&other),
        "two different OUTPUT TYPES at the same version are different work and must be different \
         keys. This is the collision the rejected `vN` parse would introduce, and the reason \
         component 8 is the whole id"
    );

    // ---- 4. and the refusal survives the retyping: an empty id is not hashed -------------------
    let mut empty = ctx_from_spec(&spec_with_schema(""));
    assert_eq!(
        empty.components().unwrap_err().as_str(),
        "schema_version_not_supplied",
        "an empty schema id is still REFUSED rather than hashed: `\"\"` is a code default wearing \
         a string's clothes, and a typed component 8 does not make an empty one meaningful"
    );
    empty.output_schema = recorded.schema;
    assert!(
        empty.components().is_ok(),
        "control: the SAME context with the spec's id restored assembles, so the refusal above is \
         component 8's and not some other field's"
    );
}

// ---- AC-016 -------------------------------------------------------------------------------------

/// **AC-016.** *"THEN a key exists per *step* (AC-014's tuple), and the activity-level key the
/// stage sees is a **separately derived** value — `SHA-256` over the digests of every card reachable
/// in the subgraph — not the step key and not a hash of it."*
///
/// *Verified by:* three instruments — *"a unit test asserting that changing one card deep in a
/// subgraph changes the activity-level key; a lint that rejects an activity-level key computed from
/// a single card reference; a unit test that the two keys are different formulas (the activity-level
/// key is unchanged when a step's resolved inputs change)."*
///
/// **The third is the discriminating one and it is asserted first.** A build that derived one key
/// from the other passes instruments 1 and 2 and fails 3.
#[test]
fn memo_activity_key_distinct() {
    let cards = vec![
        d("card/implementer@v4"),
        d("card/adversary@v2"),
        d("card/reviewer@v1"),
    ];
    let activity_key = activity_key_rule(&cards).expect("three reachable cards is a lawful key");

    // ---- INSTRUMENT 3, first: the activity key does NOT move when a step's inputs move ---------
    let step_a = baseline();
    let mut step_b = baseline();
    step_b.resolved_inputs = vec![json!({"spec": "INC-9", "shape": "code"}), json!([9])];
    assert_ne!(
        key_of(&step_a),
        key_of(&step_b),
        "premise: the STEP key moves when a step's resolved inputs move — without this the \
         assertion below is true of a key that never moves at all"
    );
    assert_eq!(
        activity_key_rule(&cards).expect("still lawful"),
        activity_key,
        "the activity-level key is a DIFFERENT FORMULA: it is over the reachable card digests, so \
         a step's resolved inputs are not among its arguments. A build that derived one key from \
         the other is red exactly here and nowhere else"
    );
    assert_ne!(
        activity_key,
        key_of(&step_a),
        "and the two values are not equal: the activity key is not the step key, and not a hash of \
         it"
    );

    // ---- INSTRUMENT 1: one card deep in the subgraph -----------------------------------------
    let mut deep = cards.clone();
    deep[2] = d("card/reviewer@v2");
    assert_ne!(
        activity_key_rule(&deep).expect("still lawful"),
        activity_key,
        "changing one card deep in the subgraph must change the activity-level key"
    );

    // The rule does not re-derive: it is rk-store's formula with a refusal in front of it.
    assert_eq!(
        activity_key,
        rk_store::memo::activity_key(&cards),
        "ADR-0119 clause 1 again — `activity_key` in rk-store does not move, and this is the lint \
         and not a second derivation"
    );
    // ... and the formula sorts, so the reachable set's discovery order is not an input.
    let shuffled = vec![cards[2].clone(), cards[0].clone(), cards[1].clone()];
    assert_eq!(
        activity_key_rule(&shuffled).expect("still lawful"),
        activity_key,
        "the digests are *\"sorted\"*: a subgraph walk that visited the cards in another order is \
         the same subgraph"
    );

    // ---- INSTRUMENT 2: the lint ---------------------------------------------------------------
    // The premise: rk-store accepts the one-element slice happily, which is why a lint is owed.
    let store_says = rk_store::memo::activity_key(std::slice::from_ref(&cards[0]));
    assert_eq!(
        store_says.as_str().len(),
        64,
        "premise: `rk_store::memo::activity_key` returns a perfectly good digest for ONE card — it \
         has no opinion, and AC-016's lint is the opinion"
    );
    assert_eq!(
        activity_key_rule(std::slice::from_ref(&cards[0])),
        Err(MemoLintError::SingleCardReference { got: 1 }),
        "*\"a lint that rejects an activity-level key computed from a single card reference\"*"
    );
    assert_eq!(
        activity_key_rule(&[]),
        Err(MemoLintError::SingleCardReference { got: 0 }),
        "and zero, which the criterion does not name — see the variant's own doc for the reading \
         and the record that would settle it"
    );
    assert_eq!(
        MemoLintError::SingleCardReference { got: 1 }.clause(),
        "single_card_reference",
        "the tag is stable and is not the message text"
    );

    // ---- THE CONTROL: the lint is not a refuse-everything rule --------------------------------
    assert!(
        activity_key_rule(&cards[..2]).is_ok(),
        "two reachable cards is lawful. Without this the whole instrument is satisfied by a rule \
         that refuses every activity key, and the stage would see no key at all"
    );
}

// ---- AC-017 -------------------------------------------------------------------------------------

/// **AC-017 — "loops cannot learn".** *"GIVEN a step inside a `repeat` / `on_false` cycle whose
/// resolved inputs are byte-identical across two iterations WHEN iteration 2 executes THEN it does
/// not hit iteration 1's memo entry, because the loop iteration counter is part of the step key."*
///
/// *Verified by:* *"a unit test over a two-iteration fixture loop asserting **two distinct keys and
/// two executions**"* — two assertions, and the second is the one that needs a memo pass rather
/// than two `digest()` calls.
///
/// **The control is what carries the test.** Two distinct keys and two executions are equally true
/// of a memo that never hits anything, so the same iteration run twice must hit and execute once.
#[test]
fn memo_loop_iteration() {
    let mut i1 = baseline();
    i1.step_id = "settle".to_owned();
    i1.loop_iteration = 0;
    let mut i2 = i1.clone();
    i2.loop_iteration = 1;

    // The GIVEN: byte-identical resolved inputs across the two iterations.
    assert_eq!(
        i1.resolved_inputs, i2.resolved_inputs,
        "the GIVEN is *\"resolved inputs byte-identical across two iterations\"*"
    );
    let c1 = i1.components().expect("iteration 1 assembles");
    let c2 = i2.components().expect("iteration 2 assembles");
    assert_eq!(
        (
            &c1.activity_id_at_version,
            &c1.rigor,
            &c1.step_id,
            &c1.input_digests,
            &c1.card_or_tool,
            &c1.kb_atom_digests,
            &c1.schema_version
        ),
        (
            &c2.activity_id_at_version,
            &c2.rigor,
            &c2.step_id,
            &c2.input_digests,
            &c2.card_or_tool,
            &c2.kb_atom_digests,
            &c2.schema_version
        ),
        "premise: seven of the eight components are identical, so the assertion below is about \
         component 7 and about nothing else"
    );
    assert_eq!((c1.loop_iteration, c2.loop_iteration), (0, 1));

    // ---- two distinct keys --------------------------------------------------------------------
    assert_ne!(
        key_of(&i1),
        key_of(&i2),
        "the loop iteration counter is part of the step key: iteration 2 must not hit iteration \
         1's entry. This is the direct regression for the rejected iteration-blind key"
    );

    // ---- two executions -----------------------------------------------------------------------
    let (ledger, decisions, report) =
        fold(MemoLedger::empty(), &[i1.clone(), i2]).expect("the two-iteration pass runs");
    assert_eq!(
        (
            report.consulted,
            report.hits,
            report.misses,
            report.distinct_keys
        ),
        (2, 0, 2, 2),
        "two activations, no hit, two distinct keys"
    );
    assert_eq!(ledger.len(), 2, "both iterations entered the memo");
    let journal = recorded_residue(&decisions);
    assert_eq!(
        kinds(&journal),
        vec![
            EventKind::MemoMiss,
            EventKind::Dispatch,
            EventKind::MemoMiss,
            EventKind::Dispatch
        ],
        "*\"two executions\"* is two `dispatch` rows in the recorded residue, not two misses in a \
         decision list"
    );
    let t = tally(&journal);
    assert_eq!((t.activations, t.memo_hits, t.executed_steps), (2, 0, 2));

    // ---- THE CONTROL: the same iteration twice hits, and executes ONCE ------------------------
    let (_, same, same_report) =
        fold(MemoLedger::empty(), &[i1.clone(), i1]).expect("the same-iteration pass runs");
    assert_eq!(
        (
            same_report.consulted,
            same_report.hits,
            same_report.misses,
            same_report.distinct_keys
        ),
        (2, 1, 1, 1),
        "THE DISCRIMINATOR: re-running the SAME iteration must HIT. Without this, 'two distinct \
         keys and two executions' is satisfied by a memo that never hits anything at all, and the \
         whole criterion is passed by deleting the lookup"
    );
    assert!(same[1].is_hit());
    let same_journal = recorded_residue(&same);
    assert_eq!(
        kinds(&same_journal),
        vec![EventKind::MemoMiss, EventKind::Dispatch, EventKind::MemoHit],
        "one execution, and the skip is not silent"
    );
    let ts = tally(&same_journal);
    assert_eq!((ts.activations, ts.memo_hits, ts.executed_steps), (2, 1, 1));
    assert!(ts.identity_holds());
}

// ---- AC-015 -------------------------------------------------------------------------------------

/// **AC-015.** *"GIVEN an activity whose declared inclusion rules injected KB atom `A` and whose
/// result is memoised WHEN `A` is edited or superseded THEN the next execution of that activity
/// misses the memo and re-runs."*
///
/// *Verified by:* *"an integration test over the store fake that edits an injected atom and asserts
/// a **`memo_miss` event followed by a re-execution**."* Two observable consequences **in order**,
/// and the order is asserted by event index rather than by reading the test top to bottom.
///
/// 🛑 The register's instrument names *"the store fake"* and a real re-execution. Neither exists at
/// 7b: `rk-ports` is declared and uncalled, and the dispatcher is Slice 4h's. The re-execution is
/// measured against [`recorded_residue`]'s projection — the events 4h **will** append — and that
/// substitution is reported rather than absorbed. The parenthetical in the criterion is also
/// honoured by omission: *"a `doctor` rule … verifies nothing in this THEN"*, and none is written.
#[test]
fn ac015_kb_atom_edit_misses_the_memo() {
    let atom_a = d("kb/decisions/retry-budget@v1");
    let atom_a_edited = d("kb/decisions/retry-budget@v2");
    let atom_b = d("kb/concepts/idempotency@v1");

    let mut before = baseline();
    before.kb_atoms = vec![atom_a.clone(), atom_b.clone()];
    let mut after = before.clone();
    after.kb_atoms = vec![atom_a_edited, atom_b.clone()];

    // The premise: the edit moved component 6 and moved NOTHING ELSE. Without it the miss below is
    // attributable to any of the eight.
    let cb = before.components().expect("assembles");
    let ca = after.components().expect("assembles");
    assert_ne!(cb.kb_atom_digests, ca.kb_atom_digests);
    assert_eq!(
        (
            &cb.activity_id_at_version,
            &cb.rigor,
            &cb.step_id,
            &cb.input_digests,
            &cb.card_or_tool,
            cb.loop_iteration,
            &cb.schema_version
        ),
        (
            &ca.activity_id_at_version,
            &ca.rigor,
            &ca.step_id,
            &ca.input_digests,
            &ca.card_or_tool,
            ca.loop_iteration,
            &ca.schema_version
        ),
        "premise: the edit is component 6's and component 6's alone"
    );

    // Run, re-run unchanged (the control), then re-run with the atom edited.
    let (_, decisions, report) = fold(
        MemoLedger::empty(),
        &[before.clone(), before.clone(), after.clone()],
    )
    .expect("the three-activation pass runs");
    assert_eq!(
        (
            report.consulted,
            report.hits,
            report.misses,
            report.distinct_keys
        ),
        (3, 1, 2, 2)
    );
    assert!(!decisions[0].is_hit(), "the first activation cannot hit");
    assert!(
        decisions[1].is_hit(),
        "THE CONTROL: the SAME activation re-run must HIT. Without it, 'the edited atom missed' is \
         true of a memo that misses everything, and AC-015 is passed by deleting the lookup"
    );
    assert!(
        !decisions[2].is_hit(),
        "AC-015's THEN: the edited atom misses"
    );

    // ---- the order: a `memo_miss` EVENT, and THEN a re-execution ------------------------------
    let journal = recorded_residue(&decisions);
    assert_eq!(
        kinds(&journal),
        vec![
            EventKind::MemoMiss,
            EventKind::Dispatch,
            EventKind::MemoHit,
            EventKind::MemoMiss,
            EventKind::Dispatch,
        ]
    );
    let edit_miss = journal
        .iter()
        .enumerate()
        .filter(|(_, e)| e.kind == EventKind::MemoMiss)
        .map(|(i, _)| i)
        .nth(1)
        .expect("the edit's memo_miss is in the journal");
    let second_dispatch = journal
        .iter()
        .enumerate()
        .filter(|(_, e)| e.kind == EventKind::Dispatch)
        .map(|(i, _)| i)
        .nth(1)
        .expect("the re-execution's dispatch is in the journal");
    assert!(
        edit_miss < second_dispatch,
        "*\"a `memo_miss` event FOLLOWED BY a re-execution\"* — the miss is at index {edit_miss} \
         and the re-execution at {second_dispatch}; a runtime that dispatched first and recorded \
         the miss afterwards would leave a journal a replay cannot read in order"
    );
    assert_eq!(
        journal[edit_miss]
            .payload
            .get("key")
            .and_then(|v| v.as_str()),
        Some(key_of(&after).as_str()),
        "the miss names the NEW key, not the one it failed to find"
    );

    // ---- the dispatch count went 1 -> 2 -------------------------------------------------------
    let t = tally(&journal);
    assert_eq!(
        (
            t.activations,
            t.memo_hits,
            t.executed_steps,
            t.dispatch_events
        ),
        (3, 1, 2, 2),
        "two executions where an un-edited run would have had one"
    );
    assert!(t.identity_holds());

    // ---- THE SECOND DISCRIMINATOR: the key is on the RESOLVED ATOM SET, not the inclusion rule -
    // The declared inclusion rule is the same in all three contexts; only the resolved digests
    // differ. A memo keyed on the rule would hit here, and a memo keyed on the resolution ORDER
    // would miss on a re-resolution that changed nothing.
    let mut reordered = before.clone();
    reordered.kb_atoms = vec![atom_b.clone(), atom_a];
    assert_eq!(
        key_of(&reordered),
        key_of(&before),
        "the same resolved atom SET in another order is the same key: a re-resolution that changed \
         no atom must not re-run the activity"
    );
    // ... and a SUPERSEDED atom — dropped from the resolved set rather than edited — also misses.
    let mut superseded = before.clone();
    superseded.kb_atoms = vec![atom_b];
    assert_ne!(
        key_of(&superseded),
        key_of(&before),
        "*\"edited OR SUPERSEDED\"*: an atom that no longer resolves is as much a change as one \
         whose contents moved"
    );
    let decision = consult(&MemoLedger::empty(), &superseded).expect("consults");
    assert!(!decision.is_hit());
    assert_eq!(decision.as_str(), "memo_miss");
}

// ---- AC-018 -------------------------------------------------------------------------------------

/// **AC-018.** *"GIVEN a `tool` step whose verb's registry entry carries `live = true` WHEN the step
/// is memoised, or declares `memo = true` THEN pack validation fails with a lint error at the step's
/// span — liveness is the verb registry's flag and nothing else, and a step-level `memo = false`
/// declaration is itself a pack validation error (there is no per-step memo field)."*
///
/// *Verified by:* *"a lint rule with a rule-table row and **two must-fail fixture activities** (a
/// live verb memoised; a step declaring `memo = false`); **a positive fixture where a non-live verb
/// memoises**."*
///
/// 🛑 **The rule is RED against the shipped catalog and that is the design.** Nineteen shipped steps
/// across fourteen activities declare the key ADR-0097 decision 3 withdrew. §7b.12:970-977 forbids
/// editing `spikes/p9/` to make a green, so the count is asserted instead: the debt is pinned by a
/// number, and the day Step 7e repairs it this assertion goes red and says which number moved.
#[test]
fn ac018_live_verb_may_not_memoise() {
    let registry = verbs();

    // ---- premises about the registry ----------------------------------------------------------
    assert_eq!(registry.len(), 4, "four verbs, two live and two not");
    assert_eq!(registry.live("deploy"), Some(true));
    assert_eq!(registry.live("metrics"), Some(true));
    assert_eq!(registry.live("test"), Some(false));
    assert_eq!(registry.live("lint"), Some(false));
    assert_eq!(
        registry.live("nonesuch"),
        None,
        "an unknown verb is NOT a non-live one: `Some(false)` would make a typo in a `run` key \
         silently memoisable"
    );
    assert_eq!(verb_of("toolchain:deploy"), Some("deploy"));
    assert_eq!(verb_of("toolchain:logs?"), Some("logs"));
    assert_eq!(verb_of("${toolchain.deploy_timeout}"), None);

    // ---- the load refusals: liveness is never defaulted ---------------------------------------
    let e = VerbRegistry::from_toml(FIXTURE_VERBS_NO_LIVE)
        .expect_err("an entry without `live` is refused at load");
    assert_eq!(e.as_str(), "shape");
    assert!(
        matches!(&e, VerbRegistryError::Shape { verb, .. } if verb == "deploy"),
        "the refusal names the under-declared verb, not the file: {e}"
    );
    assert_eq!(
        VerbRegistry::from_toml("[toolchain.feeds]\nvalues = [\"logs\"]\n")
            .expect_err("a registry with no [verb] table is refused")
            .as_str(),
        "no_verb_table",
        "an empty verb registry is an error, not an empty success — a rule that finds no live verb \
         reports green over a registry it could not read"
    );

    // ---- must-fail fixture 1: a live verb, memoised -------------------------------------------
    let cat1 = catalog("live-verb-memoised.toml", FIXTURE_LIVE_MEMOISED);
    let act1 = &cat1.activities()[0];
    let ship = act1
        .steps
        .iter()
        .find(|s| s.id == "ship")
        .expect("the fixture declares the `ship` step");
    assert_eq!(ship.kind, StepKind::Tool, "premise: it is a `tool` step");
    assert_eq!(
        ship.memo_declared,
        Some(true),
        "premise: the fixture really declares `memo = true`"
    );
    assert_eq!(
        ship.run.as_deref().and_then(verb_of),
        Some("deploy"),
        "premise: on a verb the registry calls live"
    );
    let v1 = live_verb_memo_rule(&cat1, &registry).expect_err("fixture 1 must fail");
    assert_eq!(
        v1,
        vec![
            MemoViolation::LiveVerbMemoised {
                activity: "live-verb-memoised".to_owned(),
                step: "ship".to_owned(),
                verb: "deploy".to_owned(),
            },
            MemoViolation::PerStepMemoDeclaration {
                activity: "live-verb-memoised".to_owned(),
                step: "ship".to_owned(),
            },
        ],
        "both clauses fire on this step and both are reported — see `live_verb_memo_rule`'s doc \
         for why the second is not suppressed"
    );
    assert_eq!(v1[0].clause(), "live_verb_memoised");
    assert!(
        v1.iter().all(|x| !matches!(
            x,
            MemoViolation::LiveVerbMemoised { step, .. } | MemoViolation::PerStepMemoDeclaration { step, .. }
            if step == "check-lint"
        )),
        "the lawful sibling step (`lint`, live = false, no `memo` key) produced nothing: the rule's \
         output is a SUBSET of the file's steps and not all of them"
    );

    // ---- must-fail fixture 2: the withdrawn per-step field, ISOLATED --------------------------
    let cat2 = catalog("step-declares-memo-false.toml", FIXTURE_MEMO_FALSE);
    let suite = &cat2.activities()[0].steps[0];
    assert_eq!(suite.memo_declared, Some(false), "premise: `memo = false`");
    assert_eq!(
        registry.live(suite.run.as_deref().and_then(verb_of).expect("a verb")),
        Some(false),
        "premise: on a NON-live verb, so this fixture isolates ADR-0097 decision 3's clause from \
         the liveness clause"
    );
    let v2 = live_verb_memo_rule(&cat2, &registry).expect_err("fixture 2 must fail");
    assert_eq!(
        v2,
        vec![MemoViolation::PerStepMemoDeclaration {
            activity: "step-declares-memo-false".to_owned(),
            step: "suite".to_owned(),
        }],
        "exactly one violation: a rule that only ever fired on live verbs would report none here"
    );
    assert_eq!(v2[0].clause(), "per_step_memo_declaration");

    // ---- THE MUST-LOAD POSITIVE: the discriminator --------------------------------------------
    let cat3 = catalog("non-live-verb-memoises.toml", FIXTURE_NON_LIVE);
    assert_eq!(
        cat3.activities()[0].steps.len(),
        3,
        "premise: the positive fixture is not empty — a rule over no steps passes"
    );
    assert!(
        cat3.activities()[0]
            .steps
            .iter()
            .any(|s| s.kind == StepKind::Tool),
        "premise: and it really carries a tool step"
    );
    assert_eq!(
        live_verb_memo_rule(&cat3, &registry),
        Ok(()),
        "*\"a positive fixture where a non-live verb memoises\"* must LOAD and pass. Without this \
         both must-fail assertions above are satisfied by a rule that refuses every catalog"
    );

    // ---- 🛑 THE SHIPPED CATALOG: RED BY DESIGN, and the repairs are Step 7e's ------------------
    let shipped = Catalog::from_dir(&workspace_root().join(SHIPPED_CATALOG))
        .expect("the shipped catalog loads through the new per-step `memo` parse path");
    assert_eq!(
        shipped.len(),
        102,
        "premise: this is the shipped catalog and not a subset of it"
    );
    let shipped_violations =
        live_verb_memo_rule(&shipped, &registry).expect_err("AC-018 is red against the artefacts");
    assert_eq!(
        shipped_violations.len(),
        19,
        "nineteen shipped steps declare the withdrawn `memo` key [measured 2026-09-14: \
         `grep -rn 'memo' spikes/p9/catalog/ | wc -l`]. §7b.12:970-977 forbids editing the shipped \
         artefacts to make a green, so the debt is pinned by this number. If Step 7e has repaired \
         them, this assertion is the notification"
    );
    assert!(
        shipped_violations
            .iter()
            .all(|v| v.clause() == "per_step_memo_declaration"),
        "every one of the nineteen is `memo = false` — not one shipped step declares `memo = true`, \
         so the liveness clause fires nowhere in the shipped catalog and the fixture above is the \
         only thing that reaches it"
    );
    let activities: BTreeSet<&str> = shipped_violations
        .iter()
        .map(MemoViolation::activity)
        .collect();
    assert_eq!(
        activities.len(),
        14,
        "across fourteen activities [measured 2026-09-14: `grep -rln 'memo' spikes/p9/catalog/`]"
    );
    println!(
        "ac018_live_verb_may_not_memoise: shipped_activities={} violations={} activities_affected={}",
        shipped.len(),
        shipped_violations.len(),
        activities.len()
    );
}

// ---- AC-019 -------------------------------------------------------------------------------------

/// **AC-019, first instrument.** *"THEN a `memo_hit` event carrying the key and its **eight**
/// components (exactly AC-014's list, ADR-0097) is appended; a skip is never silent."*
///
/// 🛑 *Verified by* names *"an event-schema test asserting the field set"*, and **there is no
/// schema**. `crates/rk-types/src/event.rs`:46-59 records the per-kind payload as *"provisionally
/// and visibly"* untyped and the typed-payload record as **OWED**, its earliest real consumer the
/// SQLite adapter at 7c. So what is asserted below is the **JSON key set** and the value of every
/// member, and the *"event-schema test"* half is reported **ARMED** (residual 10j). Inventing a
/// payload schema here would pre-empt the record 7c owes.
#[test]
fn ac019_memo_hit_is_never_silent() {
    let ctx = baseline();
    let comps = ctx.components().expect("the baseline assembles");
    let key = ctx.key().expect("and keys");
    let payload = memo_hit_payload(&key, &comps);
    let obj = payload
        .as_object()
        .expect("a memo_hit payload is a JSON object");

    // ---- exactly the eight components plus the key, and no ninth ------------------------------
    assert_eq!(
        obj.len(),
        StepKeyComponents::COMPONENT_COUNT + 1,
        "*\"the key and its eight components\"* is nine members. A payload carrying the key alone \
         makes the skip silent about WHY; a tenth member is a component AC-014 does not have"
    );
    let mut present: Vec<&str> = obj.keys().map(String::as_str).collect();
    present.sort_unstable();
    assert_eq!(
        present,
        MEMO_HIT_PAYLOAD_KEYS.to_vec(),
        "the member names are `StepKeyComponents`' own field names plus `key`"
    );

    // ---- every value, against the component it claims to carry --------------------------------
    assert_eq!(obj["key"], json!(key.as_str()));
    assert_eq!(
        obj["activity_id_at_version"],
        json!(comps.activity_id_at_version)
    );
    assert_eq!(obj["rigor"], json!(comps.rigor));
    assert_eq!(obj["step_id"], json!(comps.step_id));
    assert_eq!(
        obj["input_digests"],
        json!(
            comps
                .input_digests
                .iter()
                .map(|x| x.as_str().to_owned())
                .collect::<Vec<_>>()
        )
    );
    assert_eq!(obj["card_or_tool"], json!(comps.card_or_tool));
    // Component 6 IS *"the digests of the resolved KB atoms, SORTED"*, so the payload names the
    // sorted set and not the caller's resolution order — otherwise two `memo_hit` events on one key
    // could carry two different payloads. Asserted against an explicitly re-sorted copy rather than
    // against the component as supplied, because the baseline's two atoms happen to arrive sorted
    // and comparing with them would pass whether the payload sorted or not.
    let mut expected_atoms: Vec<String> = comps
        .kb_atom_digests
        .iter()
        .map(|x| x.as_str().to_owned())
        .collect();
    expected_atoms.sort();
    assert_eq!(obj["kb_atom_digests"], json!(expected_atoms));
    let mut unsorted = baseline();
    unsorted.kb_atoms.reverse();
    let unsorted_comps = unsorted.components().expect("assembles");
    assert_ne!(
        unsorted_comps.kb_atom_digests, comps.kb_atom_digests,
        "premise: the reversed context really does carry the atoms in the other order"
    );
    assert_eq!(
        memo_hit_payload(&unsorted.key().expect("keys"), &unsorted_comps),
        payload,
        "the same resolved atom set in the other order is the same key AND the same payload"
    );
    assert_eq!(obj["loop_iteration"], json!(comps.loop_iteration));
    assert_eq!(obj["schema_version"], json!(comps.schema_version));

    // ---- THE CONTROL: the payload is not a constant -------------------------------------------
    let mut other = baseline();
    other.step_id = "settle".to_owned();
    other.loop_iteration = 2;
    other.rigor = Rigor::Mandatory;
    let other_comps = other.components().expect("assembles");
    let other_payload = memo_hit_payload(&other.key().expect("keys"), &other_comps);
    assert_ne!(
        other_payload, payload,
        "a `memo_hit` payload that did not move with its components would carry nine members and \
         describe a different step"
    );
    assert_ne!(other_payload["key"], payload["key"]);
    assert_ne!(other_payload["step_id"], payload["step_id"]);
    assert_ne!(other_payload["loop_iteration"], payload["loop_iteration"]);
    assert_ne!(other_payload["rigor"], payload["rigor"]);

    // ---- the event itself ---------------------------------------------------------------------
    let hit = MemoDecision::Hit {
        key: key.clone(),
        components: comps.clone(),
    };
    let journal = recorded_residue(std::slice::from_ref(&hit));
    assert_eq!(
        journal.len(),
        1,
        "a hit leaves ONE event and no dispatch: that is what a short circuit is"
    );
    assert_eq!(journal[0].kind, EventKind::MemoHit);
    assert_eq!(journal[0].payload, payload);

    // A miss carries the same nine members, so a replay reading the two kinds reads one shape.
    let miss = MemoDecision::Miss {
        key,
        components: comps,
    };
    let miss_journal = recorded_residue(std::slice::from_ref(&miss));
    assert_eq!(
        kinds(&miss_journal),
        vec![EventKind::MemoMiss, EventKind::Dispatch]
    );
    assert_eq!(
        miss_journal[0].payload, payload,
        "a miss is not silent either, and it names the key it failed to find"
    );

    // ---- THE ARMED PIN ------------------------------------------------------------------------
    assert_eq!(
        EventKind::ALL.len(),
        38,
        // DECIDED HERE 2026-09-17 (ADR-0123, 29 -> 31; ADR-0124, 31 -> 36; ADR-0125, 36 -> 37, the same answer each
        // time): `gate_answer_spent` and `gate_resumed` are gate rows, ADR-0124's five and `stage_exit_refused` are
        // the review and verify gates' rows, not memo rows; the memo's typed-payload record is STILL OWED, unchanged.
        // DECIDED AGAIN 2026-09-17 (Step 7c Slice 1; ADR-0126, 37 -> 38): `dispatch_completed` is the adapter's
        // usage-bearing row, not a memo row — the memo fold routes it to its no-state-change arm and NOT to the
        // `Dispatch` count (decision 3: two kinds, two producers, never counted as one; asserted by
        // `tests/it/dispatch_completed.rs::adr0126_dispatch_completed_is_not_counted_as_a_dispatch`) — and ADR-0126 decision 6
        // says in terms that the typed per-kind payload record is NOT decided by it: the memo's payload record
        // is STILL OWED, unchanged.
        "`EventKind` is closed at 38 (ADR-0116; ADR-0123; ADR-0124; ADR-0125; ADR-0126). If another is minted, whoever mints it must \
         decide whether the memo's payload record is still OWED — this line goes red rather than \
         the gap quietly widening"
    );
}

/// **AC-019, second instrument.** *"a replay assertion that executed steps plus `memo_hit` events
/// equals the number of **step activations** in the recorded run — not the graph's static step
/// count, which `repeat`/`on_false` (AC-017) and `for_each` (AC-022) make unequal to it."*
///
/// The register spells out the trap, so the premise `assert_ne!(activations, static_step_count)` is
/// the first assertion and the fixture is built to satisfy it for **both** named reasons. Four
/// adverse journals follow the green one: without them the identity is true by construction, because
/// [`recorded_residue`] emits one `dispatch` per miss.
///
/// 🛑 The **recorded run** this identity is stated over is a projection, not a dispatched run: there
/// is no dispatcher until Slice 4h. The identity over a real journal is **ARMED**.
#[test]
fn ac019_executed_plus_hits_equals_activations() {
    let cat = catalog("memo-activations.toml", FIXTURE_ACTIVATIONS);
    let act = &cat.activities()[0];

    // ---- premises about the fixture, all parsed rather than asserted from a comment ------------
    assert_eq!(
        act.id_at_version().as_deref(),
        Some("memo-activations@2"),
        "component 1 comes from the catalog and not from a literal"
    );
    assert_eq!(act.steps.len(), 4, "four static steps");
    assert!(
        act.steps.iter().any(|s| s.for_each.is_some()),
        "the fixture declares a `for_each` step (AC-022's half of the inequality)"
    );
    assert_eq!(
        act.steps
            .iter()
            .find(|s| s.id == "settle")
            .and_then(|s| s.repeat_max),
        Some(3),
        "and a `repeat` loop (AC-017's half)"
    );

    // The six activations the recorded run produced: one for `draft`, three for the `for_each`
    // step's three items, two for the two iterations `settle` took of its three.
    let id_at_version = act.id_at_version().expect("the fixture declares a version");
    let mut ctxs: Vec<StepContext> = Vec::new();
    {
        let mut push = |step_id: &str, input: serde_json::Value, iteration: u32| {
            ctxs.push(StepContext {
                activity_id_at_version: id_at_version.clone(),
                rigor: Rigor::Recommended,
                step_id: step_id.to_owned(),
                resolved_inputs: vec![input],
                card_or_tool: CardOrTool::Card(d("card/implementer@v4")),
                kb_atoms: vec![d("kb/atom-a@v1")],
                loop_iteration: iteration,
                output_schema: SchemaId::new("RecordedResult/v1"),
            });
        };
        push("draft", json!({"spec": "INC-1"}), 0);
        push("per-finding", json!({"finding": 0}), 0);
        push("per-finding", json!({"finding": 1}), 0);
        push("per-finding", json!({"finding": 2}), 0);
        push("settle", json!({"draft": "d"}), 0);
        push("settle", json!({"draft": "d"}), 1);
    }

    assert_eq!(ctxs.len(), 6);
    assert_ne!(
        ctxs.len(),
        act.steps.len(),
        "THE PREMISE the register writes out: the activation count is NOT the graph's static step \
         count. A fixture with neither a `for_each` nor a loop makes the two equal and the whole \
         identity vacuous"
    );

    // A memo already holding `draft`'s entry, so the recorded run has a hit as well as misses.
    let (warm, _, _) = fold(MemoLedger::empty(), &ctxs[..1]).expect("the warming pass runs");
    assert_eq!(warm.len(), 1);
    let (_, decisions, report) = fold(warm, &ctxs).expect("the recorded run's pass runs");
    assert_eq!(
        (report.consulted, report.hits, report.misses),
        (6, 1, 5),
        "six activations consulted the memo, one of them hit"
    );

    let journal = recorded_residue(&decisions);
    let t = tally(&journal);
    assert_eq!(
        t.events_folded,
        journal.len(),
        "premise: the fold looked at every event — an empty journal satisfies the identity"
    );
    assert!(t.events_folded > 0);
    assert_eq!(t.activations, 6);
    assert_eq!(t.memo_hits, 1);
    assert_eq!(t.memo_misses, 5);
    assert_eq!(t.executed_steps, 5);
    assert_eq!(t.unattributed_dispatches, 0);
    assert_eq!(
        t.executed_steps + t.memo_hits,
        t.activations,
        "AC-019's identity, written out"
    );
    assert!(t.identity_holds());
    assert_ne!(
        t.activations,
        act.steps.len(),
        "and the identity is stated over ACTIVATIONS: against the static step count of {} it is \
         false by two",
        act.steps.len()
    );

    // ---- tier-2 noise must not move any of it -------------------------------------------------
    let mut noisy = journal.clone();
    noisy.insert(
        0,
        Event::new(EventKind::StageTransition, json!({"to": "implement"})),
    );
    noisy.push(Event::new(EventKind::DagAssembled, json!({"nodes": 4})));
    let tn = tally(&noisy);
    assert_eq!(tn.events_folded, t.events_folded + 2);
    assert_eq!(
        (tn.activations, tn.memo_hits, tn.executed_steps),
        (t.activations, t.memo_hits, t.executed_steps),
        "the other 26 kinds are counted as folded and change no quantity in the identity"
    );

    // ---- ADVERSE 1: a RETRY. Two dispatch rows, one execution. --------------------------------
    // This is what stops `executed_steps` being a rename of `dispatch_events` — a rename passes
    // every assertion above and fails here.
    let mut retried = journal.clone();
    let first_dispatch = journal
        .iter()
        .position(|e| e.kind == EventKind::Dispatch)
        .expect("the journal dispatches");
    retried.insert(first_dispatch + 1, journal[first_dispatch].clone());
    let tr = tally(&retried);
    assert_eq!(tr.dispatch_events, t.dispatch_events + 1);
    assert_eq!(
        tr.executed_steps, t.executed_steps,
        "a retried activation dispatches twice and EXECUTES ONCE: AC-019 counts executed steps, \
         and a second attempt at the same step is not a second step"
    );
    assert!(tr.identity_holds());
    assert_ne!(
        tr.dispatch_events, tr.executed_steps,
        "and the two readings really are different numbers on this journal, so the equality above \
         is a claim and not a tautology"
    );

    // ---- ADVERSE 2: a miss that never dispatched. The identity must BREAK. ---------------------
    let mut dropped = journal.clone();
    dropped.remove(first_dispatch);
    let td = tally(&dropped);
    assert_eq!(td.activations, t.activations);
    assert_eq!(td.executed_steps, t.executed_steps - 1);
    assert!(
        !td.identity_holds(),
        "a `memo_miss` with no execution behind it is a step the run decided to take and did not: \
         the identity is what catches it"
    );

    // ---- ADVERSE 3: a hit that dispatched. The skip that skipped nothing. ---------------------
    let hit_then_dispatch = vec![
        journal
            .iter()
            .find(|e| e.kind == EventKind::MemoHit)
            .expect("the journal hits")
            .clone(),
        journal[first_dispatch].clone(),
    ];
    let th = tally(&hit_then_dispatch);
    assert_eq!(
        th.unattributed_dispatches, 1,
        "a `dispatch` after a `memo_hit` is an execution the memo claimed to have short-circuited"
    );
    assert!(!th.identity_holds());
    // ... and a dispatch before any decision at all is the same defect from the other side.
    let tb = tally(&[journal[first_dispatch].clone()]);
    assert_eq!(tb.unattributed_dispatches, 1);
    assert_eq!(tb.activations, 0);
    assert!(!tb.identity_holds());

    // ---- ADVERSE 4: a malformed decision payload refuses rather than counting ------------------
    let err = tally_activations(&[Event::new(EventKind::MemoHit, json!({}))])
        .expect_err("a memo_hit with no key is not a countable activation");
    assert_eq!(err.as_str(), "malformed_payload");
    assert!(matches!(err, MemoError::MalformedPayload { kind, .. } if kind == "memo_hit"));

    println!(
        "ac019 identity: activations={} executed={} hits={} dispatch_rows={} static_steps={}",
        t.activations,
        t.executed_steps,
        t.memo_hits,
        t.dispatch_events,
        act.steps.len()
    );
}
