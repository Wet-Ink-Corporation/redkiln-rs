## F41 — S-PORTS RESULT: ADR-0099's port shape compiles on both the spike toolchain and the pinned one — five sync traits as `Arc<dyn Port + Send + Sync>`, a blocking `dispatch` returning `DispatchOutcome`, a root-owned `CancelToken`, one scripted fake, and two `compile_fail` doctests that fail for the right reason — 5/5 unit tests and 2/2 doctests on rustc 1.97.1 and 1.98.1, clippy clean

**Status: COMPLETE.** Registered 2026-09-11 per ADR-0012 against audit §8 M-01 ("nothing was compiled"); consumer:
runbook Step 7a and ADR-0099. Run the same day. Zero model cost.

**Established by:** one throwaway crate, `spikes/s-ports/` (never `crates/`; 270 lines of Rust; one dependency,
`serde_json` 1.0.151), `cargo test` on rustc **1.97.1** (8bab26f4f 2026-07-14) and on rustc **1.98.1** (48a229cea
2026-09-01), `cargo +1.98.1 clippy --all-targets -- -D clippy::all`. 2026-09-11, ~15 s per toolchain, $0.

### What was checked

| ADR-0099 clause | instrument | result |
| --- | --- | --- |
| 1 — `Harness::dispatch(&self, &ActivitySpec, &mut dyn EventStore) -> DispatchOutcome { record, result }` blocks and returns the AC-360 record beside `Result<Typed, StepError>` | `dispatch_blocks_and_returns_record_and_result` | pass |
| 2 — `CancelToken` cloned into the adapter at construction, triggered by the root, never by the kernel; a cancelled dispatch yields `Err(StepError::Cancelled { by })` and one `step_cancelled` event | `cancel_token_is_the_roots_and_yields_cancelled` | pass |
| 4 — the five traits carry `Send + Sync` supertraits and are object-safe; the composition root holds them as `Arc<dyn Port + Send + Sync>` and moves one across a thread | `five_ports_are_object_safe_and_send_sync`, `ports_struct_holds_arc_dyn_send_sync` | pass |
| ADR-0095 decision 2 — `StepError` is a closed eight-variant enum the fake maps to one event arm each; `spoke()` is total | `step_error_is_matched_exhaustively_with_one_event_arm_each` | pass |
| the `Send + Sync` rule is enforced by the compiler — a fake holding an `Rc` cannot become `Arc<dyn Harness>` | doctest `compile_fail` (line 11) | fails to compile, as required |
| AC-480 — the kernel's entry point takes `&dyn Harness`, so a port-bound generic `fn<H: Harness>` cannot be handed to it | doctest `compile_fail` (line 27) | fails to compile, as required |
| ADR-0099 clause 5 — `ChangeSummary` is an opaque newtype in the ports crate (`pub struct ChangeSummary(pub(crate) ())`) | compiles; no other site names it | pass |

Output, verbatim, on 1.98.1:

```
running 5 tests
test tests::cancel_token_is_the_roots_and_yields_cancelled ... ok
test tests::dispatch_blocks_and_returns_record_and_result ... ok
test tests::five_ports_are_object_safe_and_send_sync ... ok
test tests::step_error_is_matched_exhaustively_with_one_event_arm_each ... ok
test tests::ports_struct_holds_arc_dyn_send_sync ... ok
test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
   Doc-tests s_ports
running 2 tests
test src\lib.rs - (line 11) - compile fail ... ok
test src\lib.rs - (line 27) - compile fail ... ok
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.12s
```

Identical on 1.97.1 (`finished in 0.05s` / `0.46s`). `cargo +1.98.1 clippy --all-targets -- -D clippy::all`: clean.

### 1. What this settles [measured]

The realisability question the audit could not answer from the record (§8 M-01: "whether the Harness port can be
written as ADR-0076 states it is not-answerable from the corpus as written") is answered for the shape ADR-0099
states: the trait objects are dyn-compatible, `Send + Sync` supertraits are accepted by a fake that holds a
`Mutex<Vec<…>>`, and the two rules the design names a compile failure as the instrument for (AC-480; the port-bound
`Send + Sync`) do fail to compile in the direction required. The `StepError` census of ADR-0095 is matched
exhaustively by a `match` with one arm per variant, so adding a ninth variant is a compile error in the fake — the
property AC-229/AC-360 want.

### 2. What this does not settle

- **Nothing about a real child process.** The fake is a scripted responder; the Job Object, the process group, the
  timeout and the raw-retention pattern are unexercised. AC-224's fixture is the first measurement of containment.
- **Nothing about the kernel's replay or the memo.** The "kernel" here is one struct with one dispatch call site.
- **Nothing about the real `rk-types`.** `Typed`, `ActivitySpec`, `DispatchRecord` are sketched with the fields
  ADR-0099 names; the golden schema snapshot (ADR-0095) is Step 5.3's.
- **The instrument moved.** The spike programme ran on 1.97.1; the pin is 1.98.1 (DECISIONS-2026-09-11 #7). Both
  were run; the result is identical on both, which is why the move is recorded rather than re-measured.

### Limits

1. One crate, one author, one day, two toolchains; n = 1 shape. A different fake (one holding a runtime handle,
   per ADR-0076) is not tested for `Send + Sync`; that is the adapter's obligation and is unmeasured here.
2. The doctests assert *that* the shape fails to compile, not the error text; a future rustc could fail them for a
   different reason and still pass.
3. `cargo +1.98.1` was available on this box only because the toolchain was installed for this spike
   (`rustup toolchain install 1.98.1 --profile minimal -c rustfmt -c clippy`); the runbook's Step 0 records it.

### Correction (2026-09-11, later the same day)

ADR-0112 adds `Platform { status, retry_after }`, `Unsupported { capability }` and `Overage { reason }` to the `StepError` table —
three variants thirteen criteria already named and ADR-0095's table omitted (register §0.3 row C5). `spikes/s-ports/` was extended to
the **eleven**-variant enum and re-run on 1.98.1: `step_error_is_matched_exhaustively_with_one_event_arm_each` carries eleven cases;
5/5 unit tests and 2/2 doctests pass, unchanged. The row above that says "closed eight-variant enum" reads "eleven" after this note.
