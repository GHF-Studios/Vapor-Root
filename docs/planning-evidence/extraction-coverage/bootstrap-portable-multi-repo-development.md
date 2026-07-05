# Extraction Coverage — Portable Multi-Repo Development

- Source artifact: `../source-artifacts/bootstrap/portable-multi-repo-development.md`
- Source set: bootstrap
- Register state: atomized
- Extraction status: complete
- Validation status: entirely unvetted
- Extracted candidates: 390

## Source-document structure

The bootstrap blockquote is represented by:

- `Portable Multi-Repo Bootstrap Snapshot Is Unresolved`

The document title is non-assertive structure.

## Opening goal

The two opening goal paragraphs are represented by:

- `Vapor Multi-Repo Goal Excludes Root Convenience`
- `Multi-Repo Development Is a Normal Vapor Capability`
- `Vapor-Root Is the First Demanding Multi-Repo User`

## Steam-installed control plane

The paragraph beginning “Steam installs Vapor's complete control plane” is
represented by:

- `Steam Installs the Vapor SDK Control-Plane Component`
- `Steam Installs the Vapor Launcher Control-Plane Component`
- `Steam Installs the Vapor Rust Control-Plane Component`
- `Steam Installs the Vapor Cargo Control-Plane Component`
- `Steam Installs the Vapor Git Control-Plane Component`
- `Steam Installs Vapor Service Integrations`
- `Steam Installs Vapor Caches`
- `Steam Installs Vapor Output Facilities`
- `Steam Installs Vapor Operational State`
- `Vapor Opens an External Crate`
- `Vapor Opens an External Project`
- `Vapor Opens an External Declared Project Graph`

The component list was split because each installed component and each allowed
external target is independently assessable.

## Generic Vapor project workflow

The paragraphs beginning “Vapor knows how” and ending “That is the target” are
represented by:

- `Vapor Creates Projects`
- `Vapor Synchronizes Projects`
- `Vapor Changes Projects`
- `Vapor Validates Projects`
- `Vapor Reviews Projects`
- `Vapor Publishes Projects`
- `Vapor Project Work Does Not Require Direct Git Operation`
- `Vapor Project Work Does Not Require Direct GitHub Operation`
- `Vapor Project Work Does Not Require Direct Cargo Operation`
- `Vapor Project Work Does Not Require Direct SteamPipe Operation`
- `Vapor Project Work Does Not Require Direct Workshop Operation`
- `Moved Vapor Applications Reconnect External Projects`
- `Repaired Vapor Applications Reconnect External Projects`
- `Different Multi-Repo Projects Use the Same Vapor Operations`

“That is the target” is a within-source summary of the preceding goal and
workflow candidates and produces no additional candidate.

## A workspace type, not a famous folder

The diagram and both explanatory paragraphs are represented by:

- `Multi-Repo Workspace Includes a Declaration`
- `Multi-Repo Workspace Includes Resolution`
- `Multi-Repo Workspace Includes Local Operation State`
- `Vapor-Root Is a First-Party Platform Graph Example`
- `Content Projects Are Multi-Repo Workspace Examples`
- `Content Project Workspaces May Belong to Any Party`
- `Content Project Workspaces May Produce Workshop Outputs`
- `Another Team May Declare a Multi-Repo Graph`
- `Multi-Repo Workspace Graphs May Combine Any Repositories`
- `Vapor-Root Is One Multi-Repo Workspace Instance`
- `Vapor-Root Is Useful as a Complicated Workspace`
- `Vapor-Root Is Useful as a Controlled Workspace`
- `Vapor-Root Names Must Not Be Built Into the SDK`
- `Vapor-Root Paths Must Not Be Built Into the SDK`
- `Workspaces May Coordinate First-Party Repositories`
- `Workspaces May Coordinate Second-Party Repositories`
- `Workspaces May Coordinate Third-Party Repositories`
- `Workspaces May Build Platform Tools`
- `Workspaces May Build Custom Content`
- `Workspaces May Build Tools and Custom Content Together`
- `Workspace Membership Means Sources Are Developed Together`
- `Workspace Membership Means Sources Are Tested Together`
- `Workspace Membership Does Not Determine Ownership`
- `Workspace Membership Does Not Determine Artifact Distribution`

Diagram connectors, alignment, and column formatting are non-assertive
structure. Diagram labels with substantive workspace meaning were extracted.

