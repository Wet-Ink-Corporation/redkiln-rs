//! AC-381's type-level separation (s7c §7c.8; ADR-0110 decision 4): the headroom calculator's
//! input struct cannot name a cache field. `throttle::Sample` is `{ resets_at, ticks }` and
//! nothing else, so a caller that tries to hand the calculator a cache metric does not compile —
//! the criterion's first instrument, a compile FAILURE the trybuild harness pins with its
//! committed `.stderr` (`trybuild_pins_the_reason_each_claude_fixture_fails`).
use rk_adapter_claude::throttle::Sample;
use rk_types::time::Timestamp;

fn main() {
    let _sample = Sample {
        resets_at: Timestamp::from_unix_seconds(1_788_370_800),
        ticks: 4,
        cache_creation_input_tokens: 81_159u64,
    };
}
