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
    

**Answer:** Player, Pack/Mod User, Content/Game/Engine/Mod/Pack developer, vapor ecosystem and toolchain and server developer, and Root Authority, with each role being a superset of the other... if that makes any sense. Like, all a Player can do a Pack/Mod User can also do, and more, yk? And so on.

---

## 1.2 Player

### What is a Player primarily trying to accomplish?

**Answer:** Play the default app composition, or play other official and/or third-party content in the form of finished compositions. The creation of another full app composition (aka a packagepack) is not meant for players. "Content Developers" are the ones who should be developing any kind of content, be it just Packagepacks from pre-existing content, or content of your own, even up to a full on entirely (or majorly so) in-house made packagepack full with custom content. And then there is the "Root Developers" who have actual access to the code and infrastructure of the ecosystem itself. There currently is no hard distinction planned (yet?) to differentiate Content Developers, and similarly no distinction is made between Server Devs and Toolchain Devs and whatever... they're all Root Devs.

### What knowledge can Vapor assume from a Player?

**Answer:** How to use a Computer 101. How to use Steam 101. A bit of common sense.

### What parts of Vapor should a Player normally encounter?

**Answer:** Only the Launcher, and the portion of the launcher that allows searching for and installing and selecting finished compositions, essentially full on games, with the base engine + base game and associated mandatory and/or optional mods being what encompasses the default composition. And ofc the engine-executable/game itself. And the Steam Ecosystem, specifically mostly the launching of an app, and the use of the steam workshop (but only if you want to; vapor launcher can manage all of this, kinda like curseforge is for minecraft).

### What parts should normally remain invisible to them?

**Answer:** Programming. Configs. Out-of-Steam Setup or Experience of any kind. Everything legal here is what you get via steam, without changing the mode of the launcher away from the default, which is plain old "Player" I guess.

---

## 1.3 Mod User

### What distinguishes a Mod User from a normal Player?

**Answer:** N/A because we only have Player, Content Developer, and Ecosystem/Root Developer(Ecosystem Developer can commit code to everything and anything, and Root Developer is just one step further in terms of being the github repo owner and root authority for all of this, which is me, which must ofc be a modelled edge case). Buuut, additionally we maaaaay say that a "Content User" is also like a middleground, this time of Player and Content Developer: Someone who uses Content; to use it simply, or to compose it into any kind of pack, who knows. But true content development of non-pack things like an Engine, a Game, a Mod of any kind, yk, those need a full on Content Developer.

### What should a Mod User be able to accomplish?

**Answer:** N/A (I feel like the questions really needed to quickly take into account new answers and rearrange and change themselves accordingly, which is ofc not possible with this rigid set-in-stone document which I simply cannot send back and forth to you dozens of times.... hmmmmm.)

Lemme try actually: A Mod user should be able to understand and apply the architecture of the content ecosystem to effectively compose existing (and/or self-made) content into a new composition or substituent of a full composition (any pack or individual piece of content, apart from packagepack, which ofc represents the whole full composition).

### What technical knowledge can Vapor assume from them?

**Answer:** Yeah no, we should really completely rework this format, ofc not throwing the answers away I am going to give, but yeah, this (necessarily so) rigid format is really annoying, so please excuse my sub-par answers...

---

## 1.4 Content Developer

### What should a Content Developer be able to accomplish?

**Answer:** Develop actual pieces of content, not just packs of content like packagepack, enginepack, etc., yk? They need to actually do some configuring and programming and stuff, yk? But the nicer friendly "script-y" kind of programming I guess, with guardrails and like.... ikd.

### What technical knowledge can Vapor assume from them?

**Answer:** I think I answered that implicitly via my answers so far.

### What should Vapor automate for them?

