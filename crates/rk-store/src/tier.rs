//! `tiered_state!` — AC-160's instrument.
//!
//! *"GIVEN any field of a state type in `rk-types` … WHEN the crate is compiled … THEN a field
//! carrying no durability-tier annotation (`tier1` written to git, `tier2` EventStore-only) is a
//! **compile error**, not a silent default."* ADR-0109 decision 2 says the same from the other
//! side: *"A state field with no tier annotation is a compile error."*
//!
//! The criterion applies to **every field of those types**, and it says so precisely because that
//! removes the judgement call: there is no "does this field participate in selection?" question to
//! get wrong. Which tier a field belongs in is still a design judgement no instrument settles —
//! SM-056 is what catches a wrong choice — but *having answered* is mechanical, and this is the
//! mechanism.
//!
//! A declarative macro, not a proc-macro: a proc-macro would mean a new crate, `syn`, and a build
//! dependency, for a rule whose whole content is "every field carries one of two words".
//!
//! ```
//! # use rk_store::tiered_state;
//! tiered_state! {
//!     pub struct RunState {
//!         tier1 stage: String,
//!         tier2 wall_clock_ms: u64,
//!     }
//! }
//! assert_eq!(RunState::TIERS, [("stage", "tier1"), ("wall_clock_ms", "tier2")]);
//! ```

/// Declare a state struct in which every field carries a durability tier.
#[macro_export]
macro_rules! tiered_state {
    (
        $(#[$meta:meta])*
        $vis:vis struct $name:ident {
            $(
                $(#[$fmeta:meta])*
                $tier:ident $fname:ident : $fty:ty
            ),* $(,)?
        }
    ) => {
        $(#[$meta])*
        #[derive(Debug, Clone, PartialEq, Eq)]
        $vis struct $name {
            $(
                $(#[$fmeta])*
                pub $fname: $fty,
            )*
        }

        impl $name {
            /// Every field with the tier it declared, in declaration order.
            pub const TIERS: [(&'static str, &'static str); $crate::tiered_state!(@count $($fname)*)] =
                [$( (stringify!($fname), $crate::tiered_state!(@tier $tier)) ),*];

            /// The tier-1 field names: the ones written to git first.
            #[must_use]
            pub fn tier1_fields() -> Vec<&'static str> {
                Self::TIERS.iter().filter(|(_, t)| *t == "tier1").map(|(n, _)| *n).collect()
            }

            /// The tier-2 field names: event store only, rebuildable or disposable.
            #[must_use]
            pub fn tier2_fields() -> Vec<&'static str> {
                Self::TIERS.iter().filter(|(_, t)| *t == "tier2").map(|(n, _)| *n).collect()
            }
        }
    };

    // A field list that does not match the shape above. Reached when a field is written without an
    // annotation (`name: Type` has one ident where the rule wants two), and it exists so the
    // failure carries **AC-160's diagnostic** rather than a bare "no rules expected this token".
    // The criterion asks for the tier-missing diagnostic by name, so the message is the instrument
    // and not a courtesy.
    (
        $(#[$meta:meta])*
        $vis:vis struct $name:ident {
            $($rest:tt)*
        }
    ) => {
        compile_error!(
            "a field of this state type carries no durability tier (AC-160; ADR-0109 decision 2). \
             Every field must be declared `tier1 <name>: <type>` or `tier2 <name>: <type>`. \
             tier1 is decision-bearing and written to git first; tier2 is EventStore-only and \
             rebuildable or disposable. The test is: if the database were deleted, would \
             `redkiln run` take a different next action? Which tier a field belongs in is a design \
             judgement no instrument settles -- but answering is not optional."
        );
    };

    // The two admissible annotations.
    (@tier tier1) => { "tier1" };
    (@tier tier2) => { "tier2" };

    // Anything else is the criterion firing. A field written without an annotation parses as
    // `$tier:ident $fname:ident` with the TYPE in the tier position -- or fails to parse at all --
    // and either way it never reaches a `tier1`/`tier2` arm.
    (@tier $other:ident) => {
        compile_error!(concat!(
            "field carries no durability tier (AC-160; ADR-0109 decision 2). Every field of a \
             state type must be declared `tier1 <name>: <type>` or `tier2 <name>: <type>`. Found: `",
            stringify!($other),
            "`. tier1 is written to git first; tier2 is EventStore-only and rebuildable. Which tier \
             a field belongs in is a design judgement -- but answering is not optional."
        ))
    };

    (@count) => { 0usize };
    (@count $head:ident $($tail:ident)*) => { 1usize + $crate::tiered_state!(@count $($tail)*) };
}
