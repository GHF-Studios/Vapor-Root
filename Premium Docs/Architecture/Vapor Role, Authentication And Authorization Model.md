> [!info]
> **Owns:** installed Vapor Roles, authentication, authorization, Root Authority, first-party trust, and separation between local capability and remote permission.
>
> **Uses:** Product Topology and Operations.
>
> **Does not own:** exact Installer workflow, CLI spelling, provider login UI, or publication transport.

---

# Core Thesis

Vapor distinguishes:

```text
what this local environment is equipped to do
```

from:

```text
who the user is
```

and from:

```text
what that identity may do to a protected target
```

These are different dimensions.

---

# Installed Roles

Roles are cumulative:

```text
Player
⊂ Composer
⊂ Content Developer
⊂ Ecosystem Developer
```

## Player

Consumes finished Vapor Apps.

Player operation must not require source-development tooling such as Git, Rust, Cargo, or SteamCMD.

## Composer

May author composition-oriented Content:

```text
Packagepack
Enginepack
Gamepack
Modpack
```

using existing behavioral/reusable Content.

## Content Developer

May additionally author:

```text
Engine
Game
Engine Mod
Game Mod
Extension Mod
Library
```

and therefore requires richer source/Rust/Cargo development capability.

## Ecosystem Developer

May develop Vapor itself: Client, Platform, Registry, tooling, distribution, diagnostics, and other first-party infrastructure.

Ecosystem Developer is locally attainable and does not itself grant protected official authority.

---

# Role Meaning

A **Vapor Role** answers:

> What kinds of work is this local Vapor environment equipped to perform?

Role may affect installed tools, visible surfaces, local workflows, diagnostics, and authoring capability.

Role is not an account permission.

---

# Authentication

**Authentication** answers:

> Who is attempting this operation?

Authentication may involve Steam, Git providers, or Vapor Platform identity.

Local edit/build/test/run workflows should not require remote authentication merely because remote operations exist elsewhere.

---

# Authorization

**Authorization** answers:

> May this authenticated identity perform this protected operation against this target?

Examples:

```text
push to provider repo
publish this Vapor identity/version
administer this Registry namespace
deploy this Steam branch/depot
deploy production Platform infrastructure
```

Authorization is operation- and target-specific.

---

# Root Authority

**Root Authority** is ultimate trusted administrative authority over protected official Vapor resources.

It may govern:

```text
official namespace ownership
first-party classification
protected repositories
Registry administration
official publication/distribution
production Platform infrastructure
protected identity migrations
```

Root Authority is not another installed Role.

Wrong:

```text
Player → Composer → Content Developer → Ecosystem Developer → Root Authority
```

Correct:

```text
local Role:
    Ecosystem Developer

independently:
    authenticated identity
    possibly Root Authority
```

---

# Local Capability Without Remote Permission

A valid state may be:

```text
Content Developer Role
managed toolchain ready
source available
Git provider logged out
```

The user may still:

```text
edit
resolve
build
test
run
inspect
```

Only operations whose semantics require remote authority should fail.

---

# First-Party Trust

First-party classification and facility binding are trusted Vapor/Registry state.

Untrusted source must not make itself first-party merely by declaring a local flag.

---

# Provider Permission Is Not Vapor Identity

A Vapor Authority may link to a provider organization/user, but:

```text
Vapor Authority
≠ provider organization

Vapor authorization
≠ provider ACL
```

They interact without being the same identity system.

---

# Role Transition Safety

Promotion/demotion changes local capability.

It must not silently modify unrelated authored state.

In particular, downgrade does not grant permission to delete source, manifests, local commits, or unpushed work.

---

# Diagnostic Principle

Failures should distinguish:

```text
missing local Role/capability
missing authentication
missing target authorization
provider unavailable
```

Prefer specific diagnostics over generic `permission denied`.