**Answer:** Dealing with Rust or Cargo (or other Tools, for example Bevy and it's ECS count as well) complexity, yk? They should be able to just do stuff, and have that feeling of zero boilerplate/unnecessary-setup, yk?

### What should remain directly controllable by them?

**Answer:** Everything but the ecosystem itself, as stated previously; more or less.

---

## 1.5 Engine / Game Developer

### Is an Engine/Game Developer meaningfully different from a normal Content Developer?

**Answer:** No.

### If yes, how?

**Answer:** N/A.

---

## 1.6 Vapor Ecosystem Developer

### What distinguishes someone developing Vapor itself from someone developing Vapor Content?

**Answer:** Well, one thing is developing an application and an app framework and the server (and whatever else) infrastructure around it, and the other is using that stuff as a user of it, yk?

### What additional capabilities or workflows do they need?

**Answer:** Just access to push commits to the official repos.

---

## 1.7 Server / Registry Operator

### Is operating Vapor servers/registries a meaningful user role?

**Answer:** No. One central registry exists, nothing is user-hosted in that sense or planned right now.

### If yes, what should that role be able to accomplish?

**Answer:** N/A.

---

# 2. First Installation and First Launch

Assume:

> Clean machine → Steam → Install Loo Cast → Launch

## 2.1 Installation

### What does Steam install initially?

**Answer:** The Vapor Launcher without SDK/Development capabilities, and the default composition and it's constituents ofc.

### What is bundled directly with the Steam App?

**Answer:** The vapor launcher.

### What is intentionally downloaded or installed later?

**Answer:** The default composition, from the steam workshop, automatically.

---

## 2.2 First Launch

### What executable/process starts first?

**Answer:** The launcher, which allows us to start both the sdk mode which happens inside the launcher I guess, but also normal play "mode" where we just start the actual whatever_engine.exe and tell it to launch whatever game, and yeah, yk?

### What does the user see first?

**Answer:** The three launch options in Steam, one taking them directly to play the default composition (called "Loo Cast" btw, same as the Loo cast Game; weird, I know), the second one proposing the launcher, and the third one proposing jumping straight into the sdk, assuming it is installed. If not, we ofc just expect the user to be guided-into/asked-permission-for changing the mode, which ofc bears real-world consequences, installing the SteamCmd, searching for git or installing it locally in the steam-app/vapor root directory, installing the Rust/Cargo toolchain and configuring it, and so on, yk? Actually, we may just say "go to the installer and go into different mode" cause that seems like an installer responsibility, and then the installer is there instead of the weird sdk option (weird, cause the SDK IS inside the Launcher, yk? It's the same application, just in different mode!) we have an "Installer" option there, making that in total: "Play Loo Cast", "Start Vapor", "Start Installer".

### Is there any first-time setup?

**Answer:** Only if you want to become a content user or even developer, but not as a normal player, no.

### What does Vapor automatically detect on the machine?

**Answer:** As Player: Detect it's own presence directly inside the root directory of a steam app with the correct steam id, and the ownership of the game (crude steamworks api drm basically).
As Content User: Everything before, and: Git capabilities (cause vendoring this is not viable everywhere due to linux shenanigans).
As Content Developer: Everything before, and: A working logged-in-and-to-the-steam-account-linked github account! (for issues and in-app/in-sdk content *deployment*).
As Ecosystem/Root Developer: Everything before, and: Github authorization for the official/first-party github repos.

### What does Vapor automatically configure?

**Answer:** As Player: Itself, as in it should be automatically configured by the time the launcher opens up, poteeeeeentially requiring some first-launch setup, but that should then be fully automatic and opaque, simply being enforced and yeah.
As Content User: Everything before, and: A rust/cargo toolchain, and steamcmd.
As Content Developer: Everything before, and: Well, semi-automatically (cause you as the user need to explicitly decide): Try and load/clone/whatever some existing superworkspace/workspace/repo/whatever-idk, or create a default/from-scratch one.
As Ecosystem/Root Developer: Everything before, and the means to actually push/deploy actual changes as a sort of pre-release-dev-tool yk? Just a mechanism or wrapped/internally-used cargo xtask or whatever so we can immediately commit+push+live-deploy-on-a-dev-branch-ofc some changes and immediately test them, live, real, kiiiiinda dangerous ofc, but hence a pre-release tool for when there is no audience to witness my mistakes yet. 

---

## 2.3 Default Composition

### What is the default App Composition?

**Answer:** The "Loo Cast Packagepack", comprised of the "Spacetime Engine" and the "Loo Cast Game" and anything I may add in the future.

### How is that default composition selected/resolved?

**Answer:** Well, every "*pack" is basically primarily just a manifest for content and/or certain other types of packs. This way we can just select a composition by stating a packagepack ID, then having the registry check which workshop items that is, but yeah we just ask the registry what each content ID resolves into a steam workshop item id, yk? But ofc the actual existence and validity of a packagepack is guaranteed once it is published, cause it cannot be with invalid declared dependencies or stuff like that, yk? But yeah, primarily the "vapor registry" server-side-app is what holds the database of content ids and associated steam workshop ids (and same with the  steam account id and linked github account id) and can translate the two between each other on demand.

### Is the default content already installed?

**Answer:** No.

### If not, what happens?

**Answer:** It get's automatically installed on first launch, ideally on install but that is not guaranteed (yet?

---

## 2.4 Starting the Game

### What does the Player actually do to start playing?

**Answer:** Either select the default packagepack/"app compositon" (or another one ofc), and hit "Play", or just go directly to the default via the steam launch option "Play Loo Cast".

### What happens between pressing "Play" and the Engine/Game starting?

**Answer:**Nothing. Literally by definition nothing, or virtually/basically nothing, but yeah: Basically nothing afaik.

### Which parts of this process should remain invisible during normal operation?

**Answer:** N/A.

---

## 2.5 Subsequent Launches

### What changes after the first successful launch?

**Answer:** First-launch stuff is done, duh.  

### What state should Vapor remember?

**Answer:** Rust/Cargo toolchain stuff for dependency caching and incremental builds for when you do content composition and/or development (cause changing a pack or content of any kind mandates a rebuild, ALWAYS! As else it won't work as one big static units yk?). Also savegame state ofc, like savegames, quicksaves, yk what I mean. Also, authorization to the degree that the third-party providers like github or steam, and also within reason of course assuming someone allows unreasonable things. Ofc also like the composition installations, and also (if u are a developer or content user/"composer") the raw source code of all the packs and content so it can all be rebuilt from source. Like, we have a content/pack cache, and we have deployed finished bundles, one bundle just being like an engine exe and a statically-linked monolithic hyper-optimized native goooood ol' static lib, yk? Also ofc user/config state for launcher and engine/game, like I mean stuff like audio video whatever settings, or like UI settings, keybinds, remembered window layouts, whatever, that kinda stuff.

### Should subsequent launches normally go directly into the Game, through the Launcher, or somewhere else?

**Answer:** The three paths are always the same: Install, Start Launcher, Play Default Composition (Play Loo Cast).

---

# 3. App Structure and User-Facing Surfaces

## 3.1 The App

### Conceptually, what is the "Loo Cast" App?

**Answer:** The Spacetime Engine executable with the Loo Cast Game library *and/or* any other first-party content.

### Is the Launcher a separate application, a mode, a window, or merely a conceptual role?

**Answer:** A separate application, the pendant to the Installer and the freely-selectable engine executable, and also which game that executable shall launch.

### Is the SDK a separate application, a mode, a collection of tools, or merely a conceptual role?

**Answer:** Aaaalmost a separate application: It's a mode that is basically just a collection of tools like an IDE more or less.

### How do Launcher, SDK, Game, and ecosystem tooling relate from the user's perspective?

**Answer:** What the fuck kinda generic ass question is that man.... sorry, no offense!

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
    

**Answer:** Compositions(Play as a button per composition or for the selected composition one button, something like that), Content/Library, Development (Projects, Toolchain/SDK/"IDE", Diagnostics, Logs), Settings, Accounts.

### What is the normal "home" surface?

**Answer:** A list of tabs/buttons in one big menu: Compositions, Content/Library, Development, Settings, Accounts.

### Which surfaces should ordinary Players never need?

**Answer:** "Non-modding"? Compositions, Content/Library, Development, Settings.

### Which surfaces appear only in Developer Mode?

**Answer:** For "modders" only, so content user/"composer" (which are weirdly indeed one and the same role really): Compositions and Content/Library.
For real content or otherwise developers: Compositions, Content/Library, and Development.
But really, this is all a bit simplistic and not super crazy thought-through; just saying: Head up!

---

## 3.3 Navigation

### How should users move between the major parts of Vapor?

**Answer:** Installation-wise(includes user-mode switch)? Using the installer. Content-wise? Using the launcher. And the engine/game, idk I think you get it by now, right?

### Should the distinction between Player and Developer workflows be visually strong or mostly seamless?

**Answer:** Visually strong, indeed. A real like visual upgrade really, for lack of a better less vibe-based answer.

---

## 3.4 Command-Line Experience

### Should Vapor expose a CLI?

**Answer:** Yes. This is actually soooomewhat in development, but also an unplanned undocumented spontaneous clusterfuck.

### If yes, who is it intended for?

**Answer:** Developers, but not mere Content Users or "Composers", only Content and Ecosystem/Root Developers.

### Should GUI and CLI expose roughly equivalent capabilities?

**Answer:** Yes, just presented and interacted with very differently ofc.

### Which workflows should deliberately remain CLI-first or GUI-first?

**Answer:** Uuuuuggggggggggggggh- next question! (I have no idea man)

---

# 4. App Instances

## 4.1 Creation

### How is an App Instance created?

**Answer:** it is downloaded and installed and configured and used via steam.

### Is the Steam installation automatically one App Instance?

**Answer:** Yes.

### Can users manually create additional App Instances?

**Answer:** No.

### Why would someone want multiple App Instances?

**Answer:** Don't know, don't care. Makes no sense. it's like asking why someone would want to install steam twice, or edge, or firefox, or anything for that matter really, lol.

---

## 4.2 Identity

### How is an App Instance identified?

**Answer:** Steam ID, and contents and structure of contents, it's a bit complex-

### Does it have a human-readable name?

**Answer:** Yes ofc, all IDs here are supposed to be human-readable plaintext-strings-IDs.

### Does it have a stable internal ID?

**Answer:** Uuuggggggggh- next question!

### Is its filesystem path part of its identity?

**Answer:** I- next question! (Sorry man....)

---

## 4.3 Location

### Where may an App Instance live?

**Answer:** Only inside the direct folder assigned by steam to contain the one main-steamapp's files and folders.

### Can it be moved?

**Answer:** Yes, it will just detect that, live, and ask for permission for an automatic port/migration or something like that, idk really.

### What happens when the user manually moves its directory?

**Answer:** Should not be done. Move it through ""steam's move feature"", which I think steam actually has. Manual edits and moves are NOT recommended in this scope/regard!

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