## Two example workspaces

The section heading is represented by:

- `Two Example Workspaces Must Shape the Design`

The first-party platform workspace diagram and prose are represented by:

- `Vapor Repository Provides the Shared Foundation`
- `Vapor SDK Authors Platform Work`
- `Vapor SDK Builds Platform Work`
- `Vapor SDK Packages Platform Work`
- `Vapor SDK Publishes Platform Work`
- `Vapor Launcher Installs Platform Work`
- `Vapor Launcher Composes Platform Work`
- `Vapor Launcher Launches Platform Work`
- `Vapor Examples Proves Public Authoring Workflows`
- `Vapor Registry Provides Identity Authority`
- `Vapor Registry Provides Publishing Authority`
- `First-Party Platform Workspace Integrates the Platform`
- `First-Party App-Root Artifacts Use the First-Party Release Path`
- `First-Party Release Uses SteamPipe Where Appropriate`
- `Vapor Registry Is Part of the Intended Platform Workspace`
- `Vapor Registry Is Absent From Current Root Submodules`

The custom-content development workspace diagram and prose are represented by:

- `Loo Cast Development Workspace Is Not Vapor-Root`
- `Spacetime Engine and Loo Cast Build Together`
- `Spacetime Engine and Loo Cast Test Together`
- `Spacetime Engine Artifact Is a Workshop Item`
- `Loo Cast Game Artifact Is a Workshop Item`
- `Spacetime Engine Consumes Vapor`
- `Loo Cast Consumes Vapor`
- `Spacetime Engine Is Not Part of the First-Party Vapor Platform`
- `Loo Cast Is Not Part of the First-Party Vapor Platform`
- `Spacetime Engine Source Is Not a Vapor-Root Member`
- `Loo Cast Source Is Not a Vapor-Root Member`
- `Custom Content Workspace Coordinates Cross-Repo Engine Changes`
- `Custom Content Workspace Coordinates Cross-Repo Game Changes`
- `Custom Content Source Workspace Produces Workshop Artifacts`
- `Installed Workshop Artifacts Exclude Git Repositories`
- `Installed Workshop Artifacts Exclude Branches`
- `Installed Workshop Artifacts Exclude Development Metadata`
- `Two-Repo Two-Item Mapping Is the Intended Project Case`
- `Two-Repo Two-Item Mapping Is Not a Framework Rule`
- `One Repository May Produce Several Workshop Items`
- `One Workshop Item May Require Several Repositories`
- `Two Example Workspaces Are the Minimum Design Test`
- `Root-Only Workspace Models Remain Special Cases`
- `Custom-Content-Only Models Miss Platform Release Needs`

Diagram connectors, alignment, and source-to-artifact arrows are non-assertive
structure. Their substantive labels and relationships were extracted.

## Two sides of a hard wall

The diagram, placement rule, source-side layer list, and concluding paragraph
are represented by:

- `Steam App Contains Vapor-Managed Control Plane`
- `External Side Contains User-Owned Source`
- `Steam App Contains SDK and Launcher`
- `Steam App Contains Vendored Git`
- `Steam App Contains Managed Rust and Cargo`
- `Steam App Contains Service Clients`
- `Steam App Contains Caches Output and Staging`
- `Steam App Contains Runtime Depot and Workshop Artifacts`
- `External Side Contains Vapor Workspace Declaration`
- `External Side Contains Member Source Repositories`
- `External Side Contains Cargo Workspaces and Crates`
- `External Side Contains Project-Local Vapor Schema`
- `Source Placement Wall Is Categorical`
- `Vapor May Place Tools Inside the Steam App`
- `Vapor May Place Generated Output Inside the Steam App`
- `Vapor May Place Published Artifacts Inside the Steam App`
- `Vapor May Place Installed Content Inside the Steam App`
- `Vapor Must Never Create Source Repositories Inside the Steam App`
- `Vapor Must Never Clone Source Repositories Inside the Steam App`
- `Workspace Roots May Live in User Data`
- `Workspace Roots May Live in Explicit External Locations`
- `Vapor Workspace Declares the Multi-Repo Project Graph`
- `Source Repository Carries History`
- `Source Repository Carries Versioning`
- `Source Repository Defines a Project Boundary`
- `Cargo Workspace Coordinates Packages Within Its Owning Repository`
- `Crate Source Is a Buildable and Packageable Unit`
- `Content Source Is a Buildable and Packageable Unit`
- `Built Artifact May Become a Crate Release`
- `Built Artifact May Become an App-Root File`
- `Built Artifact May Become a Workshop Item`
- `Vapor Connects Source-Side Layers`
- `Vapor Hides Source-Side Backend Mechanics`
- `Installed Artifacts Compose Into One Steam Application World`
- `Source Repositories Remain Outside the Steam Application`

