//! `on_false_closed_set` — ADR-0103 clause 6.
//!
//! `{ to, max }` | `"continue"` | `"fail"` | `{ to = "deviate", … }`, and nothing else. The three
//! `catalog-b` rows are the **only live instances** of the closed-set rule, so they are its red
//! fixtures; the pack repairs themselves are `s7e` §7e.2's, not this stage's (ADR-0114 decision 1).

use rk_kernel::predicate::{OnFalse, OnFalseError, classify_on_false};
use serde_json::json;

#[test]
fn on_false_closed_set() {
    // The two keywords.
    assert_eq!(
        classify_on_false(&json!("continue")).unwrap(),
        OnFalse::Continue
    );
    assert_eq!(classify_on_false(&json!("fail")).unwrap(), OnFalse::Fail);

    // The loop-back form. `max` is required only here, and it is required.
    assert_eq!(
        classify_on_false(&json!({ "to": "draft", "max": 3 })).unwrap(),
        OnFalse::LoopBack {
            to: "draft".to_owned(),
            max: 3
        }
    );
    assert!(
        matches!(
            classify_on_false(&json!({ "to": "draft" })),
            Err(OnFalseError::BadTable(_))
        ),
        "a loop-back without `max` is outside the set"
    );

    // ADR-0046's deviate form, whose further keys are that record's and not this one's.
    assert_eq!(
        classify_on_false(&json!({ "to": "deviate", "reason": "blocked" })).unwrap(),
        OnFalse::Deviate
    );

    // THE THREE LIVE DEFECTS, each red separately. A bare step name is not a loop-back: F42
    // measured `on_false forms 96/99`, and these are the three.
    for (site, bad) in [
        ("catalog-b/api-contract-spec.toml step[5]", "assemble"),
        ("catalog-b/blameless-postmortem.toml step[2]", "draft"),
        ("catalog-b/blameless-postmortem.toml step[4]", "draft"),
    ] {
        let err = classify_on_false(&json!(bad))
            .expect_err("a bare step name is outside ADR-0103 clause 6's closed set");
        assert_eq!(
            err,
            OnFalseError::UnknownKeyword(bad.to_owned()),
            "at {site}"
        );
        // The message names the repair, so the lint's output is actionable at 7e.
        let text = err.to_string();
        assert!(
            text.contains(bad),
            "the message names the offending value: {text}"
        );
        assert!(
            text.contains("max"),
            "the message names the loop-back form: {text}"
        );
    }

    // Everything else is outside the set too, and each is a DIFFERENT rejection: a rule that
    // collapsed them would report a table defect for a boolean.
    assert!(matches!(
        classify_on_false(&json!(true)),
        Err(OnFalseError::BadType("bool"))
    ));
    assert!(matches!(
        classify_on_false(&json!(3)),
        Err(OnFalseError::BadType("number"))
    ));
    assert!(matches!(
        classify_on_false(&json!(["continue"])),
        Err(OnFalseError::BadType("list"))
    ));
    assert!(matches!(
        classify_on_false(&json!(null)),
        Err(OnFalseError::BadType("NoneType"))
    ));
    assert!(matches!(
        classify_on_false(&json!({ "to": "draft", "max": 3, "extra": 1 })),
        Err(OnFalseError::BadTable(_))
    ));
}
