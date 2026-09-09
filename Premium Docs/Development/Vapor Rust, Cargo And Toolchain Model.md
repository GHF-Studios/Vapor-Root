> [!info]
> **Owns:** Vapor Project ↔ Rust/Cargo realization, Cargo inspection/reconciliation, generated realization, managed pinned Rust/Cargo, project-aware Cargo execution, rust-analyzer/tooling integration, build/test execution, and self-hosting toolchain semantics.
>
> **Uses:** Source Identity, Content/Composition, Installation.
>
> **Does not own:** Cargo's own semantics, source acquisition, publication, or runtime extension APIs.

---

# Core Thesis

Vapor should reuse Rust/Cargo semantics wherever they already fit.

> **Vapor determines semantic development/composition intent; Cargo remains authoritative for Rust package/build execution.**

---

# Vapor Topology vs Cargo Topology

Vapor:

```text
Container
→ Source Repo / Workspace
→ Project
```

Cargo may contain:

```text
workspace
→ package
→ crate/target
```

These may align but are not identical.

A Project may contain one or several packages/targets/supporting crates.

---

# Project as Execution Context

A Vapor Project is the modeled development unit.

Managed Cargo should resolve the appropriate Project and execute in its physical Rust context.

CWD alone must not determine Vapor semantic targeting, though Cargo itself still receives a filesystem working directory.

---

# Cargo Inspection

Vapor may inspect ordinary Cargo state:

```text
package name
manifest path
workspace membership
targets
dependency bindings
features
metadata
```

Cargo concepts remain Cargo concepts.

---

# Semantic Dependency vs Cargo Dependency

A Vapor dependency expresses semantic Project/Content intent.

A Cargo dependency expresses physical Rust package realization.

Therefore:

```text
Vapor semantic graph
≠
Cargo package graph
```

Vapor may verify/derive physical Cargo bindings from semantic requirements.

---

# Cargo Reconciliation

Proven useful states:

```text
semantic relationship
+ correct Cargo binding
→ Valid

semantic relationship
+ binding missing
→ Missing

semantic relationship
+ conflicting authored Cargo state
→ Conflict
```

Vapor may construct missing physical realization when unambiguous.

It must not overwrite conflicting developer-authored intent merely because Vapor prefers another state.

---

# Generated Cargo Realization

Complete compositions may generate derived Cargo realization:

```text
Packagepack
→ exact Vapor composition
→ generated Cargo package/graph
→ Cargo build
→ Vapor App
```

Generated realization is derived state.

Do not delete useful Cargo state such as lockfiles/target caches on every regeneration without reason.

---

# Static Composition

The intended model is statically composed Rust.

Prefer ordinary direct Rust calls/types and Cargo linking over runtime registries merely to implement source composition.

The Engine/Game/Mod ecosystem owns actual integration APIs.

---

# Managed Pinned Toolchain

Vapor should operate through a controlled toolchain rather than accidentally depending on ambient Rust.

A toolchain pin may include:

```text
channel
exact version
date
components
targets
```

A practical implementation may isolate:

```text
RUSTUP_HOME
CARGO_HOME
exact toolchain root
cargo
rustc
rust-analyzer
rust-src
rustfmt
clippy
```

"Vapor-managed pinned toolchain" is the precise architecture term unless Vapor actually ships toolchain payloads itself.

---

# Managed Execution Environment

Managed Cargo should explicitly establish relevant environment such as:

```text
CARGO_HOME
RUSTUP_HOME
RUSTUP_TOOLCHAIN
RUSTC
PATH
target directory
```

and neutralize incompatible ambient wrappers where necessary.

---

# Toolchain Install / Repair / Disposability

Installation should be repeatable:

```text
missing
→ install
→ verify
→ ready
```

Managed tooling is replaceable/derived.

A valuable stress test is:

```text
working environment
→ delete managed toolchain
→ status = missing
→ reinstall exact pin
→ build/test again
```

This improves both product DX and Vapor's own development discipline.

---

# Self-Hosting

Vapor should increasingly develop Vapor through its own managed environment:

```text
bootstrap Rust/Cargo
→ build initial Vapor
→ Vapor uses pinned managed toolchain
→ managed Vapor builds/tests Vapor
→ managed-built Vapor operates Vapor
```

Ambient Cargo may remain a minimal bootstrap bridge, not the ordinary first-party workflow.

---

# rust-analyzer / IDE Tooling

Vapor-managed development should consistently route language/tooling integration through the managed environment where appropriate:

```text
rust-analyzer
Cargo
rustfmt
clippy
```

GUI/editor UX belongs to SDK Experience.

Tool ownership belongs here.

---

# Build / Test

Semantic build/test may realize through Cargo plus authored operation steps:

```text
resolve subject/Project
→ establish managed toolchain
→ establish Cargo context
→ execute Cargo/other recipe steps
→ capture diagnostics/artifacts
```

---

# Incremental State

Preserve useful underlying incremental mechanisms when valid:

```text
Cargo.lock
target cache
compiler incremental state
generated realization directory
```

Regeneratable does not mean "delete every time".

---

# Error Transparency

Cargo/Rust failures should remain visible and attributable.

Vapor may add semantic context:

```text
which Project
which Vapor dependency
which generated binding
which toolchain pin
```

without hiding the underlying diagnostics.