Diagram layout and arrows are non-assertive structure.

## Source graphs and distribution graphs are different

The two framing questions, distribution diagram, and explanatory paragraph are
represented by:

- `Multi-Repo Workspace Identifies Revisions Changed Together`
- `Multi-Repo Workspace Identifies Revisions Proven Together`
- `Distribution Identifies Published Artifacts`
- `Distribution Identifies Publication Channels`
- `Distribution Identifies Publication Authority`
- `Repository Builds May Publish to Crates Io or GitHub`
- `Repository Builds May Publish App Packages Through SteamPipe`
- `Repository Builds May Publish Content Through Workshop`
- `Workshop Distribution Produces Installed Content`
- `Shared Development Does Not Require Vapor-Root Membership`
- `Installed Vapor Relationship Uses Workshop Items`
- `Source Workspace Relationship Is a Separate Choice`
- `Submodules May Materialize Workspace Members`

The blockquote formatting, diagram layout, and arrows are non-assertive
structure.

## Identity is not role is not location

The table and explanatory prose are represented by:

- `Workspace Member Identity Dimensions Are Independent`
- `Member Control Is an Independent Dimension`
- `Member Control May Be First Party`
- `Member Control May Be Delegated Second Party`
- `Member Control May Be Third Party`
- `Member Role Is an Independent Dimension`
- `Member Role May Be a Platform Tool`
- `Member Role May Be Content`
- `Member Role May Be Registry Data`
- `Member Source Location Is an Independent Dimension`
- `Member Source May Be a Workspace Member`
- `Member Source May Be a Standalone Checkout`
- `Member Source May Be a Remote Dependency`
- `Member Output Type Is an Independent Dimension`
- `Member Output May Be a Rust Crate`
- `Member Output May Be an Executable`
- `Member Output May Be an App Package`
- `Member Output May Be a Content Artifact`
- `Member Distribution Is an Independent Dimension`
- `Member Distribution May Use Crates Io`
- `Member Distribution May Use GitHub`
- `Member Distribution May Use SteamPipe`
- `Member Distribution May Use Workshop`
- `Member Distribution May Be Local Only`
- `No Workspace Identity Dimension Determines Another`
- `First-Party Repositories May Produce Workshop Examples`
- `Second-Party Engines May Be Workshop Items`
- `Third-Party Projects May Use Multi-Repo Workspaces`
- `Third-Party Workspace Use Does Not Grant First-Party Authority`
- `Second-Party Needs a Precise Definition`
- `Second-Party Likely Means Delegated Authority`
- `Second-Party Authority Is Registry Recorded`
- `Second-Party Authority Has a Bounded Namespace`
- `Second-Party Authority Has a Bounded Publishing Scope`
- `Second-Party Does Not Mean a Vapor-Root Folder`

Table headers, separators, and question punctuation are non-assertive structure.

## Three kinds of workspace truth

The lifecycle diagram, candidate filename examples, and explanatory list are
represented by:

- `Workspace Metadata Has Three Truth Lifecycles`
- `Workspace Declaration Describes Meaning`
- `Workspace Resolution Describes the Exact Passing Graph`
- `Workspace Local State Describes This Machine`
- `Workspace Declaration Contains Members Roles and Requirements`
- `Workspace Declaration May Be Vapor Toml`
- `Workspace Resolution Contains Repository Commits`
- `Workspace Resolution Contains Dependency and Toolchain Pins`
- `Workspace Resolution May Be Vapor Lock`
- `Workspace Local State Contains Paths Caches and Worktrees`
- `Workspace Local State May Use Dot Vapor`
- `Workspace Truth Filenames Are Undecided`
- `Workspace Truth Lifecycle Split Is Important`
- `Workspace Declaration Is Reviewable`
- `Workspace Declaration Can Describe Any Multi-Repo Workspace`
- `Workspace Resolution Reconstructs an Exact Constellation in CI`
- `Workspace Resolution Reconstructs an Exact Constellation on Another Machine`
- `Workspace Local State Maps External Checkouts`
- `Workspace Local State Maps App-Local Tools`
- `Workspace Credentials Use Local or Platform-Secure State`
- `Workspace Credentials Are Not Committed in Declaration`
- `Workspace Credentials Are Not Committed in Resolution`
- `Workspace Truth Classes Do Not Permit App-Internal Repositories`
- `Current Root Submodules Supply Partial Resolution`
- `Current Root Submodules Supply Checkout Behavior`
- `Current Root Submodules Supply Almost No Semantic Declaration`
- `Submodules May Remain a Workspace Backend`
- `Submodules Cannot Be the Whole Workspace Framework`

