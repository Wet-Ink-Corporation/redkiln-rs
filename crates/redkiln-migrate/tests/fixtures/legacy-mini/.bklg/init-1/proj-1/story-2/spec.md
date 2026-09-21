---
id: story-2-spec
uid: 01JB8Q2R4T6V8X0Z2B4D6F8H5R
type: spec
slug: story-2-spec
title: Opaque carry
status: in-progress
process: bug-standard
stage: implement
template_sig: 9f2b1c4e8a0d6f37
rendered_sig: 4c7e0a91b3d85f26
---

## AC-002

GIVEN content with no schema home
WHEN it is lifted
THEN it is carried opaque with a marker.

## Diff prose

```diff
-    the old reconciler walked the tree twice
+    the new one walks it once and records what it saw
```

This section is narrative commentary on a change. No Phase-1 schema declares a field for it.
