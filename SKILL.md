---
name: map
description: Interactive architectural cartography for codebases. Generates and maintains a living spatial map of the system (systems, connectors, protocols, and invariants) that updates as the code evolves.
triggers:
  - "/map"
  - "/map/init"
  - "/map/scan"
  - "/map/update"
  - "/map/serve"
---

# Map: Isomorphic System Cartography Skill

This skill maintains a living spatial map of the codebase inside `.map/`. It translates unfamiliar or complex codebases into an interactive, visual cartography representing:
- **Systems (LOD 0)**: Frontends, Backends, Databases, Workers, External APIs.
- **Connectors**: Protocols, Ports, Auth gates, and Request/Response contracts.
- **Subsystems (LOD 1)**: Internal bounded domains, routing topologies, middleware chains, ORMs, and state stores.
- **Invariants**: Security guarantees, transaction isolation, validation constraints, and source locations.

---

## Commands & Workflows

### 1. `/map/init` (or `/map/scan`)
Run a comprehensive discovery pass on the repository and initialize `.map/`.

**Steps to execute**:
1. **Reconnaissance & Deep Domain Discovery**:
   - **Frontend Discovery**:
     - Locate routing files (`routes/`, `pages/`, `app/`, `router.tsx`, `index.tsx`).
     - Map root shells and page hierarchy (e.g. `/`, `/dashboard`, `/settings/billing`, auth views).
     - Locate global state stores (`store/`, `state/`, `context/`, `zustand`, `redux`, `pinia`).
     - Locate networking clients (`apiClient.ts`, `services/api/`, fetch/axios wrappers).
   - **Backend Discovery**:
     - Identify controllers/routers (`routes/`, `controllers/`, `api/`, `@app.get`, `router.post`).
     - Map middleware chains (CORS, rate limiting, JWT token validation).
     - Map domain services (Auth, Billing, Notifications, Analytics).
     - Map database access layer (Repositories, Unit of Work, Prisma/SQLAlchemy models).
   - **Database & Queue Discovery**:
     - Schema definitions (`schema.prisma`, `models/`, `migrations/`, `schema.sql`).
     - Queues / brokers (`celery`, `bullmq`, `redis`, `kafka`).

2. **Synthesize `architecture.json`**:
   - Construct macro system nodes with sensible starting `position: { x, y }` coordinates.
   - Construct directional connectors between systems specifying protocol, auth mechanism, and exposed endpoints.
   - Build **rich multi-node subsystem graphs** inside `system.internal.nodes` and `system.internal.links` so that single-clicking into a system reveals its true internal routing, state, and pipeline topology.

3. **Deploy `.map/` companion**:
   - Ensure directory `.map/` exists in the target project root.
   - Look for the template assets in:
     - The map skill directory: `<skill_dir>/poc-visual-docs`
   - Copy `static/index.html` into `<project_root>/.map/static/index.html`.
   - Copy the prebuilt `serve` binary into `<project_root>/.map/serve` and ensure it is executable (`chmod +x .map/serve`).
   - Write `.map/architecture.json`.

4. **Launch or Report**:
   - Inform the user that the cartography is initialized at `.map/` and can be launched via `/map/serve`.

---

### 2. `/map/update`
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

### 3. `/map/serve`
Start the lightweight spatial viewer.

**Steps to execute**:
1. Check if `.map/serve` exists.
2. If it does not exist, build it from `<skill_dir>/poc-visual-docs` using `cargo build --release` or run directly.
3. Launch `./.map/serve` in background.
4. Open or report the URL: `http://127.0.0.1:4173`.

---

## Schema Guidelines for `architecture.json`
- **Root Object**:
  - `title`: String project name
  - `description`: String overview
  - `root.systems`: Array of macro components (`id`, `name`, `type`, `badge`, `icon`, `position`, `summary`, `metrics`, `internal`)
  - `root.connectors`: Array of communication pipes (`id`, `from`, `to`, `name`, `protocol`, `port`, `auth`, `summary`, `specs`, `endpoints`)
- **System Internal Graph (`internal`)**:
  - `title`: Subsystem title
  - `description`: Architectural summary
  - `nodes`: Array of internal component nodes (`id`, `name`, `type`, `badge`, `position: {x,y}`, `summary`, `details`)
  - `links`: Array of internal directed links (`from`, `to`, `label`)
- **Connector Endpoints**:
  - `method`: HTTP verb, SQL, RPC, EVENT
  - `path`: URL pattern or channel topic
  - `desc`: Plain explanation of purpose
  - `contract`: Input -> Output shape