Diagram layout and arrows are non-assertive structure.

## The real atom is larger than a commit

The two example change trees, cross-repository explanation, change-record list,
and lifecycle diagram are represented by:

- `Meaningful Changes May Cross Repository Boundaries`
- `Portable Workspace Support May Change Vapor`
- `Portable Workspace Support May Change Vapor SDK`
- `Portable Workspace Support May Change Vapor Examples`
- `Portable Workspace Support May Change Vapor Root Resolution`
- `Engine Game Contract Change May Change Spacetime Engine`
- `Engine Game Contract Change May Change Loo Cast Game`
- `Engine Game Contract Change May Change Content Workspace Resolution`
- `Git Cannot Make a Cross-Repo Change One Atomic Commit`
- `GitHub Cannot Make a Cross-Repo Change One Atomic Pull Request`
- `Cross-Repo Non-Atomicity Is Acceptable`
- `Cross-Repo Human Intent Can Be One Change`
- `Cross-Repo Review Remains Repository Local`
- `Cross-Repo Merge Remains Repository Local`
- `Cross-Repo Change Record Stores Overall Intent`
- `Cross-Repo Change Record Stores Participating Repositories`
- `Cross-Repo Change Record Stores Each Repository Part`
- `Cross-Repo Change Record Stores Dependency and Merge Order`
- `Cross-Repo Change Record Stores Passing Integrated Revisions`
- `Cross-Repo Change Record Stores Affected Artifacts and Distribution`
- `Cross-Repo Change Record Stores Lifecycle State`
- `Cross-Repo Change Lifecycle Has an Ordered Progression`
- `Cross-Repo Final Lifecycle States Remain Separate`
- `Merged Engine Changes Are Not Automatically Workshop Releases`

Tree connectors, diagram arrows, and layout are non-assertive structure.

## The development loops — one crate or one repository

The standalone-project subsection is represented by:

- `Vapor Opens External Standalone Projects`
- `Vapor Supplies App-Local Toolchain to Standalone Projects`
- `Vapor Supplies App-Local Environment to Standalone Projects`
- `Standalone Projects Resolve Honest Canonical Dependencies`
- `Standalone Projects Retain Their Own History`
- `Standalone Projects Retain Their Own Version`
- `Standalone Projects Retain Their Own Release Boundary`
- `Vapor Presents Standalone Project Changes`
- `Vapor Presents Standalone Project Checks`
- `Vapor Presents Standalone Project Review`
- `Vapor Presents Standalone Project Publishing`
- `Standalone Workflow Hides Backing Git and GitHub Concepts`
- `Standalone Projects Need No Multi-Repo Workspace`

## The development loops — any multi-repo workspace

The multi-repository subsection and product-level question list are represented
by:

- `Vapor Opens External Multi-Repo Declarations`
- `Vapor Creates or Finds Member Repositories Externally`
- `Vapor Resolves Member Backing Revisions`
- `Vapor Redirects Selected Dependencies to Local Members`
- `Vapor Reports Projects With Changes`
- `Vapor Reports Projects With Conflicts`
- `Vapor Reports Projects With Unpublished Work`
- `Vapor Reports Projects With Remote Updates`
- `Vapor Reports Released Dependencies`
- `Vapor Reports Remotely Resolved Dependencies`
- `Vapor Reports Dependencies Developed in the Workspace`
- `Vapor Reports Workspace Coherence`
- `Vapor Reports Workspace Check or Release Blockers`
- `Workspace Checks Run in Dependency Order`
- `Workspace Checks May Target One Member`
- `Workspace Checks May Target an Affected Subgraph`
- `Workspace Checks May Target the Whole Workspace`
- `Successful Resolution Identifies Exact Working Revisions`
- `Vapor Root Uses the Generic Multi-Repo Development Loop`
- `Loo Cast and Spacetime Use the Generic Multi-Repo Development Loop`
- `Workspace Behavior Differs Through Declarations`
- `Workspace Behavior Differs Through Publishing Authority`

