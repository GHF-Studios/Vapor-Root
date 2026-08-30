- **Capability**: *An exported, permissioned, composable operation surface. A Capability represents something another Capability, context, script, system, or layer may be allowed to invoke.*
- **Capability ID**: *A stable string-like identifier for a Capability, used to connect metadata, dependency declarations, provisioning, permissions, and cross-layer calls.*
- **Atomic Capability**: *A single operation-level Capability, usually corresponding to one exported callable surface such as a function, method-like wrapper, constructor-like operation, command, query, or action.*
- **Composite Capability**: *A named grouping of multiple Atomic Capabilities for permissioning, provisioning, dependency declaration, or authoring convenience. A Composite Capability is not itself necessarily one callable operation.*
- **Capability Surface**: *The public/API-like boundary through which a Capability is exposed. Vapor tracks the Surface, while ordinary internal Rust implementation details usually remain untracked.*
- **Capability Backing**: *The concrete logic behind a Capability Surface. A Backing may be handwritten Rust, generated Rust, a normal Rust function, a method-like wrapper, a command system, a registry operation, or a Layer 3 dynamic/scripted callable.*
- **Capability Contract**: *The public expectations of a Capability Surface, including its ID, input types, output types, permissions, markers, dependency requirements, and call constraints.*

- **Capability-Exposing Type**: *A Rust type that has associated Capabilities, such as constructors, methods, queries, conversions, or operations. The type itself is not a Capability.*
- **Capability-Exposing Module**: *A Rust module that organizes or exposes Capabilities under a namespace. The module itself is not a Capability.*
- **Capability-Exposing Crate**: *A Rust crate that declares, contains, exports, or depends on Capabilities. The crate itself is not a Capability.*
- **Vapor-Tracked Surface**: *A public/exported item that participates in Vapor's metadata system. Internal implementation details do not need to be Vapor-tracked.*
- **Internal Implementation Detail**: *Any Rust item, module, helper, private type, private function, or internal subsystem that is not exposed as a Capability Surface and therefore does not need Capability metadata.*

- **Capability Supergraph**: *The logical directed acyclic graph of Vapor Capabilities across all active layers. The Supergraph tracks Capability IDs, calls, permissions, markers, provisioning rules, and cross-layer callability.*
- **Capability Node**: *A graph node representing one exported Capability Surface. In Layer 1 and Layer 2, this usually corresponds to a concrete Rust function, closure, method-like wrapper, command, or generated callable item. In Layer 3, it may correspond to a runtime/scripted callable.*
- **Capability Call**: *A directed call relationship between two Capability Nodes. If Capability A calls Capability B as part of its implementation, the Supergraph contains a call from A to B, represented as an edge in the graph.*
- **Caller Capability**: *The Capability Node whose implementation calls another Capability.*
- **Callee Capability**: *The Capability Node being called.*
- **Outbound Capability Call**: *A Capability Call as viewed from the Caller Capability.*
- **Inbound Capability Call**: *A Capability Call as viewed from the Callee Capability.*
- **Capability Marker Trait**: *A declarative, behaviorless marker used to classify Capability Surfaces and constrain which Capabilities may be called or provisioned in a given context.*

- **Capability Handle**: *A permissioned access reference to a Capability Node. In Layer 3, this may exist as an actual runtime handle object. In Layer 1 and Layer 2, it may compile away into ordinary static Rust calls, imports, generated bindings, or direct function references.*
- **Static Capability Reference**: *A Layer 1 or Layer 2 reference to another Capability that resolves to a normal Rust call, import, function pointer, closure, generated wrapper, or statically known callable item.*
- **Static Capability Call**: *A Capability call resolved through normal Rust compilation and static linking. This is the preferred model for Layer 1 and Layer 2.*
- **Dynamic Capability Call**: *A Capability call resolved through runtime lookup, registry access, scripting, or dynamic dispatch. This is the preferred model for Layer 3.*
- **Capability Call Surface**: *The concrete way a Capability can be invoked from a given layer or context. The same logical Capability may expose different call surfaces to compiled Rust and Layer 3 scripting.*
- **Capability Invocation Permission**: *The rule determining whether one Capability, context, script, system, or provisioned scope may invoke another Capability.*

- **Capability Metadata**: *The framework-readable information describing a Capability Surface. This may include its ID, namespace, input/output types, marker traits, dependency declarations, permissions, layer, crate/module location, documentation, and exposure rules.*
- **Capability Manifest**: *A metadata file or manifest section declaring the Capabilities exposed by a package, crate, module, content unit, or Vapor project.*
- **Native Capability Registration**: *A Rust macro- and metadata-driven registration path for Rust-implemented Capability Surfaces.*
- **Capability Registration**: *The act of making a Capability Surface known to Vapor through metadata, macros, manifests, generated catalogs, or runtime registration.*
- **Capability Catalog**: *The collected view of known Capability metadata for a workspace, package, app composition, build, or runtime session.*

- **Immutable Core**: *The statically linked runtime-immutable core of the selected engine/game composition. It consists of Layer 1 and Layer 2.*
- **Layer 1**: *The low-level Rust-native foundation provided by the engine/root runtime. It exposes root Capability Surfaces for third-party bindings, ECS foundations, schedulers, primitive operations, platform APIs, and other functionality whose internal capability dependencies Vapor does not attempt to track.*
- **Layer 2**: *The statically compiled, Vapor-tracked content layer. It contains compiled Rust content, game logic, engine/game extensions, first-party or third-party content crates, and other statically linked Capability Surfaces selected for the composition.*
- **Layer 3**: *The runtime/dynamic capability layer. It uses Rhai, Rhai-DSL, runtime registration, scripting, or dynamic dispatch to invoke existing Capabilities and optionally define additional runtime Capabilities.*
- **Layer Boundary**: *The conceptual boundary between Capability layers. The boundary does not imply a dynamic linkage boundary; Layer 1 and Layer 2 are fused into one statically linked Immutable Core.*

