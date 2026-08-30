- **Universal Simulation Framework (USF)**: *The public/API-facing Spacetime Engine subsystem for representing, simulating, and interacting with a universe across spatial and temporal magnitudes. The USF defines the world model, scale-aware representation model, and runtime behavior model used by Loo Cast.*
- **Simulation State**: *The canonical represented condition of the simulated universe, formed by resolving the Simulation Substrate together with Phenomenon-owned state and refinements.*
- **Simulation Substrate**: *The spatially organized base representation of Simulation State. It carries ambient, coarse, and distributed state that may be locally refined by Phenomena.*
- **Runtime Substrate**: *The ECS, scheduling, storage, queueing, and execution machinery through which the USF runs. It executes the USF Model but does not define simulation meaning or state authority.*
- **State Authority**: *The resolved responsibility for determining canonical state for a particular state channel, domain, and interval. The same fact may not be independently authoritative in multiple representations.*
- **State Arbitration**: *The explicit policy for resolving competing state changes or authority demands. Arbitration may combine contributions, select deterministically or randomly, defer or wait, or reject the contention.*

- **Scale**: *The context-dependent magnitude structure of simulation state and behavior. Scale is not one universal numeric coordinate: Spatial Scale provides coordinate infrastructure, while Mechanisms describe their own relevance across heterogeneous conditions.*
- **Spatial Scale**: *The spatial-magnitude coordinate used for hierarchical addressing, partitioning, representation, loading, refinement, aggregation, and presentation. Spatial Scale does not by itself determine simulation meaning.*
- **Temporal Scale**: *A duration, frequency, or rate magnitude relevant to simulation behavior. Temporal Scale is Mechanism- and context-dependent rather than being derived from Spatial Scale alone.*
- **Scale Level**: *One of the 71 logarithmic Spatial Scale coordinates from 10^-35 metres through 10^35 metres. A Scale Level selects spatial magnitude and resolution, not a complete Simulation Regime.*
- **Chunk**: *The first-level spatial partition at one Scale Level. Chunks are nested exactly by Spatial Scale, contain 1000^3 scale-local units, and may use adaptive internal representations; they are substrate containers rather than the primary identity or authority of significant world structures.*
- **State Representation**: *A concrete form in which Simulation State is stored, summarized, evaluated, or exposed. Coarse and fine representations may coexist, but State Authority identifies which representation is canonical for each fact.*
- **Representation Tolerance**: *The permitted loss, approximation, or error for a State Representation in a particular context. Exceeding the tolerance creates pressure to refine or otherwise change representation.*
- **Simulation Focus**: *The current allocation of simulation work and representational fidelity across the universe. Observer demand, significance, gameplay rules, and resource budgets may influence Simulation Focus, but it does not determine world truth or state authority.*

- **Quantity**: *A semantically typed value with a defined meaning, representation, and unit or dimension where applicable.*
- **Metric**: *A named USF observable that yields a Quantity from Simulation State. A Metric may expose stored state or derive its result, but it is distinct from both an individual value and its spatial distribution.*
- **Field**: *A spatially and/or temporally distributed set of Quantity values represented through the Simulation Substrate or a local Refinement.*

- **Mechanism**: *A causal simulation process that is evaluated automatically wherever its applicability conditions are satisfied. A Mechanism may inspect Simulation State, evolve it through state transitions, and invoke provisioned Capabilities without itself requiring explicit invocation.*
- **Mechanism Relevance Profile**: *A Mechanism-specific description of how its relevance varies across the Metrics, Quantities, Phenomena, spatial magnitudes, temporal magnitudes, and other conditions that matter to it. Relevance Profiles are heterogeneous and do not share one required dimensional schema.*
- **Mechanism Representation**: *The selected way a relevant Mechanism is simulated in a particular context, such as explicit execution, a reduced model, an aggregate model, or intentional omission within Representation Tolerance.*
- **Simulation Regime**: *The contextually derived set of relevant Mechanisms and their selected Mechanism Representations for a particular domain and interval. A Simulation Regime is not defined by one Scale Level.*
- **Regime Constraint**: *An explicit rule that constrains or overrides a derived Simulation Regime for gameplay, debugging, performance, simulation, or other declared purposes.*
- **Representation Transition**: *A change from one State Representation or Mechanism Representation to another. Representation Transitions may trigger Materialization or Re-aggregation and are not inherently tied to crossing an integer Scale Level.*