Question punctuation is non-assertive structure; each substantive question was
extracted as its underlying candidate.

## The development loops — promotion and distribution

The promotion subsection, diagram, and concluding distinctions are represented
by:

- `Integration Produces a Known-Good Source Graph`
- `Promotion Produces Versioned Artifacts`
- `Distribution Uses Each Artifact's Real Channel`
- `Vapor Change Workflow Hides Repository Histories`
- `Vapor Change Workflow Hides Remote Reviews`
- `Resolved Multi-Repo Integration Proves Standalone Members`
- `Resolved Multi-Repo Integration Proves the Whole Workspace`
- `Resolved Multi-Repo Integration Proves App-Root Relocation When Relevant`
- `Resolved Multi-Repo Integration Includes Packaging Dry Runs`
- `Versioned Artifacts May Use Crates Io or GitHub Releases`
- `Versioned App-Root Artifacts Use SteamPipe`
- `Versioned Content Artifacts Use Workshop`
- `SteamPipe and Workshop Are Distinct Operations`
- `SteamPipe Updates the Application World`
- `Workshop Introduces Content Into the Application World`
- `Registry Policy May Authorize Publishing Without Merging Pipelines`

Diagram connectors, arrows, and layout are non-assertive structure.

## The non-negotiable placement rule

The placement diagram and explanatory prose are represented by:

- `Downloaded Steam Application Contains Vapor Tools`
- `Downloaded Steam Application Contains the Final Composed Runtime`
- `Steam Application Directory Contains SDK and Launcher`
- `Steam Application Directory Contains Vendored Git`
- `Steam Application Directory Contains Managed Rust and Cargo`
- `Steam Application Directory Contains Runtime Artifacts`
- `Steam Application Directory Contains Caches Output Staging and State`
- `Steam Application Directory Never Contains Source Repositories`
- `Vapor Root Source Lives Externally`
- `Loo Cast Workspace Source Lives Externally`
- `All Vapor Project Source Lives Externally`
- `Vapor Rejects Creating Managed Repositories Inside the App`
- `Vapor Rejects Cloning Managed Repositories Inside the App`
- `Vapor Rejects Importing Managed Repositories Inside the App`
- `Vapor Rejects Relocating Managed Repositories Inside the App`
- `Steam Updates Never Own Source History`
- `Steam Repairs Never Own Source History`
- `Steam Uninstalls Never Own Source History`
- `Steam Verify Files Never Owns Source History`
- `App Portability and Project Portability Are Separate`
- `Vapor App Brings Its Known Toolchain to External Projects`
- `Vapor Known Toolchain Includes Git`
- `Vapor Project Brings Its Schema to Compatible Installations`
- `Vapor Project Brings Its Source Graph to Compatible Installations`
- `Project Reconnection Must Not Depend on an Old App Path`

Tree connectors and layout are non-assertive structure.

## Vapor's interface consumes the backends

The workflow table and explanatory prose are represented by:

