> [!info]  
> This document defines the intended **User Experience (UX)** and **Developer Experience (DX)** of the Vapor Ecosystem.
> 
> It sits between the **Ecosystem Model Glossary** and lower-level system specifications / architecture / TDDs.
> 
> The goal is not to determine implementation details yet.
> 
> Answer primarily in terms of:
> 
> - What the user wants to accomplish.
>     
> - What the user sees and does.
>     
> - What Vapor does in response.
>     
> - What state changes as a result.
>     
> - What should happen when things go wrong.
>     
> 
> Messy answers are fine. Unknowns may simply be marked `TODO`, `UNKNOWN`, or `UNDECIDED`.

---

# 1. Actors

## 1.1 Actor Types

### What kinds of people interact with Vapor?

Known possibilities:

- Player
    
- Mod user
    
- Content developer
    
- Game developer
    
- Engine developer
    
- Vapor ecosystem/toolchain developer
    
- Server/operator
    
- Other?
    

**Answer:**

---

## 1.2 Player

### What is a Player primarily trying to accomplish?

**Answer:**

### What knowledge can Vapor assume from a Player?

**Answer:**

### What parts of Vapor should a Player normally encounter?

**Answer:**

### What parts should normally remain invisible to them?

**Answer:**

---

## 1.3 Mod User

### What distinguishes a Mod User from a normal Player?

**Answer:**

### What should a Mod User be able to accomplish?

**Answer:**

### What technical knowledge can Vapor assume from them?

**Answer:**

---

## 1.4 Content Developer

### What should a Content Developer be able to accomplish?

**Answer:**

### What technical knowledge can Vapor assume from them?

**Answer:**

### What should Vapor automate for them?

**Answer:**

### What should remain directly controllable by them?

**Answer:**

---

## 1.5 Engine / Game Developer

### Is an Engine/Game Developer meaningfully different from a normal Content Developer?

**Answer:**

### If yes, how?

**Answer:**

---

## 1.6 Vapor Ecosystem Developer

### What distinguishes someone developing Vapor itself from someone developing Vapor Content?

**Answer:**

### What additional capabilities or workflows do they need?

**Answer:**

---

## 1.7 Server / Registry Operator

### Is operating Vapor servers/registries a meaningful user role?

**Answer:**

### If yes, what should that role be able to accomplish?

**Answer:**

---

# 2. First Installation and First Launch

Assume:

> Clean machine → Steam → Install Loo Cast → Launch

## 2.1 Installation

### What does Steam install initially?

**Answer:**

### What is bundled directly with the Steam App?

**Answer:**

### What is intentionally downloaded or installed later?

**Answer:**

---

## 2.2 First Launch

### What executable/process starts first?

**Answer:**

### What does the user see first?

**Answer:**

### Is there any first-time setup?

**Answer:**

### What does Vapor automatically detect on the machine?

**Answer:**

### What does Vapor automatically configure?

**Answer:**

---

## 2.3 Default Composition

### What is the default App Composition?

**Answer:**

### How is that default composition selected/resolved?

**Answer:**

### Is the default content already installed?

**Answer:**

### If not, what happens?

**Answer:**

---

## 2.4 Starting the Game

### What does the Player actually do to start playing?

**Answer:**

### What happens between pressing "Play" and the Engine/Game starting?

**Answer:**

### Which parts of this process should remain invisible during normal operation?

**Answer:**

---

## 2.5 Subsequent Launches

### What changes after the first successful launch?

**Answer:**

### What state should Vapor remember?

**Answer:**

### Should subsequent launches normally go directly into the Game, through the Launcher, or somewhere else?

**Answer:**

---

# 3. App Structure and User-Facing Surfaces

## 3.1 The App

### Conceptually, what is the "Loo Cast" App?

**Answer:**

### Is the Launcher a separate application, a mode, a window, or merely a conceptual role?

**Answer:**

### Is the SDK a separate application, a mode, a collection of tools, or merely a conceptual role?

**Answer:**

### How do Launcher, SDK, Game, and ecosystem tooling relate from the user's perspective?

**Answer:**

---

## 3.2 Major Surfaces

### What major user-facing surfaces should exist?

Possible examples:

- Play
    
- Content / Library
    
- Compositions
    
- Development
    
- Projects
    
