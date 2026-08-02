- **Unified Capability Engine**: *An engine architecture in which behavior is modeled as capability nodes connected through explicit boundaries rather than ad-hoc hooks or loose scripting.*
- **Unified Capability Supergraph**: *The full directed acyclic graph of capability nodes and their relationships within the Unified Capability Engine.*

- **Immutable Core**: *The runtime-immutable inner architecture of the engine. It consists of Layer 1 and Layer 2.*
- **Layer 1**: *The pure Rust-native foundational layer of the Immutable Core, packaged as its own library and containing hardcoded kernels, primitive types, schedulers, and other foundational machinery.*
- **Layer 2**: *The capability-authored layer of the Immutable Core, packaged as its own dynamically linkable library. It is built from Layer 2 Capability Nodes.*
- **Core Boundary**: *The constrained interface between Layer 1 and Layer 2, where interaction must obey FFI-like or similarly strict linkage rules.*

- **Layer 3**: *The pure Rhai dynamic scripting layer of the architecture, through which capability structures are defined and executed at runtime. Unlike Layer 1 and Layer 2, it remains runtime-mutable.*

- **Capability Node**: *One capability object within the Unified Capability Supergraph. A capability node is the capability itself: a first-class API object that may consume zero or more other capabilities and may declare or satisfy zero or more Capability Marker Traits.*
- **Layer 1 Capability Node**: *A Capability Node implemented directly in Rust as part of Layer 1. It is native to the Immutable Core.*
- **Layer 2 Capability Node**: *A Capability Node authored as one primary Rhai/Rhai-DSL module that produces one capability object. When composition changes, Layer 2 Capability Nodes are translated into Rust and statically linked into the Layer 2 library.*
- **Layer 3 Capability Node**: *A Capability Node authored and executed in pure Rhai as part of Layer 3. Unlike Layer 1 and Layer 2 Capability Nodes, it remains runtime-mutable.*
- **Capability Marker Trait**: *A declarative marker trait that carries no behavior and is used to classify capability nodes and constrain capability edges. A Capability Marker Trait may be declared locally or upstream and then reused or composed downstream.*
- **Capability Edge**: *One declared, typed, directed edge within the Unified Capability Supergraph's directed acyclic graph. Edge constraints such as Capability Marker Trait requirements and edge cardinality belong to the edge, not to the marker traits themselves.*

- **Cross-Time Capability Model**: *A capability model in which Layer 1, Layer 2, and Layer 3 participate in one continuous graph, with statically declared dynamic attachment patterns that allow runtime-provided capabilities to participate in that graph, including dependency inversion where appropriate.*
- **Capability Hierarchy**: *The graph-structured dependency ordering of capability nodes within the Unified Capability Supergraph. Each capability node explicitly declares its capability edges and their Capability Marker Trait constraints via metadata, schemas, macros, DSL declarations, or equivalent mechanisms, allowing the engine to construct a strict capability graph across layers, including dependency inversion where appropriate.*
- **Capability Scope**: *The operational scope of a capability instance, enforced via a strict structural provision model. In Layer 1 and Layer 2, the permitted capability set declared by a node's capability edges and Capability Marker Trait constraints is statically determined; in Layer 3, it may be dynamically allocated and reshaped. In all cases, only the subset structurally demanded by the node's metadata, schemas, or method signatures is actually provisioned as API composite objects.*