- `Vapor Interface Is Not a Thin Wrapper Menu`
- `Vapor Presents Its Own Small Workflow`
- `Vapor Maps Its Workflow Onto Vendored Tools`
- `Vapor Maps Its Workflow Onto Remote Services`
- `Create Workspace Chooses an External Location`
- `Create Workspace Writes the Schema`
- `Create Workspace Initializes Repositories`
- `Create Workspace Creates Authorized GitHub Resources`
- `Add Member Clones or Initializes a Repository`
- `Add Member Configures Remotes`
- `Add Member Manages a Resolved-Member Representation`
- `Start Change Creates Coordinated Branches`
- `Start Change Creates Coordinated Worktrees`
- `Start Change Records the Affected Graph`
- `Save Work May Commit`
- `Synchronize Work May Fetch`
- `Synchronize Work May Merge or Rebase Under Policy`
- `Synchronize Work May Push`
- `Synchronize Work Updates Resolutions`
- `Share for Review Creates Required Pull Requests`
- `Share for Review Connects Required Pull Requests`
- `Share for Review Aggregates Pull Request State`
- `Check Workspace Invokes Cargo and Other Validators`
- `Check Workspace May Use GitHub Actions`
- `Check Workspace Validators Run in Graph Order`
- `Release May Version Artifacts`
- `Release May Tag Repositories`
- `Release May Build Artifacts`
- `Release May Create GitHub and Crate Releases`
- `Release Updates Downstream Resolutions`
- `Publish Drives SteamPipe by Artifact Role and Authority`
- `Publish Drives Workshop by Artifact Role and Authority`
- `Backend Actions Are Not the Normal Vapor UX`
- `Users Work With Vapor Projects`
- `Users Work With Vapor Members`
- `Users Work With Vapor Changes`
- `Users Work With Vapor Checks`
- `Users Work With Vapor Releases`
- `Users Work With Vapor Publications`
- `Git Branches Are Backend Projections`
- `Git Submodules Are Backend Projections`
- `Pull Request Sets Are Backend Projections`
- `VDF Files Are Backend Projections`
- `Service-Specific State Is a Backend Projection`
- `Vapor Owns Strong Backend Conventions`
- `Vapor May Regenerate Managed Backend State`
- `Vapor May Repair Managed Backend State`
- `Vapor Needs Safe Failure Behavior`
- `Vapor Needs Valid Underlying Repositories for Recovery`
- `Vapor Needs Valid Underlying Repositories for Interoperability`
- `Valid Underlying Repositories Do Not Make Raw Git the Product`

Table headers and separators are non-assertive structure. Joined backend actions
were split where each action is independently assessable.

## What must be designed first

The planning-status prose, historical navigation link, concern table, and
sibling-dependency explanation are represented by:

- `Detailed Schema Work Is Not Active Yet`
- `Planning First Captures and Verifies Survival Information`
- `Planning First Captures and Verifies Repository Meaning`
- `Planning First Captures and Verifies Vapor Backend Relationships`
- `Raw Info Dump Correction Was the Directed Next Step`
- `Raw Info Dump Bytes Were the Active Planning Surface`
- `Design Concern Table Is Background Context Only`
- `Workspace Model Must Answer External Placement`
- `Workspace Model Must Answer Membership`
- `Workspace Model Must Answer Resolution`
- `Workspace Model Must Answer Dependency Modes`
- `Workspace Model Must Answer Operation Scope`
- `Workspace Model Must Answer Visible Lifecycle`
- `Workspace Model Must Answer Backend Projection`
- `Workspace Model Must Answer Outputs`
- `Workspace Model Must Answer Local State`
- `Sibling Path Dependencies Are Not a Valid Fourth Dependency Mode`
- `Sibling Path Dependencies Are an Implicit Root Local Override`
- `Sibling Local Override Lives in Canonical Child Configuration`

The historical link target, table headers, and separators are source structure;
the directive and substantive table cells were extracted.

## Planning order from here

The planning-order diagram and concluding prose are represented by:

- `Planning Begins With Atomic Direction Capture`
- `Planning Defines Vapor-Visible Lifecycle and Failure States Second`
- `Planning Defines Composed-Project Schema and Git Projection Third`
- `Planning Defines Source and Distribution Graphs Fourth`
- `Planning Proves App-Local Control Plane and External Projects Fifth`
- `Planning Defines Reconciliation CI and Publication Recovery Sixth`
- `Planning Performs Adversarial Proof and Service Automation Seventh`
- `Detailed Decision Batches Must Not Open Yet`
- `Raw Direction Must Be Complete Before Later Organization`
- `Later Organization Must Reflect the Owner Model`
- `Later Organization Must Not Fill Framework-Generated Gaps`

Diagram connectors and step numbering are non-assertive structure; ordering
language was retained in the candidate text.

## Current evidence

The evidence list is represented by:

- `Root Gitmodules Is the Current First-Party Special-Case Source Set`
- `SDK Core Cargo Manifest Assumes the Special-Case Sibling Layout`
- `Launcher Core Cargo Manifest Assumes the Special-Case Sibling Layout`

The final horizontal rule is non-assertive structure.

## Extraction ambiguities

None.

## Completion

Every source passage is represented by named atomized candidates or explicitly
accounted for as source metadata, a within-source restatement, or non-assertive
structure. Extraction is complete. The unchanged source is retained in planning
evidence and registered as `atomized`. All 390 candidates remain
entirely unvetted.