- Workshop
    
- Servers
    
- Settings
    
- Diagnostics
    
- Toolchain
    
- Logs
    
- Accounts
    

**Answer:**

### What is the normal "home" surface?

**Answer:**

### Which surfaces should ordinary Players never need?

**Answer:**

### Which surfaces appear only in Developer Mode?

**Answer:**

---

## 3.3 Navigation

### How should users move between the major parts of Vapor?

**Answer:**

### Should the distinction between Player and Developer workflows be visually strong or mostly seamless?

**Answer:**

---

## 3.4 Command-Line Experience

### Should Vapor expose a CLI?

**Answer:**

### If yes, who is it intended for?

**Answer:**

### Should GUI and CLI expose roughly equivalent capabilities?

**Answer:**

### Which workflows should deliberately remain CLI-first or GUI-first?

**Answer:**

---

# 4. App Instances

## 4.1 Creation

### How is an App Instance created?

**Answer:**

### Is the Steam installation automatically one App Instance?

**Answer:**

### Can users manually create additional App Instances?

**Answer:**

### Why would someone want multiple App Instances?

**Answer:**

---

## 4.2 Identity

### How is an App Instance identified?

**Answer:**

### Does it have a human-readable name?

**Answer:**

### Does it have a stable internal ID?

**Answer:**

### Is its filesystem path part of its identity?

**Answer:**

---

## 4.3 Location

### Where may an App Instance live?

**Answer:**

### Can it be moved?

**Answer:**

### What happens when the user manually moves its directory?

**Answer:**

---

## 4.4 Instance State

### What state belongs specifically to an App Instance?

**Answer:**

### What state is shared globally between App Instances?

**Answer:**

### What installed content belongs to an individual instance versus a shared cache/library?

**Answer:**

---

## 4.5 Instance Lifecycle

### How is an App Instance repaired?

**Answer:**

### How is it reset?

**Answer:**

### How is it deleted?

**Answer:**

### What should happen to user-generated/development data when an instance is deleted?

**Answer:**

---

## 4.6 External Modification

### What happens when App Instance files are changed outside Vapor?

**Answer:**

### Should Vapor detect this?

**Answer:**

### Should external changes be tolerated, adopted, repaired, rejected, or handled case-by-case?

**Answer:**

---

# 5. Developer Mode

## 5.1 Meaning

### What does "Developer Mode" actually mean?

**Answer:**

### Is Developer Mode a property of the App Instance, the user, or both?

**Answer:**

---

## 5.2 Upgrade

### How does a normal App Instance become a Developer Mode App Instance?

**Answer:**

### What does the user explicitly do?

**Answer:**

### What does Vapor do automatically?

**Answer:**

### What new files/state/tools are introduced?

**Answer:**

---

## 5.3 Developer Dependencies

For each tool, describe whether Vapor:

- Requires it.
    
- Detects it.
    
- Installs it.
    
- Updates it.
    
- Configures it.
    
- Allows custom installations.
    

### Git

**Answer:**

### Rust / Cargo

**Answer:**

### SteamCMD

**Answer:**

### Steam Client

**Answer:**

### IDE / Editor

**Answer:**

### Other tooling

**Answer:**

---

## 5.4 Ready State

### What exactly must be true before Vapor considers Developer Mode "ready"?

**Answer:**

### How does the user know that it is ready?

**Answer:**

### How are partially configured/broken developer environments represented?

**Answer:**

---

## 5.5 Downgrade

### Can Developer Mode be disabled/downgraded?

**Answer:**

### What gets removed?

**Answer:**

### What is preserved?

**Answer:**

### How does Vapor avoid destroying development work?

**Answer:**

---

# 6. App Compositions

## 6.1 Selection

### How does the user select the active App Composition?

**Answer:**

### Is exactly one composition always selected?

**Answer:**

### Can Vapor have no valid selected composition?

**Answer:**

---

## 6.2 Composition Representation

### Does a composition itself exist as a named/persistent object?

**Answer:**

### Can users create their own compositions?

**Answer:**

### Can they name them?

**Answer:**

### Can they duplicate/clone them?

**Answer:**

### Can they export/share them?

**Answer:**

---

## 6.3 Modification

### Can a user directly modify an existing composition?

**Answer:**

### Can they change its Packagepack?

