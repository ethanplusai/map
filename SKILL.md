---
name: map
description: Interactive architectural cartography for codebases. Generates and maintains a living spatial map of the system (systems, connectors, protocols, and invariants) that updates as the code evolves.
triggers:
  - "/map"
  - "/map init"
  - "/map scan"
  - "/map update"
  - "/map serve"
  - "/map hook"
---

# Map: Isomorphic System Cartography Skill

This skill maintains a living spatial map of the codebase inside `.map/`. It translates unfamiliar or complex codebases into an interactive, visual cartography representing:
- **Systems (LOD 0)**: Frontends, Backends, Databases, Workers, External APIs.
- **Connectors**: Protocols, Ports, Auth gates, and Request/Response contracts.
- **Subsystems (LOD 1)**: Internal bounded domains, middleware chains, ORMs, and state stores.
- **Invariants**: Security guarantees, transaction isolation, validation constraints, and source locations.

---

## Commands & Workflows

### 1. `/map init` (or `/map scan`)
Run a comprehensive discovery pass on the repository and initialize `.map/`.

**Steps to execute**:
1. **Reconnaissance**:
   - Locate package descriptors (`package.json`, `Cargo.toml`, `pyproject.toml`, `go.mod`, `pom.xml`, etc.).
   - Locate infrastructure/orchestration (`docker-compose.yml`, `Dockerfile`, Kubernetes configs).
   - Locate datastores & schemas (`schema.prisma`, `migrations/`, `alembic/`, `schema.sql`, etc.).
   - Locate route boundaries (`routes/`, `controllers/`, `api/`, `openapi.yaml`, etc.).
2. **Synthesize `architecture.json`**:
   - Construct macro system nodes with sensible starting `position: { x, y }` coordinates.
   - Construct directional connectors between systems specifying protocol, auth mechanism, and exposed endpoints.
   - Detail internal subsystems for each major component.
3. **Deploy `.map/` companion**:
   - Ensure directory `.map/` exists in the target project root.
   - Look for the template assets in:
     - The map skill directory: `<skill_dir>/poc-visual-docs` or `<skill_dir>/template`
   - Copy `static/index.html` into `<project_root>/.map/static/index.html`.
   - Copy the prebuilt `serve` binary into `<project_root>/.map/serve` and ensure it is executable (`chmod +x .map/serve`).
   - Write `.map/architecture.json`.
4. **Offer Hook Setup**:
   - Inform the user that the cartography is ready at `.map/` and can be served via `/map serve` or `./.map/serve`.
   - Ask if they want to install the git post-commit hook via `/map hook`.

---

### 2. `/map update`
Perform a fast, targeted incremental update against recent git diffs.

**Steps to execute**:
1. Check changed files via `git status --porcelain` or `git diff --name-status HEAD~1`.
2. Read the existing `.map/architecture.json`.
3. Identify which systems, routes, or subsystems are impacted by the diff.
4. **Crucial Rule: Preserve Spatial Anchors**:
   - Keep existing `x, y` positions for all unchanged nodes so the user's mental map remains stable.
   - Update contracts, endpoints, and invariants that changed.
   - Add new nodes near related subsystems if new components were introduced.
5. Save the updated `.map/architecture.json`.

---

### 3. `/map serve`
Start the lightweight spatial viewer.

**Steps to execute**:
1. Check if `.map/serve` exists.
2. If it does not exist, build it from `.opencode/skills/map/template` using `cargo build --release` or run directly.
3. Launch `./.map/serve` in background or provide the command to the user.
4. Open or report the URL: `http://127.0.0.1:4173`.

---

### 4. `/map hook`
Installs a git hook in `.git/hooks/post-commit` that warns or triggers an architecture map sync on every commit.

---

## Schema Guidelines for `architecture.json`
- **Root Object**:
  - `title`: String project name
  - `description`: String overview
  - `root.systems`: Array of macro components (`id`, `name`, `type`, `badge`, `icon`, `position`, `summary`, `metrics`, `internal`)
  - `root.connectors`: Array of communication pipes (`id`, `from`, `to`, `name`, `protocol`, `port`, `auth`, `summary`, `specs`, `endpoints`)
- **Connector Endpoints**:
  - `method`: HTTP verb, SQL, RPC, EVENT
  - `path`: URL pattern or channel topic
  - `desc`: Plain explanation of purpose
  - `contract`: Input -> Output shape
- **Subsystem Nodes**:
  - `id`, `name`, `type`, `badge`, `position`, `summary`, `details.sourceLocation`, `details.invariants`
