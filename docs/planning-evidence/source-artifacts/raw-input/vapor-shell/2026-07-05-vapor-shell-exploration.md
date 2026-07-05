# Vapor-Shell Exploration Transcript

- Date: 2026-07-05
- Context: Direct owner/assistant exploration before knowledgebase decomposition.
- Preservation: Dialogue text below is retained verbatim as the source for
  mechanical atomization. Speaker labels and this metadata are source-document
  structure.

## Owner input 1

Also..... Imma be honest man, I think we need a Vapor-Shell for SDK and Launcher CLI and general Vapor CLI stuff all bundled into one custom shell, yk? Like how certain commands like git have like these crazy shells where git itself is like a full REPL kinda loop, yk? I wanna have that, so we can route (and gate) workflow actions of ANY kind 8regarding git and the sorts) through explicit controlled simple-to-use idiot-proof vapor surfaces, via commands and submodes in this like "shell mode", lol.

Let's discuss and explore this a bit, before we decompose and move this into the documentation/planning queue, yk?

Also, how about we do a quick commit+push once we feel this Agent-upgrade is done, yk? Not now, I just wanted to mention it already.

## Assistant exploration 1

The cleanest version is not a general-purpose OS shell. It is a unified Vapor control plane with two equivalent interfaces:

```text
vapor <command>      # one-shot use and automation
vapor shell          # persistent interactive session
```

Both should dispatch the same typed Vapor operations. The existing SDK and Launcher CLIs already expose typed command models, so the shell can unify those models instead of invoking the binaries or reimplementing their parsers.

Inside the session, it could maintain explicit context:

```text
vapor> workspace open ~/Projects/MyGame
vapor[MyGame]> sdk
vapor[MyGame/sdk]> build
vapor[MyGame/sdk]> check
vapor[MyGame/sdk]> back
vapor[MyGame]> launcher
vapor[MyGame/launcher]> launch
```

The important architectural distinction is this:

- Vapor commands express user intent.
- Git, Cargo, GitHub, Steam, and filesystem operations are controlled backends.
- The shell previews and gates meaningful mutations.
- Raw backend commands are either unavailable or confined to an explicit expert escape hatch.
- Interactive state never silently weakens safety—especially no persistent `force` mode.
- Every consequential operation should produce a readable receipt: what changed, what remains pending, and whether external state was touched.

That would let commands such as `change start`, `save`, `sync`, `publish`, `repair`, and `release` replace fragile sequences of Git and tool-specific commands. SDK and Launcher then become domains or modes within one operation graph rather than unrelated command-line programs.

The major unresolved architectural choice is ownership. Putting this in Vapor-SDK would incorrectly make Launcher subordinate to SDK; putting it in Vapor core risks reversing dependency layers. A dedicated `Vapor-Shell` project—or a shared orchestration layer with a thin `vapor` executable—is currently the cleanest shape.

```text
VAPOR-ROOT / vapor-shell exploration / DECISION

The core concept...

[1] Interaction model (REPL/submodes/context)
[2] command/control model (typed command graph + SDK/Launcher)
[3] safety/backend gating (Git/GitHub/Cargo/Steam)
[4] packaging/repository ownership
[P] Park
[X] Exit
[>] Tell me what part...
>
```

The eventual Agent-upgrade commit and push is noted, but remains deferred as requested.

## Owner input 2

This also eliminates any shenanigans with working directory, as this tool can just detect whether it's in ANY sort of Vapor folder, and from there just jump to the workspace, or wherever it needs to be, yk? I really fucking dislike the idea of this shell not being a global/universal stable entrypoint but a working-directory-dependent contextual dynamic movable thing, yk?



Also, I created the Vapor-Shell repo. It should also be a submodule of Vapor-Root I think. and depend on sdk and launcher and core ofc. Examples too, for in-tool explanations, cause like this is supposed to be one giant CLI tool to teach you Vapor and help you with Vapor and to document Vapor from automated docs creation and pre-processing or whatever the fuck, yk? The real-deal, basically. Lol.

I choose [E]: Explore further!