**Answer:**

### Can they add/remove Engine Mods, Game Mods, or Extension Mods?

**Answer:**

### When does a modification create a new composition versus mutate an existing one?

**Answer:**

---

## 6.4 Validity

### What makes a composition valid?

**Answer:**

### When should Vapor refuse to launch one?

**Answer:**

### What should happen when a previously valid composition becomes invalid?

**Answer:**

---

# 7. Content States

For Vapor Content, define the user-facing meaning of the following states.

## 7.1 Known / Discoverable

### What does it mean for content to be known to Vapor?

**Answer:**

---

## 7.2 Available

### What does "available" mean?

**Answer:**

---

## 7.3 Downloaded

### What does "downloaded" mean?

**Answer:**

---

## 7.4 Installed

### What does "installed" mean?

**Answer:**

---

## 7.5 Mounted / Active

### What does "mounted" or "active" mean?

**Answer:**

---

## 7.6 Development Content

### How is local development content represented differently from installed/released content?

**Answer:**

---

## 7.7 Broken / Invalid

### What does Vapor consider broken or invalid content?

**Answer:**

### How is that presented to the user?

**Answer:**

---

# 8. Content Discovery

## 8.1 Sources

### Where can Vapor Content come from?

Consider:

- Steam Workshop
    
- Vapor Content Registry
    
- Local filesystem
    
- Development Workspaces
    
- Git repositories
    
- Direct IDs
    
- URLs
    
- Other sources
    

**Answer:**

---

## 8.2 Browsing

### How does a user browse available content?

**Answer:**

### What search/filtering capabilities should exist?

**Answer:**

---

## 8.3 Content Information

### What information should a user see before installing content?

Possible information:

- Name
    
- Human-readable ID
    
- Author
    
- Description
    
- Type
    
- Version
    
- Engine/Game target
    
- Dependencies
    
- Compatibility
    
- Workshop page
    
- Size
    
- Trust/signature status
    

**Answer:**

---

## 8.4 Direct References

### Can users install/open content directly from a Vapor ID?

**Answer:**

### From a Workshop ID?

**Answer:**

### From a URL?

**Answer:**

---

# 9. Content Installation and Dependency Resolution

## 9.1 Installation

### What does the user do to install content?

**Answer:**

### What does Vapor do afterward?

**Answer:**

---

## 9.2 Dependency Resolution

### How are dependencies resolved?

**Answer:**

### Are dependencies installed automatically?

**Answer:**

### When should Vapor ask the user first?

**Answer:**

---

## 9.3 Conflicts

### What kinds of dependency conflicts can occur?

**Answer:**

### Which conflicts can Vapor resolve automatically?

**Answer:**

### Which require user intervention?

**Answer:**

---

## 9.4 Compatibility

### How does Vapor determine compatibility?

**Answer:**

### What happens when content is technically installable but incompatible with the selected composition?

**Answer:**

---

## 9.5 Removal

### How does content get uninstalled?

**Answer:**

### What if other installed content depends on it?

**Answer:**

---

# 10. Creating Vapor Content

Answer the following separately where behavior differs between content types.

Content types:

- Packagepack
    
- Enginepack
    
- Gamepack
    
- Modpack
    
- Engine
    
- Game
    
- Engine Mod
    
- Game Mod
    
- Extension Mod
    

---

## 10.1 Entry Point

### How does the user begin creating new Vapor Content?

**Answer:**

### From where in the App/SDK?

**Answer:**

### Can creation also begin from the CLI?

**Answer:**

---

## 10.2 Content Type Selection

### How does the developer choose what kind of content they are creating?

**Answer:**

### What explanation does Vapor provide for each content type?

**Answer:**

---

## 10.3 Identity

### What must the developer name or identify?

**Answer:**

### Is a human-readable Vapor ID chosen immediately?

**Answer:**

### Is an internal ID also generated?

**Answer:**

### What naming restrictions exist at the UX level?

**Answer:**

---

## 10.4 Targets and Dependencies

### When creating an Engine Mod, how is its Engine selected?

**Answer:**

### When creating a Game Mod, how is its Game selected?

**Answer:**

### When creating an Extension Mod, how is its target selected?

**Answer:**

### How are initial dependencies selected?

**Answer:**

---

## 10.5 Repository / Workspace Placement

