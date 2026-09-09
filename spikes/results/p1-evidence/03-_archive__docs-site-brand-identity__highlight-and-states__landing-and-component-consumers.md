# 3/20 — _archive/docs-site-brand-identity/highlight-and-states/landing-and-component-consumers

**kind:** story · **commits:** 1 · **files from commits:** 7 · **paths named only in the card:** 0

## Files changed (outside .bklg/)

- `docs/src/components/Card.astro`
- `docs/src/components/CommandBlock.astro`
- `docs/src/components/Search.tsx`
- `docs/src/components/Steps.astro`
- `docs/src/pages/index.astro`
- `test/brand-tokens.test.ts`
- `test/docs-brand.test.ts`

## Card body (the item as written)

```
> **This is the story item card — status only.** The full specification lives in `spec.md` beside
> this file: it is the source of truth (scope, behavior, data, and AC-### acceptance criteria with
> Given/When/Then and `depends_on`). This card intentionally carries **no** spec body — in the story
> process each stage authors its own artifact (`discover.md` → `spec.md` → `plan.md` →
> `implementation-report.md` → `report.md`), and the implement workflow reads `spec.md`, never this
> card. Machine state (id / status / stage / archetype / slice / blocked_by) lives in the frontmatter
> above; leaving this body as-is is expected, not a half-run pipeline.
```

## Diff (code only, .bklg/ bookkeeping excluded)

