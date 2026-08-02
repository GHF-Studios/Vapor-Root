- Unified Capability Engine: *An engine architecture in which behavior is modeled as capability nodes connected through explicit boundaries rather than ad-hoc hooks or loose scripting.*
- Unified Capability Supergraph: *The full graph of capability nodes and their relationships within the Unified Capability Engine.*

- Immutable Core: *The runtime-immutable inner architecture of the engine. It consists of Layer 1 and Layer 2.*
- Layer 1: *The pure Rust-native foundational layer of the Immutable Core, packaged as its own library and containing hardcoded kernels, primitive types, schedulers, and other foundational machinery.*
- Layer 2: *The capability-authored layer of the Immutable Core, packaged as its own dynamically linkable library. It is built from Layer 2 Capability Nodes.*
- Core Boundary: *The constrained interface between Layer 1 and Layer 2, where interaction must obey FFI-like or similarly strict linkage rules.*

- Layer 3: *The pure Rhai dynamic scripting layer of the architecture, through which capability structures are defined and executed at runtime. Unlike Layer 1 and Layer 2, it remains runtime-mutable.*

- Capability Node: *One capability object within the Unified Capability Supergraph.*
- Layer 2 Capability Node: *A Capability Node authored as one primary Rhai/Rhai-DSL module that produces one capability object. When composition changes, Layer 2 Capability Nodes are translated into Rust and statically linked into the Layer 2 library.*

- Cross-Time Capability Model: *A capability model in which Layer 1, Layer 2, and Layer 3 participate in one continuous graph, with statically declared dynamic attachment patterns that allow runtime-provided capabilities to participate in that graph, including dependency inversion where appropriate.*
- Capability Hierarchy: *The graph-structured dependency ordering of capability nodes within the Unified Capability Supergraph. Each capability node is one capability and may consume zero or more other capabilities, including capabilities from other layers and from higher layers through dependency inversion.*
- Capability Scope: *The operational scope of a capability instance. In Layer 1 and Layer 2, maximum scope is statically allocated; in Layer 3, scope may be dynamically allocated, deallocated, and reshaped at runtime.*