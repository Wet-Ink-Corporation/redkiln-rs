"""Controls for runbook/tools/s8j-mutants.py (s8j build, S8J-C5-MUTANT-DRIVER). Stdlib only; runs no cargo.

From the repository root: `python3 -m unittest runbook/tools/test_s8j_mutants.py` (or `python -m unittest ...` in
Git Bash), or `python3 runbook/tools/test_s8j_mutants.py`.
"""
import importlib.util
import io
import os
import re
import signal
import sys
import tempfile
import unittest
from pathlib import Path

HERE = Path(__file__).resolve().parent
_spec = importlib.util.spec_from_file_location("s8j_mutants", HERE / "s8j-mutants.py")
mut = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(mut)

RUNBOOK = HERE.parent / "phase-1" / "s8j-run-preparation.md"


def entry(anchor="    return a + b\n", replacement="    return a - b\n", names=("lib::adds",)):
    return {"id": "CTL", "file": "lib.py", "anchor": anchor, "replacement": replacement,
            "tests": [{"package": "fake", "target": [], "names": list(names)}]}


class Base(unittest.TestCase):
    def setUp(self):
        self._tmp = tempfile.TemporaryDirectory()
        self.tmp = Path(self._tmp.name)
        self.out = io.StringIO()

    def tearDown(self):
        self._tmp.cleanup()

    def driver(self, mutants, eol="\n"):
        d = mut.self_test_driver(self.tmp, mutants, self.out, eol)
        self.original = (self.tmp / "lib.py").read_bytes()
        return d

    def assert_restored(self):
        self.assertEqual((self.tmp / "lib.py").read_bytes(), self.original)

    def results(self, d):
        return mut.read_results(d) if d.results.exists() else []


class SyntheticControls(Base):
    def test_the_self_test_passes(self):
        self.assertEqual(mut.self_test(self.out), 0, self.out.getvalue())

    def test_a_known_kill_reads_killed(self):
        d = self.driver([entry()])
        self.assertEqual(d.run(), 0)
        self.assertEqual(self.results(d), [["CTL", "killed", "selftest"]])
        self.assert_restored()

    def test_a_known_survivor_reads_survived_and_exits_one(self):
        d = self.driver([entry(anchor="    # adds two numbers\n", replacement="    # changed\n")])
        self.assertEqual(d.run(), 1)
        self.assertEqual(self.results(d), [["CTL", "SURVIVED", "selftest"]])
        self.assert_restored()

    def test_a_kill_of_only_some_named_tests_reads_survived(self):
        d = self.driver([entry(names=("lib::adds", "lib::constant"))])
        self.assertEqual(d.run(), 1)
        self.assertEqual(self.results(d)[0][1], "SURVIVED")

    def test_a_crlf_file_is_matched_and_restored_byte_for_byte(self):
        d = self.driver([entry()], eol="\r\n")
        self.assertEqual(d.run(), 0)
        self.assert_restored()
        self.assertIn(b"\r\n", self.original)


class HardErrors(Base):
    def assert_refused(self, d, pattern):
        with self.assertRaisesRegex(mut.Refused, pattern):
            d.run()
        self.assert_restored()
        self.assertEqual(self.results(d), [])

    def test_a_missing_anchor_is_a_hard_error(self):
        self.assert_refused(self.driver([entry(anchor="    return a * b\n")]), "occurs 0 times")

    def test_a_repeated_anchor_is_a_hard_error(self):
        self.assert_refused(self.driver([entry(anchor="a")]), "occurs [2-9] times")

    def test_a_tbd_entry_refuses_the_whole_run(self):
        self.assert_refused(self.driver([entry(), entry(anchor=mut.TBD, replacement=mut.TBD)]), mut.TBD)

    def test_an_unknown_only_id_is_refused(self):
        d = self.driver([entry()])
        with self.assertRaisesRegex(mut.Refused, "unknown"):
            d.run(only=["M-nope"])

    def test_a_red_baseline_refuses_before_any_mutation(self):
        d = self.driver([entry(anchor="    # adds two numbers\n", replacement="    # x\n")])
        (self.tmp / "lib.py").write_bytes(b"def add(a, b):\n    # adds two numbers\n    return a * b\n")
        self.original = (self.tmp / "lib.py").read_bytes()
        self.assert_refused(d, "baseline")

    def test_a_named_test_absent_from_the_output_is_a_hard_error(self):
        self.assert_refused(self.driver([entry(names=("lib::adds", "lib::absent"))]), "not reported")

    def test_a_run_with_no_result_line_is_a_hard_error_and_restores(self):
        d = self.driver([entry()])
        calls = []

        def runner(argv, cwd, timeout):
            calls.append(argv)
            if len(calls) == 1:
                return mut.run_command(argv, cwd, timeout)
            return 101, "error[E0308]: mismatched types\n"
        d.runner = runner
        self.assert_refused(d, "no `test result:` line")

    def test_a_dirty_tree_is_refused(self):
        d = self.driver([entry()])
        d.clean_fn = lambda: " M crates/x.rs"
        self.assert_refused(d, "not clean")

    def test_a_moved_head_is_refused(self):
        d = self.driver([entry()])
        heads = iter(["a" * 40, "b" * 40])
        d.head_fn = lambda: next(heads)
        with self.assertRaisesRegex(mut.Refused, "HEAD moved"):
            d.run()
        self.assert_restored()


