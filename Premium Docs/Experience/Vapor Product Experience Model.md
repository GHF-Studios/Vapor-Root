> [!info]
> **Owns:** high-level UX/DX principles, progressive disclosure, role-specific journeys, golden paths, transparency, and how Vapor should feel as one coherent product.
>
> **Uses:** all focused architecture/development models.
>
> **Does not own:** their exact semantic definitions or CLI/SDK grammar.

---

# Product Thesis

Vapor exists to make a complex combination of:

```text
Rust
Cargo
Git
Git providers
Steam
SteamCMD
Workshop
Registry/Platform services
build tooling
Vapor Content
Vapor Apps
```

behave like one coherent product.

> **Expose the work the user intends to perform; orchestrate incidental infrastructure required to make that work possible.**

Vapor is neither a thin GUI over Cargo nor a black box replacing the underlying tools.

---

# Progressive Disclosure

Users should encounter complexity only when useful.

```text
Player
    Apps / play / install

Composer
    + Content
    + packs
    + composition
    + dependency resolution
    + builds

Content Developer
    + source Projects
    + Git
    + Rust/Cargo
    + tests
    + diagnostics

Ecosystem Developer
    + first-party Client/Platform source
    + infrastructure
    + deployment
    + Registry/provider internals
```

Higher Role should feel substantially more capable, not merely reveal a few buttons.

---

# Player Experience

The Player begins in Steam.

Playing default Loo Cast must not require:

```text
Git
Rust
Cargo
SteamCMD
source repositories
local compilation
manual dependency resolution
```

Primary Player tasks:

```text
install
play default Loo Cast
discover complete Apps
acquire/install App
select App
launch
remove
settings/accounts
```

---

# Composer Experience

A Composer reasons mainly about:

```text
Content
packs
dependencies
compatibility
Packagepack
resolution
build
Vapor App
publication
```

They should be able to say:

```text
Build this Packagepack.
```

without manually reproducing toolchain/Cargo/source orchestration.

---

# Content Developer Experience

A Content Developer legitimately encounters:

```text
source topology
Projects
Git
Rust/Cargo
implementation source
tests
build/run
diagnostics
SDK
publication
```

Vapor should add semantic coherence without hiding normal development tools.

---

# Ecosystem Developer Experience

Expected first-party loop:

```text
acquire/recognize first-party source
→ edit
→ managed build/test
→ inspect
→ commit/push where authorized
→ local deploy
→ official deploy where authorized
```

Protected authority must not be required for local build/test.

---

# Golden Paths

Common tasks should have strong normal workflows:

```text
Create this Game.
Acquire this Container.
Test this Project.
Build this Packagepack.
Run this App.
Develop Vapor Client.
Deploy Platform Server.
Publish this Library.
```

Users should not manually coordinate every underlying tool merely to state those intentions.

---

# Minimal Sufficient Input

Do not force users to restate information Vapor already knows.

Examples:

```text
existing Project kind derives from resolved identity
short unique selector accepted
canonical acquisition destination already known
```

Ambiguity must never be guessed.

---

# Advanced Transparency

Abstraction must not require opacity.

As users become more advanced, expose:

```text
canonical identities
Git state
provider linkage
commit hashes
Cargo packages/targets/output
toolchain state
resolved dependencies
generated realization
build logs
operation recipes
Steam/provider identifiers
deployment output
```

A user should be able to understand what Vapor did.

---

# Diagnostics Are Product UX

Errors should explain:

```text
what resolved
what failed
which candidates exist
which invariant failed
which state is missing
which Role is required
which authorization is missing
which safe operation can resolve it
```

Diagnostics should teach the model rather than emit generic failure codes.

---

# Underlying Tools Stay Real

Where Git/Cargo/Rust/Steam already have meaningful concepts, preserve them.

```text
Git branch stays Git branch
Cargo package stays Cargo package
Workshop Item ID stays provider identity
```

Vapor adds semantic context rather than weaker replacements.

---

# Recovery Experience

Users should learn a simple rule:

```text
replaceable/derived state
    can be reconstructed

authored source
    is preserved
```

Machine/setup disposability should be confidence-building, not destructive.

---

# Surface Consistency

Launcher, SDK, CLI, Installer, and automation project the same Core semantics.

They may differ in interaction style.

They must not disagree about what a Project, Context, Role, dependency, or Operation means.

---

# Stable Facility Vocabulary

First-party work should be described through:

```text
Client
Platform Server
Examples
```

rather than temporary repository names.
