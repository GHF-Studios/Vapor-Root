> [!info]
> **Owns:** Context, transient Context, Operation Subject interaction, Selection, selectors, minimal sufficient information, ambiguity, CWD separation, Open/Close, and frontend/session distinctions.
>
> **Uses:** Source Identity and Operations.
>
> **Does not own:** canonical hierarchy itself, source acquisition, CLI spelling, or operation state transitions.

---

# Core Thesis

Vapor should identify intended work with as little information as is safely sufficient without weakening exact canonical identity.

The core relationship is:

```text
context  →  operation subject  →  selection
  left            middle            right
```

Context supplies missing information to the left.

Selection supplies explicit scope intent to the right.

The final result is one or more exact modeled Vapor objects.

---

# Context

A **Context** is a lightweight semantic resolution prefix.

It is a convenience, not a requirement.

Given Context:

```text
GHF-Studios/Loo-Cast/Game
```

the selector:

```text
Loo-Cast
```

may resolve to:

```text
GHF-Studios/Loo-Cast/Game/Loo-Cast
```

Context does not create an alternative identity.

---

# Persistent Context

A frontend/session may remember one effective persistent Context.

It behaves like semantic navigation location.

It is not a durable working-set membership list.

Do not resurrect hidden target models such as:

```text
Open Workspaces set
Focused Projects set
Active Content set
```

---

# Transient Context

An operation may provide a one-shot Context override.

Conceptually:

```text
--context <VAPOR-PATH>
```

Exact frontend syntax belongs elsewhere.

Transient Context:

```text
exists only for the operation
does not mutate persistent Context
participates only in resolving relative selectors
```

---

# Open and Close

**Open** establishes/navigates persistent Context.

**Close** clears/navigates away from it.

Open does not:

```text
acquire
build
test
deploy
publish
change Git
select operation scope
```

Close does not delete source, forget Registry identity, uninstall, or discard authored work.

---

# Operation Subject

Every operation has an **Operation Subject**: the thing it is fundamentally about.

Examples:

```text
Vapor Client
Vapor Platform Server
one Source Repo Container
one Source Repo
one Project
one Packagepack
```

The subject establishes the operation's natural complete scope.

---

# Natural Complete Scope

No explicit Selection means:

> Perform the operation on its natural complete subject.

It does **not** mean a generic hidden `--all`.

For example:

```text
platform-server test
```

means "perform the Platform Server test operation."

The operation recipe determines what that entails.

---

# Selection

A **Selection** explicitly narrows/specifies topology beneath an Operation Subject where supported.

Example:

```text
subject:
    Vapor Platform Server

selection:
    Registry
```

Selection is per-operation intent.

It does not mutate persistent Context.

---

# Context Is Not Selection

Suppose Context is:

```text
GHF-Studios/Vapor-Platform-Server/Registry
```

Then:

```text
platform-server deploy vps
```

still targets the complete natural Platform Server deployment subject.

Context must not silently narrow the deployment to Registry.

> **Context affects resolution. Selection affects operation scope.**

---

# Hierarchical Selection

Selecting a parent topology node requests that meaningful operation scope and whatever descendants the operation defines as participating there.

A second generic `--all-projects` should not be necessary merely to express parent scope.

---

# Multiple Selection

Some operations may support multiple explicit selections.

Frontend syntax may vary.

Vapor Core should receive exact resolved objects, not raw CLI list syntax.

The operation decides whether a combination is legal.

---

# Minimal Sufficient Selector

Rule:

> **Accept the shortest selector that resolves unambiguously within the applicable resolution universe.**

Possible forms:

```text
full Vapor Path
Context-relative path
minimal unique selector
```

Shorthand never becomes canonical identity.

---

# Resolution Precedence

Conceptually:

```text
full exact path
    → use directly

relative selector + transient Context
    → resolve against transient Context

relative selector + persistent Context
    → resolve against persistent Context

selector uniquely resolvable from subject
    → use candidate

otherwise
    → unresolved / ambiguity diagnostic
```

---

# Ambiguity

Ambiguity is normal information.

Vapor must not guess.

Example:

```text
Project `core` is ambiguous within Platform Server

matches:
    Registry/core
    Identity/core
    Diagnostics/core
```

Diagnostics should enumerate known safe disambiguations.

---

# Current Working Directory

CWD is not ordinary Vapor Context.

A user may physically be in one repository while operating on another modeled object.

> **Filesystem location and Vapor semantic location are separate.**

Raw filesystem paths remain valid where physical location is genuinely the subject or when underlying Git/Cargo is used directly.

---

# Frontend Session State

GUI state may include:

```text
open editors
expanded nodes
tool windows
navigation history
selected rows
pinned views
```

This is Development Session state.

It is not canonical identity.

It is not Context unless deliberately projected into Context.

It is not Operation Selection until an operation deliberately consumes it.

---

# Resume State

Resume State may reconstruct useful frontend state after restart.

Reopening an editor must not silently make that Project the target of future build/deploy operations.

---

# Ownership Boundary

This model answers:

> **Which exact object(s) did the user/frontend mean?**

The Operations model answers what an operation may do with those resolved objects.