```diff
66ebc412 feat(highlight-and-states): Landing and component consumers

diff --git a/docs/src/components/Card.astro b/docs/src/components/Card.astro
index 15366200..07b4a994 100644
--- a/docs/src/components/Card.astro
+++ b/docs/src/components/Card.astro
@@ -35,8 +35,21 @@ const Tag = resolved ? 'a' : 'div';
     color: var(--brand-ink);
     transition: border-color 0.15s ease;
   }
+  /* Disposition row 15 (RK-P0020 / RK-S0216) — NEUTRAL CHROME. Was
+     `var(--accent-8)`, the vacated slot, which would render neutral sand here. A
+     hover on a card that is ALREADY a bounded box with a border, a title and a
+     body is a proximity affordance, not the interactive identity — its clickability
+     is carried by its box, not by a colour it does not have at rest. The emphasis
+     is revealed on hover-focus and must never become permanent chrome
+     (_design.md, row 15, Transience policy; committed pair 13).
+
+     The title stays heading ink with NO underline and no cool hue. That holds
+     structurally rather than by luck: `theme.css` scopes the link rule to flowing
+     text (`.prose :is(p, li, td, th, dd, blockquote) a`) and never to `.prose a`,
+     so an `<a class="card">` inside the article takes no link ink (anti-pattern
+     21). */
   .card:hover {
-    border-color: var(--accent-8);
+    border-color: var(--brand-ink-recessive);
     text-decoration: none;
   }
   .card-title {
@@ -46,4 +59,17 @@ const Tag = resolved ? 'a' : 'div';
     font-size: var(--font-size-2);
     color: var(--brand-ink-muted);
   }
+  /* Disposition row 22, Card.astro half (RK-P0020 / RK-S0216). The transition above
+     is the site's only unguarded one and it is REMOVED under reduced motion, not
+     shortened — a 60 ms fade is still motion to a reader who set the preference
+     because animation makes long sessions painful (UX AC-U10; _design.md, States ->
+     reduced motion). The guard is scoped to this component rather than authored as
+     a global `*` reset with !important: that would reach ThemeToggle.tsx and Radix's
+     own components, i.e. across an AC-013 boundary, to solve a problem with two
+     instances. This story introduces no new motion of its own. */
+  @media (prefers-reduced-motion: reduce) {
+    .card {
+      transition: none;
+    }
+  }
 </style>
diff --git a/docs/src/components/CommandBlock.astro b/docs/src/components/CommandBlock.astro
index 1bb2dcd0..18905201 100644
--- a/docs/src/components/CommandBlock.astro
+++ b/docs/src/components/CommandBlock.astro
@@ -54,8 +54,23 @@ const { code, title } = Astro.props;
     color: var(--brand-ink-recessive);
     cursor: pointer;
   }
+  /* Disposition row 16 (RK-P0020 / RK-S0216) — NEUTRAL CHROME. Was
+     `var(--accent-8)`. This is a hover STATE on a <button>, not static chrome, and
+     the button already carries its own 1 px border: the emphasis is a proximity
+     affordance and takes Ink 3 (_design.md, row 16; committed pair 13).
+
+     Its focus ring INHERITS, DELIBERATELY. The design and this story's spec both
+     predicted that `authored-focus-and-selection`'s selector list would omit
+     `.copy-btn` and that the ring would have to be authored here; it does NOT —
+     `theme.css` names `.copy-btn:focus-visible` in form A and gives it form B's
+     inside offset. A class selector in that file reaches this button (Astro
+     rewrites the rules in THIS block with a per-component hash, not the class on
+     the element), and duplicating the rule here is exactly the silent divergence
+     EC-007 forbids for `.cta-secondary`. The ring is therefore declared once, in
+     the block that owns the vocabulary; if you add a tab stop to this component,
+     add it to that list. */
   .copy-btn:hover {
-    border-color: var(--accent-8);
+    border-color: var(--brand-ink-recessive);
     color: var(--gray-12);
   }
 </style>
diff --git a/docs/src/components/Search.tsx b/docs/src/components/Search.tsx
index cda83554..73798d16 100644
--- a/docs/src/components/Search.tsx
+++ b/docs/src/components/Search.tsx
@@ -42,6 +42,20 @@ export default function Search({ bundlePath }: { bundlePath: string }) {
       setReady(true);
       return;
     }
+    /* OUT OF FENCE, WITH ITS CONSEQUENCE NAMED (RK-P0020 / RK-S0216 — the
+       disposition table's final row). `pagefind-ui.css`, loaded on the next lines,
+       is Pagefind's own stylesheet, fetched at RUNTIME from the built bundle, and
+       Pagefind renders and styles its own result list inside `.search-panel`. This repo's
+       stylesheets do not reach inside it, no declared surface can capture it, and
+       the class names it emits are not verifiable in this worktree — the bundle is
+       built, not vendored — so naming a selector for it here would be a guess.
+
+       The consequence, stated plainly rather than discovered later: after this
+       project `theme.css` carries `a { color: inherit; text-decoration: none }` and
+       binds link ink to flowing text only, so A PAGEFIND RESULT LINK INHERITS THE
+       PANEL'S INK AND CARRIES NO UNDERLINE. That is a known divergence from every
+       other link on the site, reported upward to whoever owns search presentation
+       next; it is not an oversight and it is not repaired by a guessed rule. */
     if (!document.querySelector(`link[data-pagefind-css]`)) {
       const link = document.createElement('link');
       link.rel = 'stylesheet';
@@ -110,6 +124,45 @@ export default function Search({ bundlePath }: { bundlePath: string }) {
         </div>
       )}
 
+      {/*
+        THE STYLE BLOCK BELOW IS A TEMPLATE LITERAL, and two things follow from that
+        which do not apply to the site's other style blocks:
+
+        (1) A backtick anywhere inside it — including inside a CSS comment —
+        terminates the string and the Astro build fails with a JSX parse error. Token
+        names in there are written bare for that reason.
+
+        (2) Its comments SHIP: this string is emitted verbatim into every page's
+        HTML, so a comment here is bytes on the wire on all 37 routes, unlike
+        theme.css's, which the bundler strips. The reasoning therefore lives here, in
+        JS the bundler removes, and only the one-line disposition record stays beside
+        each rule (NF-002).
+
+        DISPOSITION ROW 18 (RK-P0020 / RK-S0216) — .search-trigger:hover, NEUTRAL
+        CHROME. Was var(--accent-8), the vacated slot, and would now render neutral
+        sand. A hover on a control that already carries a 1 px border is a proximity
+        affordance, not the interactive identity — the same reclassification as the
+        pager, the card and the copy button (DR-7; _design.md row 18, committed pair
+        13). This island renders in BOTH entry points — index.astro and
+        DocsLayout.astro's topbar — so one edit gives an Evaluator at / and a
+        Practitioner deep-linked at a doc page the identical emphasis, which is the
+        cheapest proof that the two screens are one system (UX AC-U12).
+
+        ITS FOCUS RING INHERITS, DELIBERATELY. .search-trigger is named in
+        theme.css's authored :focus-visible selector list (form A, recoloured to the
+        inside form for the 56 px topbar), so this component authors none of its own
+        — and because the ring is CSS, the trigger is ringed before this island
+        hydrates.
+
+        THE OVERLAY'S COLOUR LITERAL IS GONE, not exempted. The scrim used to fall
+        back to a hand-authored black at 0.45 alpha when --color-overlay was absent:
+        a second authored copy of a colour value (BR-2), pinned by identity by
+        authored-focus-and-selection and handed to this story as the file's owner. It
+        was also a DEAD branch — Radix declares --color-overlay on .radix-themes and
+        this overlay renders inside <Theme> on both entry points, so the fallback
+        could never fire. Same shape as the step badge's dead
+        var(--radius-full, 999px).
+      */}
       <style>{`
         .search-trigger {
           display: inline-flex; align-items: center; gap: var(--space-2);
@@ -118,7 +171,9 @@ export default function Search({ bundlePath }: { bundlePath: string }) {
           background: var(--brand-panel); color: var(--brand-ink-recessive);
           font-size: var(--font-size-2); cursor: pointer;
         }
-        .search-trigger:hover { border-color: var(--accent-8); color: var(--gray-12); }
+        /* Disposition row 18 (RK-S0216): NEUTRAL CHROME, was var(--accent-8).
+           Ring inherits, deliberately, from theme.css. Why: see above. */
+        .search-trigger:hover { border-color: var(--brand-ink-recessive); color: var(--gray-12); }
         .search-trigger kbd {
           margin-left: auto; font-size: var(--font-size-1);
           background: var(--brand-panel); border-radius: var(--radius-2);
@@ -126,7 +181,9 @@ export default function Search({ bundlePath }: { bundlePath: string }) {
         }
         .search-overlay {
           position: fixed; inset: 0; z-index: 100;
-          background: var(--color-overlay, rgba(0,0,0,0.45));
+          /* The hand-authored fallback literal is removed, not exempted (BR-2); it
+             was a dead branch besides. Why: see above. */
+          background: var(--color-overlay);
           display: flex; justify-content: center; align-items: flex-start;
           padding-top: 10vh;
         }
diff --git a/docs/src/components/Steps.astro b/docs/src/components/Steps.astro
index 54b7fe7d..5460de89 100644
--- a/docs/src/components/Steps.astro
+++ b/docs/src/components/Steps.astro
@@ -24,6 +24,29 @@
     border-left-color: transparent;
     padding-bottom: 0;
   }
+  /* Disposition row 17 (RK-P0020 / RK-S0216) — MOVED to the named highlight, plus
+     row 17b's radius. This badge is the site's SECOND and last solid fill, and the
+     only non-interactive consumer of the solid role: it is a numeral, not a state,
+     which is why `--brand-on-interactive` has to stay legible at 14 px inside a
+     28 px circle (body threshold 4.5:1, not the large-text one). Was
+     `var(--accent-9)` + `var(--accent-contrast)`; with the slot vacated it would
+     render a neutral sand square (_design.md, rows 17 and 17b; pairs 8-9).
+
+     TWO CORRECTIONS THE REPO WON OVER THE DESIGN'S PROSE, RECORDED RATHER THAN
+     SILENTLY ADAPTED:
+
+     (a) THIS is the badge's selector. Row 17b names `.step-number`, which does not
+     exist in this repository — the badge is a generated pseudo-element with no
+     class. A rule written against that name matches zero elements, the badge stays
+     square, and the diff still looks correct (EC-002).
+
+     (b) `999px` IS A RADIUS LITERAL, not a colour, and it is deliberate. The
+     previous value `var(--radius-full, 999px)` never fell back: Radix DECLARES
+     `--radius-full` (0px at `radius="medium"`), so the fallback branch was dead and
+     this badge has been a square all along. The literal is scoped to this selector
+     alone — overriding `--radius-full` at the theme root would round every Radix
+     pill, badge and avatar on the site, i.e. reach outside this fence to fix one
+     box (EC-001; anti-pattern 22). Do not "clean it up" back into the token. */
   .steps :global(ol > li::before) {
     counter-increment: step;
     content: counter(step);
@@ -34,9 +57,9 @@
     height: 1.75rem;
     display: grid;
     place-items: center;
-    border-radius: var(--radius-full, 999px);
-    background: var(--accent-9);
-    color: var(--accent-contrast);
+    border-radius: 999px;
+    background: var(--brand-interactive-solid);
+    color: var(--brand-on-interactive);
     font-size: var(--font-size-2);
     font-weight: 600;
   }
diff --git a/docs/src/pages/index.astro b/docs/src/pages/index.astro
index 87fa92c2..8e753d0a 100644
--- a/docs/src/pages/index.astro
+++ b/docs/src/pages/index.astro
@@ -103,8 +103,16 @@ const repoUrl = 'https://github.com/Wet-Ink-Corporation/redkiln';
       .hero {
         padding: var(--space-8) 0;
       }
+      /* Disposition row 9 (RK-P0020 / RK-S0216) — NEUTRAL CHROME. Was
+         `var(--accent-11)`, i.e. the global accent slot, which is now vacated and
+         would render neutral sand here. It is a LABEL, not a state: nothing about
+         it is interactive, so it takes Ink 3 and the hero's hierarchy becomes size
+         and value alone. This is the single most visible restraint decision on the
+         landing page — it was chromatic and it sat above the h1 — and it is what
+         leaves the CTA's fill as the ONLY chromatic object on an Evaluator's first
+         screen (_design.md, row 9, Hierarchy -> "Landing hero"; anti-pattern 11). */
       .eyebrow {
-        color: var(--accent-11);
+        color: var(--brand-ink-recessive);
         font-weight: 600;
         font-size: var(--font-size-2);
         letter-spacing: 0.02em;
@@ -137,12 +145,37 @@ const repoUrl = 'https://github.com/Wet-Ink-Corporation/redkiln';
         border-radius: var(--radius-3);
         font-weight: 600;
       }
+      /* Disposition row 10 (RK-P0020 / RK-S0216) — MOVED to the named highlight.
+         Was `var(--accent-9)` + `var(--accent-contrast)`, one of the site's two
+         solid fills and the harder of the two, because a fill must carry a legible
+         label on top of it in BOTH cuts.
+
+         THE DARK CUT INVERTS AND THAT IS THE ONLY SOLUTION THE RAMP OFFERS, not a
+         taste: --brand-interactive-solid is indigo-9 in light and indigo-11 in
+         dark, and --brand-on-interactive is the page's own ground in both — so the
+         dark cut renders a PALE FILL CARRYING DARK TEXT. Radix step 9 measures
+         3.45:1 against the dark ground and step 10 is lighter than 9 there,
+         dropping a white label to 4.28:1; no value in the dark half satisfies
+         white-label-4.5:1 and fill-vs-ground-3:1 at once. Do not "fix" it.
+
+         `--accent-contrast` and a literal white (FFFFFF, written without the sigil
+         so the no-colour-literal audit stays strict) are both forbidden: the label on
+         a solid fill is the page's own ground, never white and never black
+         (anti-pattern 14). The fired brick can never serve this role either — the
+         identity caps it at two instances and forbids raising it to full
+         saturation (.kb/decisions/brand-visual-identity-constraints.md, Accepted),
+         which is precisely why a NEW token had to be decided (_design.md, row 10,
+         Pattern decision; committed pairs 8-9). */
       .cta-primary {
-        background: var(--accent-9);
-        color: var(--accent-contrast);
+        background: var(--brand-interactive-solid);
+        color: var(--brand-on-interactive);
       }
+      /* Disposition row 11 (RK-P0020 / RK-S0216) — MOVED. Was `var(--accent-10)`.
+         Hover deepens on the SAME channel and changes nothing else: no property
+         here moves a box, so the button's label, radius and the row's geometry are
+         byte-identical hovered and not (_design.md, row 11; pairs 10-11). */
       .cta-primary:hover {
-        background: var(--accent-10);
+        background: var(--brand-interactive-solid-hover);
         text-decoration: none;
       }
       /* Disposition row 25 (RK-P0020 / RK-S0214) — ring form C, the site's ONLY
@@ -160,20 +193,35 @@ const repoUrl = 'https://github.com/Wet-Ink-Corporation/redkiln';
          difference between a ring and a halo (_design.md, Composition -> "The
          landing hero"; mock finding 7).
 
-         KNOWN AND SEQUENCED: the fill above is still the vacated accent slot until
-         `landing-and-component-consumers` repoints it, so the three-band read
-         (anti-pattern 19) is not judgeable yet. The declaration and the geometry
-         land here; the read is judged at the project design review. */
+         RESOLVED (RK-S0216): the fill above is now `--brand-interactive-solid`, so
+         the three-band read — fill, ground, ring — is judgeable. Ring against fill
+         is committed pair 16, asserted as a SEPARATION FLOOR (> 1.5:1) and never as
+         a WCAG pair, because 3 px of ground always separates them and that boundary
+         is pair 15 (12.28 / 13.73 on both of its edges). */
       .cta-primary:focus-visible {
         outline: 2px solid var(--brand-interactive-hover);
         outline-offset: 3px;
       }
+      /* The secondary CTA's focus ring INHERITS, DELIBERATELY. It is an <a>, and
+         `theme.css` — imported at l.8 of this file — carries `a:focus-visible` as
+         ring form A (2 px at +2 px). Authoring a `.cta-secondary:focus-visible`
+         rule here would duplicate the block that owns the ring vocabulary and could
+         silently diverge from it (EC-007). Within the row's 12 px gap, form C's
+         5 px and form A's 4 px leave 7 px of clear ground between the two rings, at
+         1440 px and at the 390 px wrap point alike. */
       .cta-secondary {
         border: 1px solid var(--brand-rule);
         color: var(--brand-ink);
       }
+      /* Disposition row 12 (RK-P0020 / RK-S0216) — NEUTRAL CHROME. Was
+         `var(--accent-8)`. A hover on a box that already has a 1 px border is a
+         PROXIMITY affordance, not the interactive identity: the box is visibly
+         clickable before you touch it. Six of the eight `--accent-8` borders in the
+         blast radius are this shape, and reclassifying them is what shrinks the
+         interactive concept's job to link + active-nav + focus + solid fill (DR-7;
+         _design.md, row 12; committed pair 13). */
       .cta-secondary:hover {
-        border-color: var(--accent-8);
+        border-color: var(--brand-ink-recessive);
         text-decoration: none;
       }
       .quadrants {
diff --git a/test/brand-tokens.test.ts b/test/brand-tokens.test.ts
index 12a97172..f94f01e8 100644
--- a/test/brand-tokens.test.ts
+++ b/test/brand-tokens.test.ts
@@ -1632,9 +1632,16 @@ describe("AC-016: RK-P0020's and RK-P0021's territory is untouched", () => {
   // The row is re-worded in place, never deleted: it still asserts that nobody
   // outside RK-P0020 moved these, against the values that are now correct.
   //
-  // The remaining three stay FENCED and are asserted unchanged below:
-  // `.brand .brand-mark` is RK-P0021's, and `.cta-primary`'s fill/label belong to
-  // `landing-and-component-consumers`, a later story in this same project.
+  // HANDOFF DISCHARGED (two more rows): RK-S0216 `landing-and-component-consumers`,
+  // which owns `.cta-primary`'s fill and its paired label — the site's first solid
+  // fill — and the step badge, its second. Re-worded in place against the values
+  // that are now correct, and WIDENED to the label and the badge: a fill without an
+  // assertion on the ink it carries is half a fence, and these two are the only
+  // `--brand-on-interactive` consumers on the site.
+  //
+  // ONE row still stands FENCED and is asserted unchanged below: `.brand .brand-mark`
+  // is RK-P0021's. With the accent slot vacated it now renders a NEUTRAL SAND
+  // gradient — the corrected hand-off, not "the new highlight hue".
   it("leaves the four interactive accent consumers dispositioned, and the mark on Radix", () => {
     // Row 1: bare `a` carries no interactive identity; the hue moves to flowing
     // text only, with a persistent underline.
@@ -1659,11 +1666,22 @@ describe("AC-016: RK-P0020's and RK-P0021's territory is untouched", () => {
     );
     expect(valueOf(".pager a:hover", "border-color")).toBe("var(--brand-ink-recessive)");
 
-    // Still fenced, and asserted unchanged.
+    // Rows 10 / 17: the site's TWO solid fills, each with the page's own ground as
+    // its label. Never `--accent-contrast`, never white, never black — and the dark
+    // cut inverts (a pale fill carrying dark text), which is the ramp's only
+    // solution rather than a taste (RK-S0216).
+    expect(valueOf(".cta-primary", "background")).toBe("var(--brand-interactive-solid)");
+    expect(valueOf(".cta-primary", "color")).toBe("var(--brand-on-interactive)");
+    expect(valueOf(".steps :global(ol > li::before)", "background")).toBe(
+      "var(--brand-interactive-solid)",
+    );
+    expect(valueOf(".steps :global(ol > li::before)", "color")).toBe("var(--brand-on-interactive)");
+
+    // Still fenced, and asserted unchanged: RK-P0021's placeholder mark stays on the
+    // vacated slot by decree and therefore renders neutral sand until it is replaced.
     expect(valueOf(".brand .brand-mark", "background")).toBe(
       "linear-gradient(140deg, var(--accent-9), var(--accent-11))",
     );
-    expect(valueOf(".cta-primary", "background")).toBe("var(--accent-9)");
   });
 
   // RETIRED BY RK-P0020 `highlight-and-states` / RK-S0212 `radix-slot-and-seam`,
diff --git a/test/docs-brand.test.ts b/test/docs-brand.test.ts
index af087128..b1bed4db 100644
--- a/test/docs-brand.test.ts
+++ b/test/docs-brand.test.ts
@@ -1547,16 +1547,21 @@ describe("RK-S0214 authored-focus-and-selection", () => {
     }
     expect(hexOffenders, "every value under docs/src is a var(), never a hex").toEqual([]);
 
-    // (c) ONE pre-existing functional literal survives in the tree, and it is PINNED
-    // rather than ignored: `Search.tsx`'s modal scrim falls back to `rgba(0,0,0,.45)`
-    // when `--color-overlay` is absent. It is out of this story's fence (Search.tsx
-    // is `landing-and-component-consumers`' file) and out of the disposition table,
-    // so it is recorded by identity — a second functional literal, anywhere, reds.
+    // (c) ONE pre-existing functional literal survived this story and was PINNED by
+    // identity rather than ignored: `Search.tsx`'s modal scrim fell back to a black
+    // at 0.45 alpha when `--color-overlay` was absent. It was out of THIS story's
+    // fence and out of the disposition table, and it was handed forward to the file's
+    // owner — DISCHARGED by `landing-and-component-consumers` (RK-S0216), which
+    // removed it as a dead branch (Radix declares `--color-overlay`, and the overlay
+    // renders inside `<Theme>`, so the fallback could never fire).
+    //
+    // The pin is therefore TIGHTENED, not deleted: zero functional literals under
+    // docs/src, so any new hand-authored value in any notation still reds here.
     expect(
       functional,
-      "the one pre-existing functional literal is Search.tsx's overlay fallback; a " +
-        "second one is a new hand-authored value and must be dispositioned, not absorbed",
-    ).toEqual(["docs/src/components/Search.tsx:129"]);
+      "no functional colour literal survives under docs/src — RK-S0214's pin, " +
+        "discharged by RK-S0216 and tightened to zero rather than dropped",
+    ).toEqual([]);
... TRUNCATED at 400 lines; 1 commits total
```