### Does the developer choose an existing Source Repo / Vapor Workspace or create a new one?

**Answer:**

### When is a new Container Repo created?

**Answer:**

### When is a new Source Repo / Vapor Workspace created?

**Answer:**

### When is only a new Vapor Project created?

**Answer:**

---

## 10.6 Generated Result

### What should exist after successful creation?

**Answer:**

### What files/directories should the developer immediately recognize?

**Answer:**

### What should Vapor open/show afterward?

**Answer:**

### Should the developer be able to run the new content immediately?

**Answer:**

---

# 11. Vapor Superworkspaces

## 11.1 Purpose

### What problem does a Vapor Superworkspace solve from the developer's perspective?

**Answer:**

### When should a developer need to know that a Superworkspace exists?

**Answer:**

---

## 11.2 Creation

### When is a Superworkspace created?

**Answer:**

### Is creation explicit or automatic?

**Answer:**

### Where does it live?

**Answer:**

---

## 11.3 Contents

### What gets checked out into a Superworkspace?

**Answer:**

### How are Container Repos represented inside it?

**Answer:**

### How are Source Repos / Vapor Workspaces represented?

**Answer:**

---

## 11.4 Disposable Nature

### What does "disposable" mean operationally?

**Answer:**

### What data may safely be regenerated?

**Answer:**

### What local state can still be lost if the Superworkspace is deleted?

**Answer:**

### How should Vapor warn about such state?

**Answer:**

---

# 12. Repository and Workspace Experience

## 12.1 Existing Repositories

### Can Vapor adopt an existing Container Repo?

**Answer:**

### Can Vapor adopt an existing Source Repo / Vapor Workspace?

**Answer:**

### Can Vapor adopt an existing Cargo workspace as a Vapor Project?

**Answer:**

---

## 12.2 Cloning

### When does Vapor clone repositories automatically?

**Answer:**

### When does the user explicitly request cloning?

**Answer:**

### How are authentication/private repositories handled?

**Answer:**

---

## 12.3 Submodules

### Who manages Container Repo → Source Repo submodules?

**Answer:**

### Should normal developers need to manually interact with Git submodules?

**Answer:**

### How does Vapor handle missing/uninitialized submodules?

**Answer:**

---

## 12.4 Git State

### How prominently should Vapor expose Git state?

**Answer:**

### Should Vapor perform commits?

**Answer:**

### Pushes?

**Answer:**

### Pulls?

**Answer:**

### Branch creation?

**Answer:**

### Conflict resolution?

**Answer:**

---

# 13. Normal Development Loop

Describe the intended golden-path development loop.

> Open → Edit → Build → Run → Inspect → Change → Repeat

## 13.1 Opening Work

### How does a developer find a project they were working on?

**Answer:**

### How do they open it?

**Answer:**

### Does Vapor launch an IDE/editor?

**Answer:**

---

## 13.2 Editing

### Is editing source code primarily done outside Vapor?

**Answer:**

### What editing/configuration happens directly inside Vapor?

**Answer:**

---

## 13.3 Build

### What does the developer do to build?

**Answer:**

### What feedback should they receive?

**Answer:**

---

## 13.4 Run

### What does the developer do to run/test their work?

**Answer:**

### Which App Instance is used?

**Answer:**

### Which composition is used?

**Answer:**

---

## 13.5 Inspect

### Where do logs appear?

**Answer:**

### Where do warnings/errors appear?

**Answer:**

### What runtime/development diagnostics should be available?

**Answer:**

---

## 13.6 Repeat

### How much work should be necessary between modifying code/content and seeing the result?

**Answer:**

### What parts should Vapor cache or avoid repeating?

**Answer:**

---

# 14. Build

## 14.1 Meaning

### What does "Build" mean in Vapor?

**Answer:**

### Is Vapor simply orchestrating Cargo or performing additional build stages?

**Answer:**

---

## 14.2 Scope

### Can the user build one Vapor Project?

**Answer:**

### An entire Vapor Workspace?

**Answer:**

### A Container Repo?

**Answer:**

### A complete App Composition?

**Answer:**

---

## 14.3 Build Profiles

### What build profiles/configurations should developers encounter?

**Answer:**

### How should Debug/Release or Vapor-specific profiles be represented?

**Answer:**

