# Vapor server-root topology planning capture

- Source date: 2026-07-23
- Source type: raw owner/assistant planning capture
- Context: follow-up to the diagnostics/registry/server planning discussion
  after the owner clarified the desired server-side repository and service
  separation.
- Planning status: intake only; unprocessed; not active doctrine.
- Implementation status: no server implementation is accepted by this artifact.

## Purpose

Persist the current intended Vapor server topology before implementation starts.
The owner wants the web/server side to be treated as a different deployment and
operations domain from `Vapor-Root`, while still keeping the pieces
reproducible and open-source-friendly.

## Repository topology

The intended root repository is:

```text
Vapor-Server-Root
```

It should live beside `Vapor-Root`, not inside `Vapor-Root` and not as a
submodule of `Vapor-Root`.

The intended service repositories are:

```text
Vapor-Homepage-Server
Vapor-Docs-Server
Vapor-Identity-Server
Vapor-Diagnostics-Server
```

The intended relationship is:

```text
Vapor-Server-Root/
  services/
    homepage/      -> submodule Vapor-Homepage-Server
    docs/          -> submodule Vapor-Docs-Server
    identity/      -> submodule Vapor-Identity-Server
    diagnostics/   -> submodule Vapor-Diagnostics-Server

  deploy/
    caddy/
    systemd/
    scripts/

  crates/
    vapor_server_shared/   # optional, only when useful
```

The owner created the five repositories before local clone/submodule setup.
This artifact records the intended topology by repository name, not by local
checkout path.

## Service separation

The owner wants strict separation between homepage, docs, identity, and
diagnostics. These should be separate repositories and separate server programs
or service binaries, even if they deploy together.

### Vapor-Homepage-Server

Responsibility:

- public homepage;
- product/landing page;
- license page;
- Impressum/legal contact page;
- public links to docs and other services.

Default state model:

- mostly stateless;
- rebuildable independently.

### Vapor-Docs-Server

Responsibility:

- serve docs;
- serve current docs;
- serve versioned docs;
- accept docs publication from root/deploy tooling;
- rebuild/publish docs independently.

State model:

- uploaded/generated docs artifacts;
- docs version/current pointer metadata.

### Vapor-Identity-Server

Responsibility:

- Steam identity;
- GitHub identity;
- linked Vapor developer profile;
- role assignment;
- authentication/session/JWT issuance;
- root/developer authorization source.

Identity policy direction:

- players should not require GitHub accounts;
- developers/content creators should require both Steam and GitHub identities;
- root developers should require Steam and GitHub identities plus an assigned
  root role.

State model:

- identity registry;
- linked accounts;
- roles;
- auth/session metadata as required.

### Vapor-Diagnostics-Server

Responsibility:

- opt-in diagnostics/log upload;
- rough non-identifying system specs with diagnostics;
- diagnostics listing/download for authorized root developers;
- diagnostics export/import;
- upload limits;
- retention policy.

Diagnostics policy direction:

- uploads are explicit opt-in;
- Git is not used as diagnostics transport;
- normal players do not need GitHub for diagnostics;
- hostname should not be captured;
- persistent machine id should not be captured;
- rough system specs are acceptable.

State model:

- uploaded diagnostics bundles;
- diagnostics index;
- export/import metadata.

## Single-domain routing

The owner does not want many domains. The preferred deployment shape is one
domain with path routing.

Candidate routing:

```text
/                         -> Vapor-Homepage-Server
/docs/                    -> Vapor-Docs-Server
/api/identity/            -> Vapor-Identity-Server
/api/diagnostics/         -> Vapor-Diagnostics-Server
```

Caddy or a similar reverse proxy can be the public routing layer.

No separate API/bundling service is required yet. A future aggregation layer
may be considered only if it has a concrete job beyond reverse proxy routing.

## Root orchestration responsibility

`Vapor-Server-Root` should own server deployment and operational orchestration.

Candidate responsibilities:

- clone/update submodules;
- build one service;
- deploy one service;
- deploy all services;
- initialize a fresh server;
- export the whole server state;
- import the whole server state;
- rebuild one service;
- rebuild the full service conglomerate;
- install and maintain Caddy/reverse-proxy config;
- install and maintain systemd unit templates;
- hold reproducibility/deployment documentation.

The root repo owns deployment shape, not service business logic.

## Rebuild and restore model

Each stateful service should eventually support its own export/import.

Composed full export:

```text
Vapor-Server-Root export
  calls:
    docs export
    identity export
    diagnostics export

  produces:
    vapor-server-export-YYYYMMDD.tar.zst
      manifest.toml
      docs/
      identity/
      diagnostics/
```

Composed full restore:

```text
Vapor-Server-Root init --from vapor-server-export-YYYYMMDD.tar.zst
  restores:
    docs state
    identity state
    diagnostics state
```

The whole service conglomerate should be reproducible from source plus either:

- explicit empty initialization; or
- initialization from a previously exported state bundle.

## Vapor Shell role

It is acceptable for Vapor Shell to become a higher-level operator UX later.
That should make it a wrapper over server-root tooling and/or HTTP APIs rather
than the owner of the server implementation.

Possible future commands:

```text
vapor server status
vapor server deploy homepage
vapor server deploy docs
vapor server deploy identity
vapor server deploy diagnostics
vapor server deploy all
vapor server export
vapor server init --from <archive>
```

Vapor Shell should not hide authority boundaries. Deployment, export, import,
identity initialization, and diagnostics access remain privileged operations.

## Optional shared code

A shared Rust crate or shared local library is allowed if duplication appears.
It should not be introduced before it has a concrete job.

Good candidates:

- config loading;
- response/error envelope helpers;
- build/version metadata helpers;
- archive/export format helpers;
- auth token primitives once identity design is clear.

Bad candidates:

- service-specific database schema;
- service business logic;
- coupling service release cadence unnecessarily.

## Future MCP/ACP idea

Future idea, not for the current server MVP:

Vapor could expose some server/admin capabilities through MCP or a similar
agent-facing protocol.

Current understanding to preserve:

- An MCP server can add tools/resources/prompts/capabilities for agents without
  owning the whole model context.
- Building an MCP server does not require Vapor to implement a full model,
  context window, or agent runtime.
- MCP should be treated as a capability surface over existing Vapor/server
  authority, not as the authority model itself.
- Any future MCP/ACP surface must respect the same authorization boundaries as
  direct REST/HTTP/server-root tooling.

Possible future MCP surfaces:

- read docs resources;
- inspect deployment status;
- list diagnostics metadata for authorized root users;
- fetch a specific diagnostics bundle for authorized root users;
- trigger safe deployment preflights;
- expose server-root runbooks as resources.

Explicit non-goals for now:

- no MCP implementation today;
- no ACP implementation today;
- no agent-specific auth design today;
- no replacement of REST/HTTP APIs with MCP.

## Immediate next planning step

Use this topology to decide the first server-root scaffold shape before writing
service code. Keep the first scaffold focused on repository layout,
deployment/readme conventions, and path-routed service boundaries.