## Assistant exploration 2

`vapor` should be a globally installed executable whose identity and behavior never depend on where it was invoked.

The current SDK implementation still calls `current_dir()` internally. That should eventually be replaced with an explicitly resolved execution context:

```text
invocation path
    ↓ discovery only
absolute Vapor context
    ↓
typed operation
    ↓
explicit absolute targets
```

The shell should not actually `chdir` itself. It resolves the applicable workspace/root once, then gives every operation absolute paths. Child tools such as Cargo or Git can receive an explicit working directory.

Context resolution should follow a stable precedence:

1. Explicit target supplied by the user.
2. Active interactive-session target.
3. Nearest recognized `Vapor.toml` above the invocation path.
4. Registered workspace selected by stable Vapor ID.
5. Context-free global mode.

Thus all of these invoke the same installed tool:

```text
vapor status
vapor --workspace ghf-studios.vapor-sdk build
vapor use ghf-studios.vapor-sdk
vapor shell
```

Being inside a Vapor directory is a convenience signal, never a requirement.

## Repository architecture

`Vapor-Shell` is the correct owner for the unified frontend:

```text
Vapor-Shell
├── vapor_shell_core
│   ├── context discovery and workspace registry
│   ├── session state
│   ├── operation catalog
│   ├── planning, safety, and receipts
│   └── unified dispatch
├── vapor_shell_cli
│   ├── one-shot commands
│   └── interactive shell
└── adapters
    ├── vapor_core
    ├── vapor_sdk_core
    ├── vapor_launcher_core
    ├── Git/Cargo/Steam
    └── documentation and examples
```

Its dependency direction would be:

```text
Vapor-Shell
├── Vapor-SDK → Vapor
└── Vapor-Launcher → Vapor
```

It should become another `Vapor-Root` submodule alongside the existing four. The repository currently exists remotely but is not present in the local checkout yet; adding it remains a later mutation.

`Vapor-Examples` is slightly different. The shell should consume it as an explanation/example corpus, not link against it as an ordinary Rust dependency. Otherwise examples become production infrastructure and create the wrong dependency direction. A generated, versioned reference bundle can be built from the pinned Examples submodule and packaged with `vapor`.

## One executable, several surfaces

“One giant CLI tool” should mean one coherent product, not one giant implementation crate:

```text
vapor                         interactive home
vapor sdk build               explicit one-shot operation
vapor launcher launch         explicit one-shot operation
vapor explain sdk.build       conceptual explanation
vapor example packagepack     working example
vapor docs ...                documentation workflows
vapor git ...                 controlled backend surface, if permitted
```

SDK and Launcher become operation domains. They need not always be visible as modes when Vapor can infer the domain from the selected workspace.

## Operation catalog

The shell’s central abstraction should be an operation descriptor containing:

- Stable operation ID.
- Valid contexts and prerequisites.
- Inputs and defaults.
- Filesystem and external effects.
- Risk/confirmation class.
- Planning support.
- Examples.
- Explanation and documentation links.
- Typed dispatcher.

That single catalog can drive:

- CLI parsing and completion.
- Interactive menus.
- Context-sensitive availability.
- `explain`.
- Safety previews.
- Automated reference documentation.
- Example lookup.
- Eventually a GUI.

A consequential command follows one uniform lifecycle:

```text
resolve context → validate → plan → gate → apply → verify → receipt
```

That is the real unification point. SDK and Launcher currently have separate command specifications and duplicated CLI safety logic; Vapor-Shell can normalize them initially, after which common policy can migrate into the shared operation layer.

```text
VAPOR-ROOT / vapor-shell exploration

[1] Context system
    Discovery, registration, IDs, nested workspaces

[2] Operation system
    Catalog, dispatch, plans, gates, receipts

[3] Interactive experience
    Home screen, prompts, modes, navigation

[4] Teaching and documentation
    Explain, examples, generated docs, preprocessing

[5] Repository topology
    Crates, submodule, dependency mechanics

[C] Capture the explored concept
[P] Park
[X] Package and exit
[>] Continue naturally

>
```
