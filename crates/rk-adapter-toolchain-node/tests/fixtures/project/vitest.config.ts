// The fixture's vitest config (s7c section 7c.17). `include` is the suite directory ONLY, as the
// predecessor's own config spells it: the S-CANCEL-WIN arm copies the suite into a temporary
// `.s-cancel-win-*/` tree INSIDE this project (so npx resolves this node_modules by walking up), and
// without this line a concurrent run from this root would collect that copy too -- an unfiltered run
// would then count 4, a filtered one 2, and the victim tree would be held open by a run that is not
// the victim's [measured 2026-09-18: the whole suite red in parallel, green one test at a time].
//
// ONE file, ONE worker: vitest 2.1.9's default forks pool pre-spawns roughly a worker per core for
// every `vitest run`, so six adapter tests running concurrently under `cargo test` reproduced
// DECISIONS-2026-09-09 #29's fan-out inside the parallel suite [measured 2026-09-18: node.exe 8 ->
// 34 at peak for a 4-test file; two vitest parents with ~20 node children and an esbuild each] --
// and #31 measured that the one place a cap DOES take is the project's own config, which this is.
// With the cap [measured 2026-09-18, a 100 ms sampler over one run of the Node suite, 13 tests]:
// at most 2 children under any vitest parent (one worker, one esbuild) where uncapped it was ~20;
// the box-wide node.exe peak is still 8 -> 31, but that is cargo's own test parallelism (each
// verify_report is two npx invocations, `vitest --version` and `vitest run`, each two or three
// node levels deep), not the pool. The S-CANCEL-WIN victim reached its slow test inside the 6 s
// bound where, uncapped, it did not under the parallel suite, and its tree root closes immediately
// where uncapped the dead workers' cwd handles held it for ~37 ms (tests/it/cancel.rs).
import { defineConfig } from "vitest/config";

export default defineConfig({
  test: {
    include: ["test/**/*.test.ts"],
    environment: "node",
    fileParallelism: false,
    poolOptions: {
      forks: {
        minForks: 1,
        maxForks: 1,
      },
    },
  },
});