---

## 14.4 Build Failures

### How should compiler errors be presented?

**Answer:**

### Should Vapor interpret/commonize certain Cargo/Rust errors?

**Answer:**

### When should raw tool output remain accessible?

**Answer:**

---

# 15. Development Compositions and Running

## 15.1 Development Composition

### What composition is used while developing content?

**Answer:**

### Is a dedicated development composition created?

**Answer:**

### Is it persistent or generated temporarily?

**Answer:**

---

## 15.2 Development Content Injection

### How does locally developed content enter the running composition?

**Answer:**

### Does it replace installed content?

**Answer:**

### Overlay installed content?

**Answer:**

### Get mounted from the workspace directly?

**Answer:**

---

## 15.3 Multiple Local Projects

### Can multiple locally developed Vapor Projects participate in one composition?

**Answer:**

### How are conflicts between local and installed versions resolved?

**Answer:**

---

## 15.4 Target App Instance

### How does a Vapor Project know which App Instance it runs against?

**Answer:**

### Is this stored per project, workspace, user, or run configuration?

**Answer:**

---

# 16. Testing

## 16.1 Test Types

### What kinds of tests should Vapor explicitly support?

Possible examples:

- Rust unit tests
    
- Integration tests
    
- Content validation
    
- Composition validation
    
- Runtime tests
    
- Multiplayer tests
    
- Server tests
    

**Answer:**

---

## 16.2 Running Tests

### How does the developer run tests?

**Answer:**

### Where are results displayed?

**Answer:**

### Should Vapor expose raw Cargo test functionality as well?

**Answer:**

---

# 17. Debugging and Diagnostics

## 17.1 Debugging

### What debugging workflows should Vapor support?

**Answer:**

### Should IDE debuggers attach directly to the running composition?

**Answer:**

---

## 17.2 Logs

### What logs exist?

**Answer:**

### Where are they stored?

**Answer:**

### How are they viewed?

**Answer:**

### Can users filter logs by Engine/Game/Mod/system?

**Answer:**

---

## 17.3 Diagnostics

### What diagnostic information should be visible about an App Instance?

**Answer:**

### About a composition?

**Answer:**

### About installed content?

**Answer:**

### About the development environment?

**Answer:**

---

# 18. Publishing

## 18.1 Starting Publication

### How does a developer begin publishing Vapor Content?

**Answer:**

### What must already be true before publishing is allowed?

**Answer:**

---

## 18.2 Validation

### What should Vapor validate before publication?

**Answer:**

### Which validation failures are blocking?

**Answer:**

### Which should merely produce warnings?

**Answer:**

---

## 18.3 Packaging

### What exactly gets packaged?

**Answer:**

### Is source code included?

**Answer:**

### Are build artifacts included?

**Answer:**

### What metadata accompanies the package?

**Answer:**

---

## 18.4 Steam Workshop

### What gets uploaded to Steam Workshop?

**Answer:**

### Does one Vapor Content artifact correspond to one Workshop Item?

**Answer:**

### When is a Workshop Item created?

**Answer:**

### Can an existing Workshop Item be adopted?

**Answer:**

---

## 18.5 Vapor Content Registry

### What information gets registered in the Vapor Content Registry?

**Answer:**

### When does registration happen relative to Workshop publication?

**Answer:**

### How is the human-readable Vapor ID chosen?

**Answer:**

### Can that ID ever change?

**Answer:**

---

## 18.6 Ownership and Authentication

### How does Vapor determine who owns content?

**Answer:**

### What authentication is required?

**Answer:**

### How does collaboration between multiple developers work?

**Answer:**

---

## 18.7 Releases and Versions

### What constitutes a new release?

**Answer:**

### How are versions assigned?

**Answer:**

### Are versions immutable after publication?

**Answer:**

### Can multiple published versions remain available simultaneously?

**Answer:**

---

## 18.8 Deprecation / Removal

### Can published content be deprecated?

**Answer:**

### Removed?

**Answer:**

### Unlisted?

**Answer:**

### What happens to existing users/compositions depending on it?

**Answer:**

---

# 19. Updates

## 19.1 App Updates

### What happens when Steam updates the Loo Cast App?

**Answer:**

### What state must survive App updates?

**Answer:**

---

## 19.2 Content Updates

