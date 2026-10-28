---
description: Incrementally update .map/architecture.json against recent git diffs
---

Use the `map` skill to incrementally update the system cartography.
Inspect recent git changes (`git diff --name-status HEAD~1` or staged changes), identify modified contracts/endpoints/subsystems, and update `.map/architecture.json` in-place while strictly preserving existing node coordinates ($x, y$).
