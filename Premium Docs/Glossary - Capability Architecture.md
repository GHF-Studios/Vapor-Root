- Unified Capability Engine: *An engine architecture in which behavior is modeled as capability nodes connected through explicit boundaries rather than ad-hoc hooks or loose scripting.*
- Unified Capability Supergraph: *The full graph of capability nodes and their relationships within the Unified Capability Engine.*

- Immutable Core: *The runtime-immutable inner architecture of the engine, packaged as two libraries: the Native Core and the Compiled Capability Layer.*
- Native Core: *The Layer 1 library of the Immutable Core, containing hardcoded Rust-native kernels, primitive types, schedulers, and other foundational machinery.*
- Compiled Capability Layer: *The Layer 2 library of the Immutable Core, produced by compiling Layer 2 capability nodes, authored as Rhai/Rhai-DSL modules, and statically linking them into one dynamically linkable unit.*
- Core Boundary: *The constrained interface between the Native Core and the Compiled Capability Layer, where interaction must obey FFI-like or similarly strict linkage rules.*

- Dynamic Layer: *The mutable runtime layer that extends the Immutable Core during execution.*
- Runtime Script Layer: *The interpreted dynamic scripting surface through which capability structures are defined and executed at runtime. It forms Layer 3 of the architecture.*
- Attachment Point: *An explicit extension point exposed by the Immutable Core through which the Dynamic Layer may add runtime behavior.*

- Capability Node: *One capability object within the Unified Capability Supergraph.*
- Layer 2 Capability Node: *A Capability Node authored as one primary Rhai script module that produces one capability object.*

- Cross-Time Capability Model: *A capability model in which authored, compiled, and runtime capability relationships are treated as one continuous architecture.*
- Capability Hierarchy: *The rule that capabilities are implemented by consuming other capabilities positioned closer to the root of the Unified Capability Supergraph.*
- Capability Scope: *The operational scope of a capability instance. In Layer 1 and Layer 2, maximum scope is statically allocated; in Layer 3, scope may be dynamically allocated, deallocated, and reshaped at runtime.*