### How does Vapor discover content updates?

**Answer:**

### Are updates automatic?

**Answer:**

### Can users pin specific versions?

**Answer:**

### Can updates be rolled back?

**Answer:**

---

## 19.3 Developer Tool Updates

### How are Rust/Git/SteamCMD/etc. updates handled?

**Answer:**

### Should Vapor pin known-compatible tool versions?

**Answer:**

---

# 20. Compatibility and Migration

## 20.1 Compatibility

### What dimensions of compatibility exist?

Possible examples:

- Vapor ecosystem version
    
- Engine version
    
- Game version
    
- Mod API version
    
- Content schema version
    
- Platform
    
- Rust/toolchain version
    

**Answer:**

---

## 20.2 Compatible Changes

### What should happen when everything remains compatible?

**Answer:**

---

## 20.3 Incompatible Changes

### What happens when an update breaks compatibility?

**Answer:**

### Should Vapor block the update, update anyway, create a new composition, or ask?

**Answer:**

---

## 20.4 Migration

### What kinds of things may require migration?

**Answer:**

### Which migrations should Vapor perform automatically?

**Answer:**

### Which require explicit developer/user action?

**Answer:**

### Should migrations be reversible?

**Answer:**

---

# 21. Failure and Recovery

## 21.1 Download Failure

### What happens when a download fails?

**Answer:**

---

## 21.2 Registry Failure

### What happens when the Vapor Content Registry is unavailable?

**Answer:**

### What functionality should remain available offline?

**Answer:**

---

## 21.3 Steam Failure

### What happens when Steam is unavailable?

**Answer:**

### What about SteamCMD?

**Answer:**

---

## 21.4 Git Failure

### What happens when cloning/pulling/submodule initialization fails?

**Answer:**

---

## 21.5 Build Failure

### What happens after a failed build?

**Answer:**

### What state remains usable?

**Answer:**

---

## 21.6 Corrupt Content

### How does Vapor detect corrupt content?

**Answer:**

### Can it repair/redownload it automatically?

**Answer:**

---

## 21.7 Broken Composition

### What happens when the selected composition cannot resolve?

**Answer:**

### Is the last known working composition preserved?

**Answer:**

---

## 21.8 Interrupted Operations

### What happens if Vapor closes/crashes during installation, migration, publication, workspace creation, etc.?

**Answer:**

### Which operations must be transactional/recoverable from the user's perspective?

**Answer:**

---

## 21.9 Local Changes

### What should Vapor do when an operation would overwrite uncommitted/unpushed local changes?

**Answer:**

### What data must Vapor never silently destroy?

**Answer:**

---

# 22. State and Persistence

## 22.1 Global State

### What should Vapor remember globally?

**Answer:**

---

## 22.2 Per-App-Instance State

### What should Vapor remember for each App Instance?

**Answer:**

---

## 22.3 Per-Composition State

### What should Vapor remember for each composition?

**Answer:**

---

## 22.4 Per-Workspace State

### What should Vapor remember for each Vapor Workspace / Superworkspace?

**Answer:**

---

## 22.5 Per-Project State

### What should Vapor remember for each Vapor Project?

**Answer:**

---

## 22.6 User Preferences

### What user preferences should exist?

**Answer:**

---

## 22.7 Credentials

### What authentication/credential state must be persisted?

**Answer:**

### Who should actually own/store those credentials?

**Answer:**

---

# 23. Manual Control and Escape Hatches

## 23.1 General Principle

### Where should Vapor provide a golden path?

**Answer:**

### Where should users be free to diverge from it?

**Answer:**

### What should Vapor orchestrate without trying to own?

**Answer:**

---

## 23.2 Existing Tool Installations

### Can users supply their own Git installation?

**Answer:**

### Rust/Cargo installation?

**Answer:**

### SteamCMD installation?

**Answer:**

### IDE/editor?

**Answer:**

---

## 23.3 Existing Development Structures

### Can Vapor adopt an existing Git repository?

**Answer:**

### Existing Cargo workspace?

**Answer:**

### Existing Vapor-compatible directory structure?

**Answer:**

---

## 23.4 Existing Content

### Can Vapor adopt existing locally installed content?

**Answer:**

### Existing Workshop Items?

**Answer:**

### Existing registry entries?

**Answer:**

---

