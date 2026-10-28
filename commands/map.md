---
description: Interactive architectural cartography for codebases (init, update, serve)
---

Execute the `map` command with argument: $ARGUMENTS

If $ARGUMENTS is "init" or "scan", perform a full discovery pass and initialize .map/.
If $ARGUMENTS is "update", incrementally patch .map/architecture.json against recent git diffs.
If $ARGUMENTS is "serve", start the local cartography web server at http://127.0.0.1:4173.
If no argument is given, check if .map/ exists and suggest next steps.
