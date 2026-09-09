> [!info]
> **Owns:** source/version publication: Vapor ID + SemVer, immutable published versions, Git commit binding, Registry version records, provider release linkage, publication authority, Yank/Ban, and partial publication recovery.
>
> **Uses:** Source Identity, Role/Auth, Content/Composition.
>
> **Does not own:** Steam/Workshop built distribution, Cargo build realization, or source-editing workflow.

---

# Core Thesis

Source publication preserves immutable authored Vapor Content versions.

Published identity is:

```text
Vapor ID
+
SemVer
```

bound to exact remotely recoverable source.

---

# Publication Unit

For ordinary Vapor Content:

> **One Content Project is one independently identified/versioned/published Content unit.**

Content kind is Project metadata, not a second identity layer.

---

# Published Identity / Version Immutability

Once Content is published under a Vapor ID, ordinary publication does not casually rename that identity in place.

A published:

```text
Vapor ID @ Version
```

must never later mean different source.

Corrections require a new version.

Structural migration of already-published IDs is exceptional migration work which preserves historical identity truth.

---

# Local Development vs Published Version

Local development version intent may be mutable/pre-release.

Published version identity is immutable historical state.

---

# Git Commit Binding

Every published Content version binds to an exact Git commit.

The Registry must answer:

> Which exact authored source produced this version?

The commit must be remotely recoverable before publication is complete.

---

# Git vs Registry Authority

Git remains authoritative for:

```text
source files
history
branches
tags
repository state
```

Registry is authoritative for Vapor publication semantics such as:

```text
canonical Vapor identity
published versions
version → commit binding
provider linkage
Content kind
dependency metadata
Yank/Ban
publication state
authority/ownership
built-distribution linkage
```

---

# Provider Release Representation

A published version should receive an appropriate provider-side source release representation.

For GitHub-backed source this may include tag/Release/source archives.

Exact tag spelling is implementation detail.

Semantic requirement:

```text
published Vapor version
↔ exact provider source/commit
```

---

# Published Dependency Constraints

Published dependency requirements are immutable parts of the version.

Changing a requirement requires a new version.

Distinguish:

```text
authored constraint
```

from:

```text
exact resolution used by one built Packagepack
```

Exact built composition provenance belongs to Distribution/Composition.

---

# Publication Authority

Installed Role answers what the local environment can author.

Publication authorization answers whether this identity may publish this target/version.

Possible requirements include:

```text
Vapor namespace ownership
Registry permission
Git provider permission
Root Authority for protected first-party resources
future signing authority
```

---

# Publication Flow

Conceptually:

```text
validate Project
→ choose/validate SemVer
→ ensure version unused
→ validate dependencies
→ verify authority
→ ensure commit exists remotely
→ bind version to commit
→ create provider release representation
→ register immutable version metadata
```

Built Packagepack distribution may follow separately.

---

# Partial External Failure

Publication spans external systems and is not physically atomic.

Vapor must be able to represent/reconcile partial state such as:

```text
commit pushed
provider release created
Registry write pending
Registry complete
partial publication requiring repair
```

---

# Yank / Ban

**Yank** removes a version from ordinary future resolution/selection policy without rewriting historical truth.

**Ban** is a stronger administrative restriction controlled by appropriate authority.

Neither rewrites historical source/version meaning.

---

# Diagnostics

Distinguish:

```text
version already exists
source not pushed
provider unavailable
Registry unavailable
missing namespace authority
missing provider permission
invalid dependency metadata
partial prior publication
```

because remedies differ.
