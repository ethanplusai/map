# Map 🗺️

**Isomorphic architectural cartography for AI-native codebases.**

---

## The Thesis

Frontier AI models are shifting the primary bottleneck of software engineering. When humans no longer need to read, write, or comprehend every line of raw syntax, **source code ceases to be the primary human user interface**.

However, engineers still need to maintain an **isomorphic mental model** with the system reality:
- How are systems partitioned?
- Where are the boundaries, auth gates, and communication pipes?
- What invariants and contracts govern inter-service communication?
- How has the system topology evolved over the last $N$ commits?

`map` bridges this gap. It treats architecture documentation not as static diagrams that rot, but as a **living digital twin** co-compiled alongside the code.

---

## Features

- **Spatial Cartography (Overworld & Drilldowns)**:
  - Macro architecture view (Frontend, Services, Datastores, Message Queues, Gateways).
  - Multi-level Level-of-Detail (LOD): click into any system to inspect bounded internal domains, state stores, and middleware chains.
- **Fluid Drag-and-Drop Repositioning**:
  - Dynamically reposition components to organize spatial hierarchy and optimize visual clarity.
  - Connector curves and animated data flow pulses re-anchor in real time.
- **Deep Contract & Invariant Inspection**:
  - Inspect connection protocols, ports, auth guards, CORS policies, and rate limits.
  - View endpoint signatures, input/output schemas, and direct source file locations.
- **Self-Contained & Lightweight**:
  - Powered by a tiny precompiled Rust server (`axum` + `tokio`). Zero dependency footprint (`node_modules`-free).
- **Skill-Native Workflow**:
  - Plugs directly into agent workflows (`/map init`, `/map update`, `/map serve`).

---

## Quickstart

### 1. View the Proof of Concept
Run the standalone companion server locally:

```bash
cd poc-visual-docs
./serve
```

Open [http://127.0.0.1:4173](http://127.0.0.1:4173) in your browser.

---

## OpenCode Skill Integration

The skill definition is `SKILL.md` at the repository root, and the slash commands live in `commands/`. Running `./install.sh` copies the commands into `~/.opencode/commands`.

### Available Commands:
- `/map init` (or `/map scan`): Performs a full semantic discovery scan of your repository, infers systems, connectors, and invariants, and initializes `.map/`.
- `/map update`: Scans recent git diffs (`HEAD~1` or staged) and incrementally updates `.map/architecture.json` without shuffling existing spatial node coordinates.
- `/map serve`: Launches the local spatial viewer.
- `/map hook` (planned, see Roadmap): Will install git post-commit hooks to keep your architecture cartography updated with every commit.

### Using Globally in OpenCode
To use `/map` in any project on your machine, copy the skill and the viewer template it uses (`poc-visual-docs/`) to your global OpenCode skills directory. Run this from the repository root:

```bash
mkdir -p ~/.config/opencode/skills/map
cp -r SKILL.md poc-visual-docs ~/.config/opencode/skills/map/
```

Then in any project session:
```text
/map init
```

---

## Architecture Schema (`architecture.json`)

The single source of truth for the spatial viewer is a declarative JSON manifest:

```json
{
  "title": "Platform Architecture",
  "root": {
    "systems": [
      {
        "id": "sys-backend",
        "name": "Application Service",
        "type": "service",
        "badge": "FastAPI",
        "position": { "x": 580, "y": 260 },
        "summary": "Core API and transaction orchestration service.",
        "internal": { ... }
      }
    ],
    "connectors": [
      {
        "id": "conn-fe-be",
        "from": "sys-frontend",
        "to": "sys-backend",
        "protocol": "HTTPS (TLS 1.3)",
        "auth": "Bearer JWT",
        "endpoints": [ ... ]
      }
    ]
  }
}
```

---

## Roadmap

- [x] Hierarchical spatial cartography (Overworld + Subsystems).
- [x] Real-time interactive drag-and-drop node layout.
- [x] Rust-embedded single binary web server.
- [x] OpenCode `/map` skill integration.
- [ ] Dragged coordinate persistence back to `architecture.json`.
- [ ] Commit-diff Lottie micro-animations illustrating topological mutations.
- [ ] Bidirectional steering (modify canvas links to prompt agent refactors).
- [ ] Git commit hook automated pipeline.

---

## License

MIT
