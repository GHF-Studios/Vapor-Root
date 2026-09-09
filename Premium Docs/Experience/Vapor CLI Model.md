> [!info]
> **Owns:** semantic command-line grammar: command hierarchy, typed creation, generic existing-object operations, first-party bespoke nouns, Context/Selection syntax projection, and CLI diagnostics.
>
> **Uses:** Source Identity, Context/Selection, Operations, Role/Auth, Development models.
>
> **Does not own:** underlying acquisition, publication, Cargo, or runtime semantics.

---

# Core Principle

The CLI should express the work the user intends to perform in Vapor terms.

> **Use explicit semantic nouns where the noun itself matters, and generic operations where resolved target identity already supplies the missing type.**

Intentional asymmetry is desirable when it reflects real semantics.

---

# Command Shapes

## Generic System Domains

Stable cross-cutting concepts may own namespaces:

```text
installation
role
authority
toolchain
source-repo-container
source-repo
```

## Bespoke First-Party Facilities

Trusted facilities may own dedicated namespaces:

```text
client
platform-server
examples
```

They are semantic facilities, not directory aliases.

## Existing-Object Operations

For an existing modeled object:

```text
test <TARGET>
inspect <TARGET>
publish <TARGET>
```

is preferable to requiring:

```text
game test <TARGET>
library inspect <TARGET>
```

when identity already tells Vapor the kind.

---

# Typed Creation

Creation is different because the object does not exist.

Typed creation may use:

```text
packagepack create
enginepack create
gamepack create
modpack create

engine create
game create
engine-mod create
game-mod create
extension-mod create
library create
```

Creation supplies semantic information unavailable from a nonexistent target.

---

# Paths / Selectors

CLI targets should use Vapor Paths or safely resolvable selectors.

Full paths remain exact.

Shorter selectors are accepted when unambiguous.

The CLI must not invent alternate path syntax which skips arbitrary hierarchy segments.

---

# Context Projection

The CLI should expose persistent/transient Context deliberately.

Conceptually:

```text
--context <VAPOR-PATH>
```

may provide one-operation transient resolution.

Persistent Context may be established by explicit open/navigation commands.

CWD is not a substitute.

---

# Selection Projection

Operations supporting narrowing may expose:

```text
--select <SELECTION>
```

Selection is hierarchical.

Do not force redundant type flags when the selector topology already identifies the selected object kinds.

---

# Natural Subject

A command such as:

```text
platform-server test
```

has Platform Server as its natural complete subject.

No selection means "test Platform Server", not "run the same Cargo command once against every descendant."

The operation recipe defines realization.

---

# Source Topology Commands

Generic namespaces may expose approximately:

```text
source-repo-container create/acquire/status/open/close
source-repo create/acquire/status/open/close
```

The Source Development Model owns what those operations guarantee.

Independent Source Repo acquisition legality is modeled source policy, not a parser choice.

---

# Existing Project Operations

Possible generic operations:

```text
inspect <TARGET>
test <TARGET>
verify <TARGET>
repair <TARGET>
publish <TARGET>
```

Legality depends on resolved kind, subject, Role/authority, and operation semantics.

Not every verb applies to every target.

---

# Packagepack / App Operations

Packagepack-specific complete-App semantics may require operations such as:

```text
resolve
build
run
install
select
remove
publish
```

Exact final grammar may evolve as generic target operations mature.

---

# Managed Cargo Escape Hatch

An advanced passthrough may resemble:

```text
vapor toolchain cargo ... -- <cargo args>
```

It must route through managed Project/toolchain semantics rather than arbitrary ambient Cargo.

Explicit Cargo flags remain Cargo flags.

---

# First-Party Facilities

Useful shapes may include:

```text
client status/build/test
client deploy local
client deploy steam

platform-server status/build/test
platform-server deploy ...
```

Different variants may accept different Selection scopes.

---

# Ambiguity Diagnostics

When selectors are ambiguous, enumerate exact candidates.

Example:

```text
Project `core` is ambiguous within Platform Server

matches:
    Registry/core
    Identity/core
    Diagnostics/core
```

Then show safe stronger selectors and Context options.

---

# Missing Parent Diagnostics

Typed creation should teach exact missing topology rather than silently inventing it.

Example:

```text
Source Repo `GHF-Studios/My-Stuff/Game` does not exist.

help:
    source-repo create GHF-Studios/My-Stuff/Game
```

---

# CLI/Core Boundary

CLI parses syntax into semantic requests:

```text
resolved subject
resolved Context
resolved Selection
Operation Variant
parameters
```

Core does not depend on raw Clap syntax.

CLI does not duplicate business logic owned by Core.