- **Vapor Framework**: *The project, metadata, and build orchestration framework around Cargo/Rust. Vapor does not replace rustc; it manages Vapor source structure, scaffolding, Capability metadata, validation, package composition, and then invokes the normal Rust toolchain.*
- **Vapor Toolchain**: *The set of tools used to manage Vapor projects, metadata, workspaces, packages, Capabilities, builds, validation, and runtime composition.*
- **Vapor Build Orchestration**: *The process by which Vapor prepares, validates, and invokes the normal Cargo/Rust build pipeline for a selected App Composition.*
- **Vapor Scaffold**: *A generated template for a crate, module, type, function, manifest, or Capability Surface. Scaffolding helps create metadata-bearing public surfaces but is not required for every internal implementation detail.*
- **Vapor Validation Pass**: *A framework-level check over metadata, dependency declarations, capability visibility, layer boundaries, permissions, package composition, or graph invariants.*
- **Capability Catalog Generation**: *The process of collecting declared Capability metadata into a usable catalog for build-time validation, tooling, editor support, documentation, and runtime provisioning.*

- **Layer 2 Content Unit**: *A statically compiled content crate, module, package component, or source unit that exposes Capability Surfaces into the Immutable Core.*
- **Static Capability Dependency**: *A dependency between Capabilities that must be resolvable before the final static build.*
- **Capability Dependency Depth**: *The inferred depth of a Capability within the static dependency graph, based on what other Capabilities it calls.*
- **Capability Stratum**: *A validation and build-ordering view over Layer 2 Capabilities. A Stratum contains Layer 2 Capabilities whose tracked static calls only target Layer 1 roots or earlier Layer 2 strata.*
- **Capability Strata Inference**: *The process of analyzing Layer 2 Capability call metadata to infer static dependency depth. Strata prevent Layer 2 from becoming an unbounded number of explicit architectural layers.*
- **Layer 2 Reconstitution**: *The final static composition step in which selected Layer 2 content is built together with Layer 1 into the Immutable Core.*

- **Rhai Capability Authoring**: *A Rhai or Rhai-DSL authoring path for invoking available Capabilities and defining additional runtime/dynamic Capabilities within Layer 3.*
- **Capability Registry Context**: *A provisioned runtime context that exposes selected Capability Handles, language features, and registration operations without exposing the full global engine/runtime state.*
- **Runtime Capability Registration**: *The act of making a Layer 3 Capability known during runtime through a provisioned registry context.*
- **Runtime Capability Node**: *A Capability Node introduced by Layer 3 at runtime. It participates in the logical Capability Supergraph but may use dynamic dispatch, scripting, or runtime-managed backing logic.*
- **Capability Language Feature**: *A function, operation, type adapter, helper, or DSL construct exposed to Layer 3 authoring so scripts can use or define Capabilities safely.*

- **Capability Provision**: *A scoped view of available Capabilities. A Provision determines which Capability Handles, call surfaces, language features, permissions, and registration operations are available to a context.*
- **Provisioned Context**: *A runtime, script, tool, build step, system, or authoring environment that receives a limited set of Capabilities through a Provision.*
- **Capability Permission**: *A rule controlling whether a Capability may be viewed, invoked, registered, extended, replaced, or used as a dependency.*
- **Dependency-Inverted Capability Slot**: *An attachment point where a lower or earlier layer declares a need or extension point that may be satisfied by a higher or later layer, while preserving the graph's validity rules.*

- **Capability Bundle**: *A package, crate, content unit, or runtime artifact that contains or exposes one or more Capability Surfaces. A Bundle is not itself a Capability.*
- **Capability Package Surface**: *The set of Capabilities publicly exposed by a package, mod, engine, game, or content unit.*
- **Capability Exposure Boundary**: *The public boundary at which internal Rust/content implementation becomes visible to Vapor as declared Capability metadata.*
- **App Composition Capability Set**: *The effective set of Capabilities available in a selected App Composition after resolving the selected Packagepack, Engine, Game, Mods, and Extensions.*
- **Ecosystem Capability Mapping**: *The relationship between ecosystem-level units and Capability-level surfaces. Projects, repos, packages, engines, games, and mods provide organizational and distribution structure; Capabilities provide invocable operation surfaces inside that structure.*

- **Cross-Layer Capability Model**: *The model in which Layer 1, Layer 2, and Layer 3 participate in one logical Capability Supergraph while remaining free to use different internal implementation and dispatch models.*
- **Static-to-Dynamic Capability Bridge**: *The metadata, registry, handle, or scripting surface that allows Layer 3 to invoke Capabilities exposed by the statically linked Immutable Core.*
- **Dynamic-to-Static Capability Reference**: *A Layer 3 reference to a Layer 1 or Layer 2 Capability through its stable Capability ID and provisioned call surface.*
- **Static Capability Surface Export**: *The process of exposing a Layer 1 or Layer 2 static Rust Capability to the metadata/catalog/provisioning system so it can be referenced by other layers.*