## 23.5 Manual Filesystem Interaction

### Is manually editing Vapor-managed files supported?

**Answer:**

### Which files are explicitly considered user-editable?

**Answer:**

### Which files are implementation detail / generated state?

**Answer:**

---

## 23.6 Manual CLI / Tool Usage

### Can developers freely invoke Cargo/Git/SteamCMD themselves?

**Answer:**

### How should Vapor react when external tools alter state behind its back?

**Answer:**

---

# 24. Trust Boundaries and Ownership

## 24.1 Vapor-Owned State

### What does Vapor consider itself responsible for owning and maintaining?

**Answer:**

---

## 24.2 User-Owned State

### What does Vapor consider explicitly user-owned?

**Answer:**

---

## 24.3 Third-Party-Owned State

### What state is fundamentally owned by Steam, Git hosts, Rustup, external IDEs, etc.?

**Answer:**

---

## 24.4 Principle

Complete the sentence:

> Vapor should orchestrate ____________________, but should not attempt to own ____________________.

**Answer:**

---

# 25. Golden Paths

For each important workflow, describe the **ideal boring case**.

Do not explain every failure mode here. Just describe the intended straight-line experience.

---

## 25.1 Player

> Install → Launch → Play

**Answer:**

---

## 25.2 Mod User

> Discover Mod → Install → Enable → Play

**Answer:**

---

## 25.3 First-Time Developer

> Enable Developer Mode → Environment Ready → Create Content → Open → Run

**Answer:**

---

## 25.4 Returning Developer

> Launch Vapor → Open Existing Project → Edit → Build → Run

**Answer:**

---

## 25.5 New Mod

> Create Mod → Select Target → Generate Project → Develop → Test

**Answer:**

---

## 25.6 Publishing

> Validate → Package → Publish → Register → Available to Users

**Answer:**

---

## 25.7 Ecosystem Development

> Obtain Vapor Sources → Establish Development Environment → Modify Vapor → Build/Test Against App Instance

**Answer:**

---

# 26. UX Principles

These are not implementation requirements yet. They are guiding rules used later to resolve ambiguous decisions.

## 26.1 Automation

Complete:

> Vapor should automatically ____________________.

**Answer:**

> Vapor should never automatically ____________________.

**Answer:**

---

## 26.2 Transparency

### How much internal detail should normal users see?

**Answer:**

### How much should developers be able to inspect?

**Answer:**

---

## 26.3 Recoverability

Complete:

> When something goes wrong, Vapor should generally ____________________.

**Answer:**

---

## 26.4 Destructive Actions

Complete:

> Vapor may only destroy/overwrite user state when ____________________.

**Answer:**

---

## 26.5 Complexity

Complete:

> Beginners should be able to ____________________ without understanding ____________________.

**Answer:**

> Advanced users should still be able to ____________________.

**Answer:**

---

## 26.6 Progressive Disclosure

### What complexity should only appear when the user asks for it or needs it?

**Answer:**

---

# 27. Explicit Non-Goals

### What should the Vapor App/SDK deliberately **not** try to become?

**Answer:**

### What responsibilities should remain with Steam?

**Answer:**

### Git?

**Answer:**

### Cargo/Rustup?

**Answer:**

### IDEs/editors?

**Answer:**

### Operating system/package managers?

**Answer:**

---

# 28. Open Questions

Use this section for anything discovered while answering the questionnaire that does not yet have a natural home.

## Open Question 1

**Question:**

**Current Thoughts:**

---

## Open Question 2

**Question:**

**Current Thoughts:**

---

## Open Question 3

**Question:**

**Current Thoughts:**

---

# 29. Decisions

Use this section for conclusions that emerge strongly enough that they should no longer remain buried inside individual answers.

## Decision 1

**Decision:**

**Reasoning:**

**Consequences:**

---

## Decision 2

**Decision:**

**Reasoning:**

**Consequences:**

---

# 30. Glossary Changes Discovered

Use this section whenever answering the Experience Model reveals that the existing Ecosystem Model Glossary is incomplete, ambiguous, or incorrect.

## Proposed Glossary Change 1

**Affected Concept:**

**Problem:**

**Proposed Change:**

---

## Proposed Glossary Change 2

**Affected Concept:**

**Problem:**

**Proposed Change:**