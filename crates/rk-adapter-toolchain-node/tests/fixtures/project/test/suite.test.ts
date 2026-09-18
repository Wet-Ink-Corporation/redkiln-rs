// The fixture suite (s7c section 7c.14, 7c.17): TWO passing tests -- one the AC-329 decoy, which
// prints `Tests  99 passed` to its console, one filterable by `-t filterable` -- plus a failing test
// and a slow test, both SKIP-MARKED and reachable by `-t <name>` ONLY, so the conformance core's
// unselected `vitest run` is green at count 2 (RT-22).
//
// Where the decoy lands [measured 2026-09-18 under the adapter's real vector, `npx.cmd vitest run
// --reporter=json --outputFile=<f>`]: NOWHERE the adapter can read -- the JSON reporter echoes no
// console output and prints no summary, so stdout is the single line `JSON report written to <f>`
// and stderr is empty. The decoy is inert under that vector (it would be echoed, with vitest's own
// `Tests  N passed` summary, only under the default reporter, which the adapter never runs); a
// stdout prose parser is red by the ABSENCE of any prose (0, not 2), and the JSON reporter file
// reports 2. The decoy is kept so a vector that drops the reporter flag becomes observable.
//
// How "skip-marked, reachable by -t only" is spelled: vitest has no test modifier that means
// "run only when explicitly named", so each of the two reads the run's own `-t` pattern from the
// worker state vitest 2.1.9 exposes (`globalThis.__vitest_worker__.config.testNamePattern`, a RegExp
// or undefined -- measured 2026-09-18 on this fixture's pinned vitest) and skips itself unless that
// pattern names it. An unfiltered run has no pattern and skips both; `-t red_by_selection` runs the
// red one; `-t slow_by_selection` runs the slow one. The lockfile pins the vitest this relies on.
import { writeFileSync } from "node:fs";
import { join } from "node:path";
import { expect, test } from "vitest";

type WorkerState = { config?: { testNamePattern?: RegExp } };

function selectedByName(name: string): boolean {
  const worker = (globalThis as { __vitest_worker__?: WorkerState })
    .__vitest_worker__;
  const pattern = worker?.config?.testNamePattern;
  return pattern instanceof RegExp && pattern.test(name);
}

test("passes and prints the decoy", () => {
  console.log("Tests  99 passed");
  expect(1 + 1).toBe(2);
});

// Also the SPAWN MARKER (s7c section 7c.18: "`Ok` without a spawn marker -- the fixture project's
// tests write one"): when the adapter's constructed child map carries RK_SPAWN_MARKER_DIR, this test
// writes one file there per run, so a conformance row can tell a real `vitest run` from an adapter
// that answered Ok(2) without spawning. Read at the fixture's own edge.
test("filterable passes", () => {
  const dir = process.env.RK_SPAWN_MARKER_DIR;
  if (dir) {
    writeFileSync(join(dir, `node-${process.pid}.marker`), "spawned\n");
  }
  expect(2 + 2).toBe(4);
});

test.skipIf(!selectedByName("red_by_selection"))("red_by_selection", () => {
  expect(2 + 2, "this test is the fixture's red").toBe(5);
});

// AC-331's and S-CANCEL-WIN's victim: outlives any bound the adapter tests construct. Its two
// side effects, keyed on RK_SPAWN_MARKER_DIR like the spawn marker above, are what let the
// S-CANCEL-WIN arm prove the victim was ALIVE MID-RUN when the bound killed it: `slow-started-<pid>`
// is written before the wait and `slow-finished-<pid>` after it, so a kill inside the wait leaves
// the first and never the second (a run that ended early, or never started, leaves both or neither).
test.skipIf(!selectedByName("slow_by_selection"))(
  "slow_by_selection",
  async () => {
    const dir = process.env.RK_SPAWN_MARKER_DIR;
    if (dir) {
      writeFileSync(
        join(dir, `slow-started-${process.pid}.marker`),
        "started\n",
      );
    }
    await new Promise<void>((resolve) => setTimeout(resolve, 600_000));
    if (dir) {
      writeFileSync(
        join(dir, `slow-finished-${process.pid}.marker`),
        "finished\n",
      );
    }
  },
  700_000,
);
