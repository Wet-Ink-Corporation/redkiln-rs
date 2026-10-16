//! **What `resume`'s outcome means when the routed toolchain kept a refusal** (`s8i` §8i.8;
//! ADR-0199 decision 3, proposed).
//!
//! [`crate::review_input::route`] keeps an [`AutomatedVerdictRefused`] and refuses the step to the
//! kernel, so the stage's walk ends at that refusal. Only then does the human gate open. Any other
//! outcome `resume` returns while a refusal is kept (a success, a store or tier-1 failure, another
//! step's class) is not explained by the refusal: it is reported, and handed back to be mapped
//! exactly as it would be with no refusal kept. No outcome is dropped unexamined.

use rk_kernel::replay::ReplayError;

use super::told;
use crate::exit::Exit;
use crate::review_input::{AutomatedVerdictRefused, Refusals, refused_the_step};

/// What the stage does next.
#[derive(Debug)]
pub(super) enum Next<'r, T> {
    /// The walk ended at the kept refusal: the human gate opens from it.
    Gate(&'r AutomatedVerdictRefused),
    /// `resume`'s outcome, handed back whole for the stage's own mapping.
    Map(Result<T, ReplayError>),
    /// The report of an unexplained outcome could not be written.
    Exit(Exit),
}

/// Read `resumed` against the refusal `refusals` kept, reporting to `err` an outcome the refusal
/// does not explain.
///
/// The refusal is handed out only when the outcome is its own: a walk that did not commit, or
/// failed after a commit, with exactly the class [`refused_the_step`] names.
pub(super) fn next<'r, T>(
    item: &str,
    resumed: Result<T, ReplayError>,
    refusals: &'r Refusals,
    err: &mut dyn std::io::Write,
) -> Next<'r, T> {
    let Some(refused) = refusals.first() else {
        return Next::Map(resumed);
    };
    let outcome = match &resumed {
        Err(
            ReplayError::NotCommitted {
                class: Some(class), ..
            }
            | ReplayError::StepFailedAfterCommit { class, .. },
        ) if refused_the_step(class) => return Next::Gate(refused),
        Ok(_) => "success".to_owned(),
        Err(e) => format!("`{e}`"),
    };
    let reason = rk_kernel::workflow::gate::GateReason::AutomatedVerdictRefused;
    let line = format!(
        "redkiln: `{item}`'s change summary was refused to the kernel ({reason}), but the stage \
         returned {outcome} rather than that refusal; no gate opens over it, and the outcome is \
         mapped as it stands"
    );
    match told(err, &line, Exit::Success) {
        Exit::Success => Next::Map(resumed),
        unwritten => Next::Exit(unwritten),
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use rk_kernel::replay::ReplayError;
    use rk_ports::toolchain::{
        BinaryChangeSummary, ChangeSummary, Classification, Selection, StepBound, TestOutcome,
        Toolchain, VerbCall, VerbOutput,
    };
    use rk_types::error::{StepError, TransportKind};
    use rk_types::verb_output::BuildReport;

    use super::{Next, next};
    use crate::exit::Exit;
    use crate::review_input::{self, Refusals};

    const ITEM: &str = "story-1";
    const REASON: &str = rk_kernel::workflow::gate::GateReason::AutomatedVerdictRefused.as_str();

    /// A toolchain whose only answer is a content-free `Binary` summary.
    struct BinaryOnly;

    fn not_this_fake(verb: &str) -> StepError {
        StepError::Unsupported {
            capability: format!("{verb}: the kept-refusal fake answers change_summary only"),
        }
    }

    impl Toolchain for BinaryOnly {
        fn detect(&self) -> Result<bool, StepError> {
            Ok(true)
        }
        fn build(&self, _bound: StepBound) -> Result<BuildReport, StepError> {
            Err(not_this_fake("build"))
        }
        fn test(
            &self,
            _selection: &Selection,
            _bound: StepBound,
        ) -> Result<TestOutcome, StepError> {
            Err(not_this_fake("test"))
        }
        fn lint(&self, _bound: StepBound) -> Result<(), StepError> {
            Err(not_this_fake("lint"))
        }
        fn classify(&self, _path: &str) -> Result<Classification, StepError> {
            Err(not_this_fake("classify"))
        }
        fn change_summary(
            &self,
            _range: &str,
            _bound: StepBound,
        ) -> Result<ChangeSummary, StepError> {
            BinaryChangeSummary::new(
                Vec::new(),
                Vec::new(),
                "reports/tests.xml".to_owned(),
                "logs/build.log".to_owned(),
                "",
            )
            .map(ChangeSummary::Binary)
            .map_err(|e| not_this_fake(&format!("change_summary: {e:?}")))
        }
        fn run_verb(&self, call: &VerbCall<'_>) -> Result<VerbOutput, StepError> {
            Err(not_this_fake(call.verb))
        }
    }

    /// The refusal a routed toolchain keeps, and the step error it hands the kernel for it.
    fn kept() -> (Refusals, StepError) {
        let (routed, refusals) = review_input::route(Arc::new(BinaryOnly));
        let class = routed
            .change_summary(
                "4b825dc642cb6eb9a060e54bf8d69288fbee4904..9daeafb9864cf43055ae93beb0afd6c7d144bfa4",
                StepBound::secs(300).expect("a non-zero bound"),
            )
            .expect_err("PREMISE: a Binary summary is refused to the kernel");
        assert!(
            refusals.first().is_some(),
            "PREMISE: the routed toolchain kept the refusal"
        );
        (refusals, class)
    }

    fn not_committed(class: Option<StepError>) -> ReplayError {
        ReplayError::NotCommitted {
            step_id: "summarise".to_owned(),
            activity_id_at_version: "emit-report@1".to_owned(),
            class,
        }
    }

    /// A writer that refuses every write.
    struct Closed;

    impl std::io::Write for Closed {
        fn write(&mut self, _buf: &[u8]) -> std::io::Result<usize> {
            Err(std::io::Error::from(std::io::ErrorKind::BrokenPipe))
        }
        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }

    #[test]
    fn the_refusals_own_outcome_opens_the_gate_from_the_kept_refusal_and_reports_nothing() {
        let (refusals, class) = kept();
        let kept = refusals.first().expect("PREMISE: kept");
        for resumed in [
            not_committed(Some(class.clone())),
            ReplayError::StepFailedAfterCommit {
                step_id: "summarise".to_owned(),
                activity_id_at_version: "emit-report@1".to_owned(),
                class,
            },
        ] {
            let shown = format!("{resumed:?}");
            let mut err = Vec::new();
            let Next::Gate(refused) = next(ITEM, Err::<(), _>(resumed), &refusals, &mut err) else {
                panic!("the refusal's own outcome did not open the gate: {shown}");
            };
            assert!(
                std::ptr::eq(refused, kept),
                "the gate opens from the kept refusal: {shown}"
            );
            assert!(
                err.is_empty(),
                "the refusal's own outcome is not an unexplained one: {}",
                String::from_utf8_lossy(&err)
            );
        }
    }

    #[test]
    fn an_unrelated_failure_with_a_kept_refusal_is_reported_and_handed_back_not_gated() {
        let (refusals, _class) = kept();
        let cases = [
            ReplayError::StoreRefused {
                detail: "the store refused the append".to_owned(),
            },
            not_committed(Some(StepError::Transport {
                kind: TransportKind::Spawn,
                detail: "the harness did not start".to_owned(),
            })),
            not_committed(Some(StepError::Unsupported {
                capability: "another refusal altogether".to_owned(),
            })),
            not_committed(None),
            ReplayError::StepFailedAfterCommit {
                step_id: "summarise".to_owned(),
                activity_id_at_version: "emit-report@1".to_owned(),
                class: StepError::Unsupported {
                    capability: "another refusal altogether".to_owned(),
                },
            },
        ];
        for resumed in cases {
            let shown = format!("{resumed:?}");
            let named = resumed.to_string();
            let mut err = Vec::new();
            let Next::Map(Err(back)) = next(ITEM, Err::<(), _>(resumed), &refusals, &mut err)
            else {
                panic!("an unrelated failure was not handed back for its own mapping: {shown}");
            };
            assert_eq!(
                format!("{back:?}"),
                shown,
                "the outcome is handed back unchanged"
            );
            let report = String::from_utf8(err).expect("the report is UTF-8");
            assert!(
                report.contains(ITEM) && report.contains(REASON) && report.contains(&named),
                "the unrelated failure is reported, naming the item, the kept refusal and the \
                 outcome `{named}`: {report:?}"
            );
            assert_eq!(report.lines().count(), 1, "one line: {report:?}");
        }
    }

    #[test]
    fn a_success_with_a_kept_refusal_is_reported_and_handed_back_not_dropped() {
        let (refusals, _class) = kept();
        let mut err = Vec::new();
        let Next::Map(Ok(7_u8)) = next(ITEM, Ok::<u8, ReplayError>(7), &refusals, &mut err) else {
            panic!("a success with a kept refusal was not handed back for its own mapping");
        };
        let report = String::from_utf8(err).expect("the report is UTF-8");
        assert!(
            report.contains(ITEM) && report.contains(REASON) && report.contains("success"),
            "the success is reported, naming the item and the kept refusal: {report:?}"
        );
        assert_eq!(report.lines().count(), 1, "one line: {report:?}");
    }

    #[test]
    fn no_kept_refusal_hands_the_outcome_back_and_reports_nothing() {
        let refusals = Refusals::default();
        let mut err = Vec::new();
        let Next::Map(Err(ReplayError::StoreRefused { detail })) = next(
            ITEM,
            Err::<(), _>(ReplayError::StoreRefused {
                detail: "x".to_owned(),
            }),
            &refusals,
            &mut err,
        ) else {
            panic!("an outcome with no kept refusal was not handed back unchanged");
        };
        assert_eq!(detail, "x");
        assert!(
            err.is_empty(),
            "nothing was kept, so nothing is reported: {}",
            String::from_utf8_lossy(&err)
        );
    }

    #[test]
    fn an_unwritable_report_exits_config_error() {
        let (refusals, _class) = kept();
        let next = next(ITEM, Ok::<(), ReplayError>(()), &refusals, &mut Closed);
        assert!(
            matches!(next, Next::Exit(Exit::ConfigError)),
            "a report that cannot be written ends the run at 3: {next:?}"
        );
    }
}