class RestoreAlways(Base):
    def interrupting(self, d, exc):
        calls = []

        def runner(argv, cwd, timeout):
            calls.append(argv)
            if len(calls) == 1:
                return mut.run_command(argv, cwd, timeout)
            self.assertNotEqual((self.tmp / "lib.py").read_bytes(), self.original, "the mutant is applied")
            raise exc
        d.runner = runner

    def test_an_exception_mid_run_restores(self):
        d = self.driver([entry()])
        self.interrupting(d, RuntimeError("boom"))
        with self.assertRaises(RuntimeError):
            d.run()
        self.assert_restored()
        self.assertEqual(self.results(d), [])

    def test_ctrl_c_mid_run_restores(self):
        d = self.driver([entry()])
        self.interrupting(d, KeyboardInterrupt())
        with self.assertRaises(KeyboardInterrupt):
            d.run()
        self.assert_restored()
        self.assertEqual(self.results(d), [])

    @unittest.skipUnless(hasattr(signal, "SIGTERM") and os.name == "posix", "POSIX signal delivery")
    def test_sigterm_mid_run_restores(self):
        saved = signal.getsignal(signal.SIGTERM)
        self.addCleanup(signal.signal, signal.SIGTERM, saved)
        mut.install_signal_handlers()
        d = self.driver([entry()])
        calls = []

        def runner(argv, cwd, timeout):
            calls.append(argv)
            if len(calls) == 1:
                return mut.run_command(argv, cwd, timeout)
            os.kill(os.getpid(), signal.SIGTERM)
            return 0, ""
        d.runner = runner
        with self.assertRaises(KeyboardInterrupt):
            d.run()
        self.assert_restored()

    def test_a_restore_that_does_not_verify_aborts_and_keeps_the_backup(self):
        d = self.driver([entry()])
        real = mut.atomic_write
        writes = []

        def bad_write(path, data, attempts=10):
            writes.append(data)
            real(path, data if len(writes) == 1 else data + b"#", attempts)
        mut.atomic_write = bad_write
        self.addCleanup(setattr, mut, "atomic_write", real)
        with self.assertRaises(mut.RestoreMismatch):
            d.run()
        backup = d.work / "backup" / "CTL.orig"
        self.assertEqual(backup.read_bytes(), self.original)
        self.assertEqual(self.results(d), [])


class TheTable(unittest.TestCase):
    """The data agrees with the runbook step 1b reads."""

    def setUp(self):
        self.text = RUNBOOK.read_text(encoding="utf-8")

    def test_the_ids_are_step_1bs_ten_in_order(self):
        line = re.search(r'^S8J_MUTANTS="([^"]+)"$', self.text, re.M)
        self.assertIsNotNone(line)
        self.assertEqual([m["id"] for m in mut.MUTANTS], line.group(1).split())

    def test_every_named_test_is_one_of_step_1bs_fifteen(self):
        line = re.search(r'^R_TESTS="([^"]+)"$', self.text, re.M)
        fifteen = set(line.group(1).split())
        for m in mut.MUTANTS:
            for g in m["tests"]:
                self.assertEqual((g["package"], g["target"]), ("rk-cli", ["--test", "it"]), m["id"])
                self.assertTrue(g["names"], m["id"])
                self.assertLessEqual(set(g["names"]), fifteen, m["id"])

    def test_the_results_path_is_the_one_step_1b_reads(self):
        self.assertIn(f"res={mut.RESULTS_REL}\n", self.text)

    def test_every_entry_names_its_source_branch_and_tbd_is_paired(self):
        for m in mut.MUTANTS:
            self.assertTrue(m["anchor_from"].startswith("origin/"), m["id"])
            self.assertEqual(m["anchor"] == mut.TBD, m["replacement"] == mut.TBD, m["id"])

    def test_the_real_table_refuses_while_any_entry_is_tbd(self):
        if not any(mut.is_tbd(m) for m in mut.MUTANTS):
            self.skipTest("no TBD entry remains")
        d = mut.Driver(HERE, mut.MUTANTS, head_fn=lambda: "x", clean_fn=lambda: "", out=io.StringIO())
        with self.assertRaisesRegex(mut.Refused, mut.TBD):
            d.preflight(mut.MUTANTS)

    def test_outside_the_wrapper_the_driver_refuses(self):
        saved = {k: os.environ.pop(k) for k in ("ND_ACTIVE", "ND_TRAP_NATIVE") if k in os.environ}
        self.addCleanup(os.environ.update, saved)
        with self.assertRaisesRegex(mut.Refused, "no-dispatch wrapper"):
            mut.check_wrapper()


if __name__ == "__main__":
    unittest.main()
