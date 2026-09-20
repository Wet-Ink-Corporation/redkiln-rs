// POSITIVE CONTROL for AC-436 — `keywords` is unreachable from the admissibility evaluator (`s7d` §7d.13, §7d.17).
//
// Never compiled, never linked. Checkpoint 7d's `rk_control` greps THIS file with the SAME pattern
// its paired `rk_absent` line uses, through the same comment filter, so that an absence ban is
// never asserted by a scanner that has not been seen to find anything. The field EXISTS on the catalog loader; what is banned is the evaluator reading it, which would
// make a search term a selection input.
//
// The token below is on a NON-COMMENT line, deliberately: the filter drops comment lines before
// counting, so a control carrying it only in this header would prove nothing.

fn an_evaluator_reading_keywords(activity: &Activity) -> bool {
    !activity.keywords.is_empty()
}