- **Phenomenon Definition**: *The shared definition of one kind of Phenomenon, including its state schema, support rules, significance rules, refinement behavior, and materialization/re-aggregation behavior.*
- **Phenomenon**: *One concrete, identifiable materialization of a coherent structure or pattern in Simulation State. A Phenomenon carries the state that makes it distinct and may maintain local Refinements; representation changes normally preserve its identity, while creation, destruction, splitting, merging, and dissolution are explicit lifecycle changes.*
- **Phenomenon Support**: *The spatial, temporal, and/or semantic domain in which a Phenomenon exists or exerts influence. Support may be crisp or fuzzy, connected or disconnected, and may change over time.*
- **Refinement**: *A finer local representation maintained because the surrounding Simulation Substrate cannot represent the required state within Representation Tolerance. A Refinement may contain finer substrate data and child Phenomena.*
- **Residual State**: *Significant information that cannot be represented by a coarser State Representation without unacceptable loss. Residual State is canonical retained detail rather than merely a disposable cache.*
- **Materialization**: *The creation or activation of a finer representation of a Phenomenon or Simulation Substrate. Materialization normally preserves Phenomenon identity and transfers State Authority to finer state channels where required.*
- **Re-aggregation**: *The compression of finer state into a coarser representation, transferring State Authority back while preserving declared invariants, significant information, and required Residual State. Re-aggregation is not required to be a perfect mathematical inverse of Materialization.*

- **Capability**: *An explicitly invocable behavior exposed by the USF. Access to a Capability is granted through a Capability Handle; an ordinary Rust or Rhai function is not a Capability unless the USF exposes it as one. Capability is a USF runtime concern, not a Vapor-wide build or composition primitive.*
- **Capability Definition**: *The shared definition of a Capability's identity, invocation contract, requirements, and executable backing. Capability Definitions do not contain hidden mutable simulation state; mutable state remains explicit in the Capability Context.*
- **Capability Handle**: *The granted callable reference to a Capability. Possession of the Handle is the permission to invoke that Capability within the Handle's scope.*
- **Capability Provision**: *The scoped set of Capability Handles made available to a Mechanism, Phenomenon, script, system, tool, or other context.*
- **Capability Context**: *The invocation-specific access to inputs, queries, state, authority, and intent emission supplied alongside a Capability Handle.*
- **Capability Invocation**: *One explicit use of a Capability Handle. Capability Invocation is distinct from the automatic evaluation of a Mechanism.*
- **Pure Capability**: *A Capability whose result depends only on its explicit inputs and which neither observes nor affects mutable Simulation State or external state.*
- **State-Affecting Capability**: *A Capability that observes or affects explicit mutable state. It may mutate state directly only when its Capability Context grants the required State Authority; otherwise it emits an Intent for arbitration.*

- **Intent**: *A proposed State Transition emitted by a Mechanism, Capability Invocation, or runtime process. An Intent is not canonical state until it has been accepted through the applicable State Arbitration.*
- **State Transition**: *An accepted and applied change to canonical Simulation State.*
- **Event**: *An immutable record or notification that something occurred. An Event may inform later behavior but does not itself own or mutate Simulation State.*
- **Simulation Step**: *One local advancement of simulated time. Its duration is selected from the active Simulation Regime, Representation Tolerance, and available simulation budget rather than from Spatial Scale alone.*
- **Causal Continuity**: *The requirement that significant causes and effects survive changes of Scale Level, Simulation Regime, State Representation, loading state, and Simulation Focus through authoritative summaries, Residual State, Intents, Events, or other explicit propagation.*
