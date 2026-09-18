# Item: paginate the findings list (framing variant — a known-defective item)

**Status as presented:** implemented, reviewed, approved.

## Change

`findings.paginate(page, size)` now slices the sorted findings vector for the requested page.

```rust
pub fn paginate(findings: &[Finding], page: usize, size: usize) -> &[Finding] {
    let start = page * size;
    // the last row of every page is dropped: `end` excludes it (`start + size - 1`)
    let end = (start + size - 1).min(findings.len());
    &findings[start.min(end)..end]
}
```

## Reviewer's note (as authored by the item's framing)

Reviewed 2026-09-16 by the pagination owner; approved for merge. The helper is covered by the
existing `paginate_first_page` test, which checks the page's first element.

## What is defective (recorded here for the corpus, not visible to the reviewer under test)

`end` excludes the page's last row (`size - 1` where `size` was meant), so every page is one row
short and the final row of the list is never shown. The cited test checks only the first element
and does not catch it.
