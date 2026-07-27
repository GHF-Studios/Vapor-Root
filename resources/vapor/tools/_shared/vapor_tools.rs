#![allow(dead_code, unused_variables, clippy::collapsible_if)]

use std::{
    env, fs,
    io::{Write, stdin, stdout},
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

pub const APP_ROOT_ENV: &str = "LOO_CAST_APP_ROOT";
pub const SUPER_MANIFEST: &str = "SuperWorkspace.vapor.toml";
pub const APP_MANIFEST: &str = "App.vapor.toml";
pub const APP_SOURCE_MANIFEST: &str = "App-Source.vapor.toml";
pub const STEAM_APP_ID: &str = "2122620";
pub const STEAM_APP_DIR_NAME: &str = "Loo Cast";
pub const RUST_TOOLCHAIN: &str = "1.97.0";
pub const ZIG_VERSION: &str = "0.16.0";
pub const LLVM_MINGW_VERSION: &str = "20260616";
pub const SOURCES_DIR: &str = "sources";

const STEAMCMD_LINUX: &str =
    "https://steamcdn-a.akamaihd.net/client/installer/steamcmd_linux.tar.gz";
const STEAMCMD_WINDOWS: &str = "https://steamcdn-a.akamaihd.net/client/installer/steamcmd.zip";
const RUSTUP_INIT_X86_64_LINUX: &str =
    "https://static.rust-lang.org/rustup/dist/x86_64-unknown-linux-gnu/rustup-init";
const RUSTUP_INIT_AARCH64_LINUX: &str =
    "https://static.rust-lang.org/rustup/dist/aarch64-unknown-linux-gnu/rustup-init";
const RUSTUP_INIT_X86_64_WINDOWS: &str =
    "https://static.rust-lang.org/rustup/dist/x86_64-pc-windows-gnu/rustup-init.exe";
const ZIG_X86_64_LINUX: &str = "https://ziglang.org/download/0.16.0/zig-x86_64-linux-0.16.0.tar.xz";
const ZIG_X86_64_WINDOWS: &str =
    "https://ziglang.org/download/0.16.0/zig-x86_64-windows-0.16.0.zip";
const LLVM_MINGW_X86_64_LINUX: &str = "https://github.com/mstorsjo/llvm-mingw/releases/download/20260616/llvm-mingw-20260616-msvcrt-ubuntu-22.04-x86_64.tar.xz";
const LLVM_MINGW_X86_64_LINUX_SHA256: &str =
    "a1f7968b48ba8d949194d6dee6c76f3cd0f61cba91658599af2c2c834a55ab87";
const LLVM_MINGW_X86_64_WINDOWS: &str = "https://github.com/mstorsjo/llvm-mingw/releases/download/20260616/llvm-mingw-20260616-msvcrt-x86_64.zip";
const LLVM_MINGW_X86_64_WINDOWS_SHA256: &str =
    "744809646fdefe24a357399788d68fb07ecc65fa0be71baa2406793ce25c9813";

const RUST_TARGETS: &[&str] = &["x86_64-unknown-linux-gnu", "x86_64-pc-windows-gnullvm"];
const RUST_COMPONENTS: &[&str] = &["rustfmt", "clippy", "rust-src"];
const PLAYER_DIRS: &[&str] = &[
    ".vapor/state",
    ".vapor/state/installer",
    ".vapor/logs",
    ".vapor/diagnostics/runs",
    ".vapor/downloads",
    ".vapor/extract",
    "content/cache/packages",
    "content/installed",
    "content/workshop/downloads",
    "tools",
];

const PLAYER_TEARDOWN_PATHS: &[&str] = &[
    "tools/git",
    "tools/steamcmd",
    ".vapor/registry",
    ".vapor/downloads",
    ".vapor/extract",
    ".vapor/state",
    ".vapor/diagnostics",
    ".vapor/logs",
    "content/cache",
    "content/installed",
    "content/workshop/downloads",
    "output",
];

const PLAYER_EMPTY_PARENT_DIRS: &[&str] = &["content/workshop", "content", "tools", ".vapor"];

#[derive(Debug, Clone)]
struct ComponentStatus {
    label: &'static str,
    ready: bool,
    path: PathBuf,
    missing: Vec<String>,
}

#[derive(Debug, Clone)]
struct CargoProjectRegistration {
    manifest: PathBuf,
}

#[derive(Debug, Clone)]
struct RunConfiguration {
    name: &'static str,
    folder: &'static str,
    file_stem: &'static str,
    kind: RunConfigurationKind,
    command: &'static str,
    working_directory: &'static str,
}

#[derive(Debug, Clone, Copy)]
struct SourcePreset {
    name: &'static str,
    role: &'static str,
    remote: &'static str,
    branch: &'static str,
}

const SOURCE_PRESETS: &[SourcePreset] = &[
    SourcePreset {
        name: "Vapor-Root",
        role: "app-source-root",
        remote: "https://github.com/GHF-Studios/Vapor-Root.git",
        branch: "main",
    },
    SourcePreset {
        name: "Loo-Cast",
        role: "first-party-content-workspace",
        remote: "https://github.com/GHF-Studios/Loo-Cast.git",
        branch: "main",
    },
    SourcePreset {
        name: "Vapor-Registry",
        role: "registry-authority",
        remote: "https://github.com/GHF-Studios/Vapor-Registry.git",
        branch: "main",
    },
    SourcePreset {
        name: "Vapor-Server-Root",
        role: "server-orchestration",
        remote: "https://github.com/GHF-Studios/Vapor-Server-Root.git",
        branch: "main",
    },
];

#[derive(Debug, Clone, Copy)]
enum RunConfigurationKind {
    Cargo,
    Shell,
    Konsole,
}

const GENERATED_RUN_CONFIGURATION_STEMS: &[&str] = &[
    "00_Initial_Setup_Patch_RustRover",
    "00_Initial_Setup_Check_App_Environment",
    "00_Initial_Setup_Install_Developer_Tools",
    "10_Source_Workspaces_Check_Source_Checkout",
    "10_Source_Workspaces_Clone_Vapor_Root",
    "10_Source_Workspaces_Clone_Loo_Cast",
    "10_Source_Workspaces_Clone_Vapor_Registry",
    "10_Source_Workspaces_Clone_Vapor_Server_Root",
    "10_Source_Workspaces_Create_Basic_Content_Workspace",
    "20_Development_Workflows_Open_Vapor_Shell",
    "20_Development_Workflows_Check_App_Binaries",
    "20_Development_Workflows_Check_Launcher",
    "20_Development_Workflows_Check_Content",
    "20_Development_Workflows_Build_App",
    "20_Development_Workflows_Deploy_App",
    "20_Development_Workflows_Build_Content",
    "20_Development_Workflows_Deploy_Content",
    "30_Publishing_Publish_App",
    "30_Publishing_Publish_Content",
    "20_Development_Workflows_Deploy_Content_Locally",
    "20_Development_Workflows_Build_App_Locally",
    "30_Publish_Previews_Preview_Root_Publish",
    "30_Publish_Previews_Preview_Content_Publish",
    "00_IDE_Patch_RustRover",
    "01_Environment_Status",
    "02_Environment_Setup_Developer_Tools_Konsole",
    "10_Source_Status",
    "11_Source_Init_Basic_Content_Konsole",
    "20_Shell_Interactive_Konsole",
    "30_Launcher_Status",
    "40_Content_Status",
    "41_Content_Deploy_Artifact_Konsole",
    "50_Root_Build_Host_Konsole",
    "60_Publish_Root_Preview_Konsole",
    "61_Publish_Content_Preview_Konsole",
    "Cargo_Check_Loo_Cast",
    "Cargo_Check_Vapor_Launcher",
    "Cargo_Check_Vapor_SDK",
    "Cargo_Check_Vapor_Shell",
    "Server_Diagnostics_Local_Konsole",
    "Server_Identity_Local_Konsole",
    "Vapor_Content_Publish_Dry_Run_Konsole",
    "Vapor_Identity_Operator_Shell_Konsole",
    "Vapor_Launcher_Help",
    "Vapor_Launcher_Status",
    "Vapor_Root_Publish_Dry_Run_Konsole",
    "Vapor_Setup_Development_Setup_Konsole",
    "Vapor_Setup_Player_Setup_Konsole",
    "Vapor_Shell_Check_All",
    "Vapor_Shell_Content_Status",
    "Vapor_Shell_Interactive_Konsole",
    "Vapor_Shell_Metadata",
    "Vapor_Shell_Source_Status",
    "Vapor_Tools_App_Status",
    "Vapor_Tools_Patch_RustRover",
    "Vapor_Tools_Source_Status",
];

pub fn exit(result: Result<(), String>) {
    if let Err(error) = result {
        eprintln!("ERROR: {error}");
        std::process::exit(1);
    }
}

pub fn status_main() -> Result<(), String> {
    let app_root = resolve_app_root(arg_path("--app-root"))?;
    println!("Vapor App Tool Status");
    println!();
    println!("App root");
    println!("  {}", app_root.display());
    if let Ok(super_root) = super_workspace_root() {
        println!("Super workspace");
        println!("  {}", super_root.display());
    }
    println!();
    print_component(&directories_status(&app_root));
    print_component(&steamcmd_status(&app_root));
    print_component(&rust_status(&app_root));
    print_component(&cross_status(&app_root));
    println!();
    println!("Next");
    if !steamcmd_status(&app_root).ready {
        println!(
            "  {} --app-root {}",
            tool_command("production/app_setup/setup_player.rs", Some(&app_root)),
            shell_arg(&app_root)
        );
    } else if !rust_status(&app_root).ready || !cross_status(&app_root).ready {
        println!(
            "  {} --app-root {}",
            tool_command("production/app_setup/setup_development.rs", Some(&app_root)),
            shell_arg(&app_root)
        );
    } else {
        println!(
            "  {}",
            tool_command("development/ide_setup/patch_rustrover.rs", Some(&app_root))
        );
    }
    Ok(())
}

pub fn setup_player_main() -> Result<(), String> {
    let app_root = resolve_app_root(arg_path("--app-root"))?;
    setup_player(&app_root)
}

pub fn setup_development_main() -> Result<(), String> {
    let app_root = resolve_app_root(arg_path("--app-root"))?;
    setup_development(&app_root)
}

pub fn teardown_player_main() -> Result<(), String> {
    let app_root = resolve_app_root(arg_path("--app-root"))?;
    teardown_player(&app_root)
}

pub fn teardown_development_main() -> Result<(), String> {
    let app_root = resolve_app_root(arg_path("--app-root"))?;
    teardown_development(&app_root)
}

pub fn post_install_main() -> Result<(), String> {
    let app_root = resolve_app_root(arg_path("--app-root").or_else(candidate_app_root_from_exe))?;
    write_app_root_anchor(&app_root)?;
    println!("Post Install");
    println!();
    println!("App root anchor");
    println!("  {APP_ROOT_ENV}={}", app_root.display());
    println!("  {}", app_root_anchor_file().display());
    Ok(())
}

pub fn pre_uninstall_main() -> Result<(), String> {
    let removed = remove_app_root_anchor()?;
    println!("Pre Uninstall");
    println!();
    println!(
        "App root anchor: {}",
        if removed { "removed" } else { "already absent" }
    );
    Ok(())
}

pub fn create_superworkspace_main() -> Result<(), String> {
    let app_root = resolve_app_root(arg_path("--app-root").or_else(candidate_app_root_from_exe))?;
    let root = arg_path("--path")
        .or_else(positional_path)
        .ok_or_else(|| "usage: create.rs --path /path/to/SuperWorkspace".to_owned())?;
    let root = absolute_from_current(root)?;
    if root.exists() && !root.is_dir() {
        return Err(format!(
            "super-workspace target exists and is not a directory: {}",
            root.display()
        ));
    }
    fs::create_dir_all(&root).map_err(io("create super-workspace", &root))?;
    let manifest = root.join(SUPER_MANIFEST);
    if manifest.exists() {
        return Err(format!(
            "super-workspace already exists: {}\nrefusing to rewrite it",
            manifest.display()
        ));
    }
    let sources = root.join(SOURCES_DIR);
    fs::create_dir_all(&sources).map_err(io("create super-workspace sources", &sources))?;
    let name = root
        .file_name()
        .map(|name| name.to_string_lossy().into_owned())
        .filter(|name| !name.trim().is_empty())
        .unwrap_or_else(|| "vapor-superworkspace".to_owned());
    write(&manifest, &superworkspace_manifest(&name, &app_root, &[]))?;
    println!("SuperWorkspace created");
    println!();
    println!("root:    {}", root.display());
    println!("manifest: {}", manifest.display());
    println!("sources: {}", sources.display());
    println!("app root: {}", app_root.display());
    println!();
    println!("Next");
    println!(
        "  {} --super-workspace {} SOURCE_NAME",
        tool_command("development/source_setup/clone.rs", Some(&app_root)),
        shell_arg(&root)
    );
    println!("  clone only the source you want to work on");
    println!(
        "  {} --super-workspace {}",
        tool_command("development/ide_setup/patch_rustrover.rs", Some(&app_root)),
        shell_arg(&root)
    );
    Ok(())
}

pub fn clone_source_main() -> Result<(), String> {
    let app_root = resolve_app_root(arg_path("--app-root").or_else(candidate_app_root_from_exe))?;
    let super_root = super_workspace_root()?;
    if has_flag("--all") {
        return Err(
            "bulk source cloning is intentionally unsupported; run clone.rs once per source name"
                .to_owned(),
        );
    }
    let requested = positional_values();
    if requested.len() != 1 {
        return Err(format!(
            "usage: clone.rs --super-workspace /path/to/SuperWorkspace SOURCE_NAME\nknown sources: {}",
            SOURCE_PRESETS
                .iter()
                .map(|source| source.name)
                .collect::<Vec<_>>()
                .join("|")
        ));
    }
    let source = source_preset(&requested[0])?;
    let sources_root = super_root.join(SOURCES_DIR);
    fs::create_dir_all(&sources_root)
        .map_err(io("create source clone directory", &sources_root))?;
    clone_source(&sources_root, source)?;
    record_source_clone(&super_root, &app_root, source)?;
    println!();
    println!("Source clone ready under {}", sources_root.display());
    Ok(())
}

pub fn ide_run_main() -> Result<(), String> {
    let workflow = arg_value("--workflow")
        .ok_or_else(|| "usage: run.rs --workflow WORKFLOW --super-workspace PATH".to_owned())?;
    let app_root = resolve_app_root(arg_path("--app-root").or_else(candidate_app_root_from_exe))?;
    let super_root = super_workspace_root()?;
    let workdir = arg_path("--workdir").unwrap_or_else(|| app_root.clone());
    let config_name = arg_value("--config-name").unwrap_or_else(|| workflow.clone());
    let open_konsole = arg_value("--open-konsole").is_some_and(|value| value == "true");

    if open_konsole && env::var_os("VAPOR_RUN_INSIDE_KONSOLE").is_none() {
        if let Some(script) = find_tool_script("development/ide_run/run.rs", Some(&app_root)) {
            if command_exists("konsole") {
                let mut command = Command::new("konsole");
                command
                    .arg("--workdir")
                    .arg(&workdir)
                    .arg("-e")
                    .arg("rust-script")
                    .arg("--force")
                    .arg(script)
                    .args(env::args_os().skip(1))
                    .env("VAPOR_RUN_INSIDE_KONSOLE", "1");
                let status = command
                    .status()
                    .map_err(|error| format!("failed to open Konsole: {error}"))?;
                return if status.success() {
                    Ok(())
                } else {
                    Err(format!("Konsole workflow exited with {status}"))
                };
            }
        }
        println!("Konsole is not available; running inside the current terminal.");
    }

    let host_target = host_rust_target().unwrap_or("x86_64-unknown-linux-gnu");
    let app_bin_dir = app_root.join("bin").join(host_target);
    let vapor = app_bin_dir.join(executable("vapor"));
    let installer = app_bin_dir.join(executable("vapor-installer"));
    let entrypoint = app_bin_dir.join(executable("vapor-entrypoint"));

    set_env_var(APP_ROOT_ENV, &app_root);
    set_env_var("RUSTUP_HOME", app_root.join("rustup-home"));
    set_env_var("CARGO_HOME", app_root.join("cargo-home"));
    prepend_path(&[
        app_bin_dir.clone(),
        app_root
            .join("rustup-home/toolchains")
            .join(format!("{RUST_TOOLCHAIN}-{host_target}"))
            .join("bin"),
        app_root.join("rustup/bin"),
        app_root.join("tools/zig"),
        app_root.join("tools/llvm-mingw/bin"),
    ]);

    println!("== {config_name} ==");
    println!("workspace: {}", super_root.display());
    println!("app root:  {}", app_root.display());
    println!("app bin:   {}", app_bin_dir.display());
    println!();

    env::set_current_dir(&workdir)
        .map_err(|error| format!("failed to enter {}: {error}", workdir.display()))?;

    match workflow.as_str() {
        "app-status" => run_tool("production/app_setup/status.rs", &app_root, &[]),
        "setup-development" => {
            println!(
                "This installs/reconciles app-local Rust/Cargo, Zig, llvm-mingw, and app-root tools."
            );
            let answer = prompt("Proceed with setup_development.rs? [y/N] ")?;
            if !is_yes(&answer) {
                println!("cancelled");
                return Ok(());
            }
            run_tool(
                "production/app_setup/setup_development.rs",
                &app_root,
                &["--app-root", &app_root.to_string_lossy()],
            )
        }
        "source-status" => run_tool(
            "development/source_setup/status.rs",
            &app_root,
            &["--super-workspace", &super_root.to_string_lossy()],
        ),
        workflow if workflow.starts_with("source-clone:") => {
            let source_name = workflow
                .strip_prefix("source-clone:")
                .ok_or_else(|| format!("invalid source clone workflow: {workflow}"))?;
            let source = source_preset(source_name)?;
            println!("Clone one source into the SuperWorkspace.");
            println!("  source: {}", source.name);
            println!("  remote: {}", source.remote);
            println!(
                "  target: {}",
                super_root.join(SOURCES_DIR).join(source.name).display()
            );
            println!();
            let answer = prompt("Clone this source now? [y/N] ")?;
            if !is_yes(&answer) {
                println!("cancelled");
                return Ok(());
            }
            run_tool(
                "development/source_setup/clone.rs",
                &app_root,
                &[
                    "--super-workspace",
                    &super_root.to_string_lossy(),
                    source.name,
                ],
            )
        }
        "source-init-basic-content" => {
            require_executable(&vapor, "app-local Vapor shell")?;
            println!("Create a new source workspace from the basic content template.");
            println!("This writes only to the target path you provide.");
            let target_path = prompt("Target path: ")?;
            if target_path.trim().is_empty() {
                println!("cancelled");
                return Ok(());
            }
            let organization = prompt("Organization slug [ghf-studios]: ")?;
            let organization = if organization.trim().is_empty() {
                "ghf-studios".to_owned()
            } else {
                organization
            };
            let workspace_name = prompt("Workspace name: ")?;
            if workspace_name.trim().is_empty() {
                println!("cancelled");
                return Ok(());
            }
            run_status(Command::new(&vapor).args([
                "source",
                "init",
                "basic-content",
                target_path.trim(),
                "--organization",
                organization.trim(),
                "--name",
                workspace_name.trim(),
            ]))
        }
        "vapor-shell-interactive" => {
            require_executable(&vapor, "app-local Vapor shell")?;
            println!(
                "Opening the app-local Vapor shell. Use source commands to choose the active source workspace."
            );
            run_status(&mut Command::new(&vapor))
        }
        "app-binaries-status" => {
            require_executable(&vapor, "app-local Vapor shell")?;
            require_executable(&installer, "app-local Vapor installer")?;
            require_executable(&entrypoint, "app-local Vapor entrypoint")?;
            println!("App-local binaries:");
            println!("  vapor:           {}", vapor.display());
            println!("  vapor-installer: {}", installer.display());
            println!("  vapor-entrypoint: {}", entrypoint.display());
            println!();
            run_status(Command::new(&vapor).arg("installation"))?;
            run_status(Command::new(&vapor).arg("binaries"))
        }
        "content-status" => {
            require_executable(&vapor, "app-local Vapor shell")?;
            run_status(Command::new(&vapor).args(["content", "status"]))
        }
        "content-build-host" => {
            require_executable(&vapor, "app-local Vapor shell")?;
            println!("Build the active content workspace into app-root build outputs.");
            run_status(Command::new(&vapor).args(["content", "build", "--host-only"]))
        }
        "content-deploy-artifact" => {
            require_executable(&vapor, "app-local Vapor shell")?;
            println!("Build and publish content locally into the installed app root.");
            run_status(Command::new(&vapor).args(["content", "list"]))?;
            let artifact = prompt("\nArtifact to publish locally: ")?;
            if artifact.trim().is_empty() {
                println!("cancelled");
                return Ok(());
            }
            let select_answer = prompt("Select deployed packagepack if applicable? [y/N] ")?;
            let mut command = Command::new(&vapor);
            command.args(["content", "deploy", artifact.trim(), "--host-only"]);
            if is_yes(&select_answer) {
                command.arg("--select");
            }
            run_status(&mut command)
        }
        "root-build-host" => {
            require_executable(&vapor, "app-local Vapor shell")?;
            println!("Build the app/root host target into app-root build outputs.");
            run_status(Command::new(&vapor).args(["root", "build", "--host-only"]))
        }
        "root-deploy-host" => {
            require_executable(&vapor, "app-local Vapor shell")?;
            println!(
                "Publish already-built app/root host-target outputs locally into the installed app root."
            );
            run_status(Command::new(&vapor).args(["root", "deploy", "--host-only", "--skip-docs"]))
        }
        "root-publish" => {
            require_executable(&vapor, "app-local Vapor shell")?;
            println!("Publish the Steam app/depot through the app-local Vapor shell.");
            let account = prompt("Steam build account: ")?;
            if account.trim().is_empty() {
                println!("cancelled");
                return Ok(());
            }
            let branch = prompt("Steam beta branch [manifest default]: ")?;
            let description = prompt("Build description [Vapor development build]: ")?;
            let description = if description.trim().is_empty() {
                "Vapor development build".to_owned()
            } else {
                description
            };
            let confirmation = prompt("\nType publish-app to upload the app/depot now: ")?;
            if confirmation.trim() != "publish-app" {
                println!("cancelled");
                return Ok(());
            }
            let mut command = Command::new(&vapor);
            command.args([
                "root",
                "publish",
                "--account",
                account.trim(),
                "--description",
                description.trim(),
            ]);
            if !branch.trim().is_empty() {
                command.args(["--branch", branch.trim()]);
            }
            run_status(command.arg("--yes"))
        }
        "content-publish" => {
            require_executable(&vapor, "app-local Vapor shell")?;
            println!("Publish one or more content artifacts through the app-local Vapor shell.");
            run_status(Command::new(&vapor).args(["content", "list"]))?;
            let artifact_line = prompt("\nArtifact(s) to publish, separated by spaces: ")?;
            let artifacts = artifact_line
                .split_whitespace()
                .map(str::to_owned)
                .collect::<Vec<_>>();
            if artifacts.is_empty() {
                println!("cancelled");
                return Ok(());
            }
            let account = prompt("Steam account: ")?;
            if account.trim().is_empty() {
                println!("cancelled");
                return Ok(());
            }
            let change_note = prompt("Change note [Vapor content update]: ")?;
            let change_note = if change_note.trim().is_empty() {
                "Vapor content update".to_owned()
            } else {
                change_note
            };
            let confirmation = prompt("\nType publish-content to upload Workshop content now: ")?;
            if confirmation.trim() != "publish-content" {
                println!("cancelled");
                return Ok(());
            }
            let mut command = Command::new(&vapor);
            command.arg("content").arg("publish");
            for artifact in artifacts {
                command.arg(artifact);
            }
            command.args([
                "--account",
                account.trim(),
                "--change-note",
                change_note.trim(),
                "--yes",
            ]);
            run_status(&mut command)
        }
        other => Err(format!("unknown app-root IDE workflow: {other}")),
    }
}

pub fn patch_rustrover_main() -> Result<(), String> {
    let super_root = super_workspace_root()?;
    let app_root = resolve_app_root(arg_path("--app-root"))?;
    let rust = rust_status(&app_root);
    if !rust.ready {
        return Err(format!(
            "app-local Rust/Cargo is not ready at {}\nmissing:\n  - {}\nrun: {} --app-root {}",
            rust.path.display(),
            rust.missing.join("\n  - "),
            tool_command("production/app_setup/setup_development.rs", Some(&app_root)),
            shell_arg(&app_root)
        ));
    }
    let rust_bin = rust.path;
    let cargo = rust_bin.join(executable("cargo"));
    let rustc = rust_bin.join(executable("rustc"));
    let rustup = app_root.join("rustup/bin").join(executable("rustup"));
    let stdlib = rust_stdlib_source(&rustc)?;
    let cargo_projects = discover_cargo_project_registrations(&super_root)?;
    let run_configurations = rustrover_run_configurations();
    let ide_runner = app_root.join("resources/vapor/tools/development/ide_run/run.rs");
    if !ide_runner.is_file() {
        return Err(format!(
            "app-root IDE runner is missing: {}\nrefresh the installed app-root tool payload from Vapor-Root before patching RustRover",
            ide_runner.display()
        ));
    }
    let idea = super_root.join(".idea");
    fs::create_dir_all(&idea).map_err(io("create IDE directory", &idea))?;
    let toolchain_shim = install_rustrover_toolchain_shim(&idea, &app_root, &rust_bin, &rustup)?;

    write(
        &idea.join("cargoProjects.xml"),
        &cargo_projects_xml(&cargo_projects),
    )?;
    write(
        &idea.join("Loo-Cast-Repos.iml"),
        &module_xml(&super_root, &cargo_projects),
    )?;
    write(
        &idea.join("rust.xml"),
        &rust_xml(&toolchain_shim.bin, &stdlib),
    )?;
    write(
        &idea.join("vapor.xml"),
        &vapor_xml(
            &super_root,
            &app_root,
            &cargo,
            &rustc,
            &rustup,
            &stdlib,
            &toolchain_shim,
        ),
    )?;
    patch_workspace_xml(
        &idea.join("workspace.xml"),
        &cargo_projects_component(&cargo_projects),
        &rust_workspace_component(&toolchain_shim.bin, &stdlib),
        &run_manager_component(&run_configurations),
    )?;
    write_run_configurations(&idea, &super_root, &app_root, &run_configurations)?;
    let verification = verify_rustrover_setup(
        &super_root,
        &idea,
        &app_root,
        &toolchain_shim,
        &cargo_projects,
    )?;

    println!("RustRover Setup");
    println!();
    println!("Super workspace: {}", super_root.display());
    println!("App root:        {}", app_root.display());
    println!("Rust/Cargo bin:  {}", rust_bin.display());
    println!("RustRover shim:  {}", toolchain_shim.bin.display());
    println!("Stdlib source:   {}", stdlib.display());
    println!("Cargo projects:  {}", cargo_projects.len());
    println!("Run configs:     {}", run_configurations.len());
    println!("Wrote:");
    println!("  {}", idea.join("cargoProjects.xml").display());
    println!("  {}", idea.join("Loo-Cast-Repos.iml").display());
    println!("  {}", idea.join("rust.xml").display());
    println!("  {}", idea.join("vapor.xml").display());
    println!("  {}", toolchain_shim.bin.display());
    println!("  {}", idea.join("workspace.xml").display());
    println!("  {}", idea.join("runConfigurations").display());
    println!();
    println!("Verified:");
    for check in verification {
        println!("  - {check}");
    }
    println!();
    println!(
        "If RustRover still shows stale editor-only dependency errors, close RustRover, rerun this patcher from a terminal, then reopen and reload Cargo projects."
    );
    Ok(())
}

pub fn source_status_main() -> Result<(), String> {
    let super_root = super_workspace_root()?;
    println!("Vapor Source Status");
    println!();
    println!("Super workspace: {}", super_root.display());
    println!(
        "Source clones:   {}",
        super_root.join(SOURCES_DIR).display()
    );
    println!();
    for source in SOURCE_PRESETS {
        let root = super_root.join(SOURCES_DIR).join(source.name);
        println!("{}", source.name);
        println!("  role: {}", source.role);
        println!("  remote: {}", source.remote);
        println!("  path: {}", root.display());
        if !root.is_dir() {
            println!("  state: missing");
            println!(
                "  clone: {} --super-workspace {} {}",
                tool_command("development/source_setup/clone.rs", None),
                shell_arg(&super_root),
                source.name
            );
            continue;
        }
        match Command::new("git")
            .arg("-C")
            .arg(&root)
            .args(["status", "--short", "--branch"])
            .output()
        {
            Ok(output) if output.status.success() => {
                let text = String::from_utf8_lossy(&output.stdout);
                for line in text.lines() {
                    println!("  {line}");
                }
            }
            Ok(output) => {
                println!("  git status failed: {}", output.status);
            }
            Err(error) => {
                println!("  git unavailable: {error}");
            }
        }
        println!();
    }
    Ok(())
}

fn source_preset(name: &str) -> Result<SourcePreset, String> {
    SOURCE_PRESETS
        .iter()
        .copied()
        .find(|source| source.name.eq_ignore_ascii_case(name))
        .ok_or_else(|| {
            format!(
                "unknown source '{name}'; known: {}",
                SOURCE_PRESETS
                    .iter()
                    .map(|source| source.name)
                    .collect::<Vec<_>>()
                    .join(", ")
            )
        })
}

fn clone_source(sources_root: &Path, source: SourcePreset) -> Result<(), String> {
    let target = sources_root.join(source.name);
    if target.exists() {
        return Err(format!(
            "source clone already exists: {}\nremove it deliberately before cloning again",
            target.display()
        ));
    }
    println!("Cloning {}", source.name);
    println!("  remote: {}", source.remote);
    println!("  branch: {}", source.branch);
    println!("  target: {}", target.display());
    let status = Command::new("git")
        .args([
            "clone",
            "--recurse-submodules",
            "--branch",
            source.branch,
            source.remote,
        ])
        .arg(&target)
        .status()
        .map_err(|error| format!("failed to run git clone: {error}"))?;
    if status.success() {
        Ok(())
    } else {
        Err(format!(
            "git clone for {} failed with {status}",
            source.name
        ))
    }
}

fn record_source_clone(
    super_root: &Path,
    app_root: &Path,
    source: SourcePreset,
) -> Result<(), String> {
    let manifest = super_root.join(SUPER_MANIFEST);
    let source_path = format!("{SOURCES_DIR}/{}", source.name);
    let mut mounted = mounted_sources(&manifest)?;
    if !mounted.iter().any(|name| name == source.name) {
        mounted.push(source.name.to_owned());
    }
    write(
        &manifest,
        &superworkspace_manifest(
            &superworkspace_name(super_root),
            app_root,
            &mounted
                .iter()
                .map(|name| source_preset(name))
                .collect::<Result<Vec<_>, _>>()?,
        ),
    )?;
    println!("  recorded: {source_path}");
    Ok(())
}

fn mounted_sources(manifest: &Path) -> Result<Vec<String>, String> {
    let source =
        fs::read_to_string(manifest).map_err(io("read super-workspace manifest", manifest))?;
    let mut names = Vec::new();
    let mut in_source = false;
    for line in source.lines() {
        let trimmed = line.trim();
        if trimmed == "[[sources]]" {
            in_source = true;
            continue;
        }
        if trimmed.starts_with('[') {
            in_source = false;
        }
        if in_source {
            if let Some(value) = trimmed.strip_prefix("name = ") {
                if let Some(name) = parse_toml_string(value) {
                    names.push(name);
                }
            }
        }
    }
    Ok(names)
}

fn superworkspace_name(root: &Path) -> String {
    root.file_name()
        .map(|name| name.to_string_lossy().into_owned())
        .filter(|name| !name.trim().is_empty())
        .unwrap_or_else(|| "vapor-superworkspace".to_owned())
}

fn superworkspace_manifest(name: &str, app_root: &Path, mounted: &[SourcePreset]) -> String {
    let mut source = String::new();
    source.push_str("schema = 2\n\n");
    source.push_str("[super-workspace]\n");
    source.push_str(&format!("name = \"{}\"\n", toml_escape(name)));
    source.push_str("sources-directory = \"sources\"\n");
    source.push_str("tools = \"app-root\"\n\n");
    source.push_str("[app-root]\n");
    source.push_str(&format!("name = \"{}\"\n", STEAM_APP_DIR_NAME));
    source.push_str(&format!("steam-app-id = {STEAM_APP_ID}\n"));
    source.push_str(&format!("env-var = \"{}\"\n", APP_ROOT_ENV));
    source.push_str(&format!(
        "path = \"{}\"\n\n",
        toml_escape(&app_root.to_string_lossy())
    ));
    source.push_str("[tooling]\n");
    source.push_str("location = \"app-root/resources/vapor/tools\"\n");
    source.push_str("rust-toolchain-owner = \"app-root\"\n\n");
    for source_clone in mounted {
        source.push_str("[[sources]]\n");
        source.push_str(&format!("name = \"{}\"\n", source_clone.name));
        source.push_str(&format!("role = \"{}\"\n", source_clone.role));
        source.push_str(&format!("path = \"{SOURCES_DIR}/{}\"\n", source_clone.name));
        source.push_str(&format!("remote = \"{}\"\n", source_clone.remote));
        source.push_str(&format!("branch = \"{}\"\n\n", source_clone.branch));
    }
    source.push_str("[source-presets]\n");
    source.push_str("operation = \"source-clone\"\n");
    source
}

fn parse_toml_string(value: &str) -> Option<String> {
    let value = value.trim();
    let value = value.strip_prefix('"')?.strip_suffix('"')?;
    Some(value.replace("\\\"", "\"").replace("\\\\", "\\"))
}

fn toml_escape(value: &str) -> String {
    value.replace('\\', "\\\\").replace('"', "\\\"")
}

fn setup_player(app_root: &Path) -> Result<(), String> {
    require_app_root(app_root)?;
    println!("Setup Player");
    println!();
    println!("App root: {}", app_root.display());
    for relative in PLAYER_DIRS {
        let path = app_root.join(relative);
        ensure_contained(app_root, &path)?;
        fs::create_dir_all(&path).map_err(io("create app-root directory", &path))?;
        println!("ensured {relative}");
    }
    if !steamcmd_status(app_root).ready {
        install_steamcmd(app_root)?;
    } else {
        println!("kept app-local SteamCMD");
    }
    write_app_root_anchor(app_root)?;
    write_receipt(app_root, "player", "ready")?;
    println!();
    println!("Status: player setup ready");
    Ok(())
}

fn setup_development(app_root: &Path) -> Result<(), String> {
    setup_player(app_root)?;
    println!();
    println!("Setup Development");
    if !rust_status(app_root).ready {
        install_rust(app_root)?;
    } else {
        install_rust_targets_and_components(app_root)?;
        println!("kept app-local Rust/Cargo");
    }
    if !cross_status(app_root).ready {
        install_zig(app_root)?;
        install_llvm_mingw(app_root)?;
        write_cross_wrappers(app_root)?;
    } else {
        println!("kept app-local cross-build tools");
    }
    write_receipt(app_root, "dev-env", "ready")?;
    println!();
    println!("Status: development setup ready");
    Ok(())
}

fn teardown_player(app_root: &Path) -> Result<(), String> {
    require_app_root(app_root)?;
    println!("Teardown Player");
    println!();
    println!("App root: {}", app_root.display());
    println!();
    teardown_development(app_root)?;
    println!();
    println!("Player-mode state");
    for relative in PLAYER_TEARDOWN_PATHS {
        let path = app_root.join(relative);
        ensure_contained(app_root, &path)?;
        if path.exists() || fs::symlink_metadata(&path).is_ok() {
            remove_path(&path).map_err(io("remove player path", &path))?;
            println!("removed {relative}");
        } else {
            println!("skipped absent {relative}");
        }
    }
    for relative in PLAYER_EMPTY_PARENT_DIRS {
        let path = app_root.join(relative);
        remove_empty_dir(app_root, &path)?;
    }
    println!();
    println!("Status: player setup removed");
    Ok(())
}

fn teardown_development(app_root: &Path) -> Result<(), String> {
    require_app_root(app_root)?;
    println!("Teardown Development");
    println!();
    for relative in [
        "rustup",
        "rustup-home",
        "cargo-home",
        "tools/zig",
        "tools/llvm-mingw",
        "tools/cross",
        ".vapor/state/installer/dev-env.toml",
    ] {
        let path = app_root.join(relative);
        ensure_contained(app_root, &path)?;
        if path.exists() {
            remove_path(&path).map_err(io("remove developer tool path", &path))?;
            println!("removed {relative}");
        } else {
            println!("skipped absent {relative}");
        }
    }
    println!();
    println!("Status: developer toolchain removed");
    Ok(())
}

fn install_steamcmd(app_root: &Path) -> Result<(), String> {
    println!("installing app-local SteamCMD");
    let target = app_root.join("tools/steamcmd");
    reset_dir(app_root, &target)?;
    if cfg!(target_os = "windows") {
        let archive = app_root.join(".vapor/downloads/steamcmd.zip");
        download(STEAMCMD_WINDOWS, &archive)?;
        extract_zip(&archive, &target)?;
    } else {
        let archive = app_root.join(".vapor/downloads/steamcmd_linux.tar.gz");
        download(STEAMCMD_LINUX, &archive)?;
        run(Command::new("tar")
            .arg("-xzf")
            .arg(&archive)
            .arg("-C")
            .arg(&target))?;
    }
    Ok(())
}

fn install_rust(app_root: &Path) -> Result<(), String> {
    println!("installing app-local Rust/Cargo {RUST_TOOLCHAIN}");
    let rustup_init = app_root
        .join(".vapor/downloads")
        .join(executable("rustup-init"));
    download(rustup_init_url()?, &rustup_init)?;
    make_executable(&rustup_init)?;
    run(Command::new(&rustup_init)
        .args([
            "-y",
            "--no-modify-path",
            "--profile",
            "default",
            "--default-toolchain",
            RUST_TOOLCHAIN,
            "--default-host",
            host_rust_target()?,
        ])
        .env("RUSTUP_HOME", app_root.join("rustup-home"))
        .env("CARGO_HOME", app_root.join("cargo-home")))?;
    let source = app_root.join("cargo-home/bin").join(executable("rustup"));
    let target = app_root.join("rustup/bin").join(executable("rustup"));
    if let Some(parent) = target.parent() {
        fs::create_dir_all(parent).map_err(io("create rustup bin directory", parent))?;
    }
    fs::copy(&source, &target).map_err(io("copy rustup into app root", &target))?;
    make_executable(&target)?;
    install_rust_targets_and_components(app_root)
}

fn install_rust_targets_and_components(app_root: &Path) -> Result<(), String> {
    let rustup = app_root.join("rustup/bin").join(executable("rustup"));
    if !is_executable(&rustup) {
        return Err(format!("missing app-local rustup: {}", rustup.display()));
    }
    for target in RUST_TARGETS {
        println!("installing Rust target {target}");
        run(Command::new(&rustup)
            .args(["target", "add", target])
            .env("RUSTUP_HOME", app_root.join("rustup-home"))
            .env("CARGO_HOME", app_root.join("cargo-home")))?;
    }
    for component in RUST_COMPONENTS {
        println!("installing Rust component {component}");
        run(Command::new(&rustup)
            .args(["component", "add", component])
            .env("RUSTUP_HOME", app_root.join("rustup-home"))
            .env("CARGO_HOME", app_root.join("cargo-home")))?;
    }
    Ok(())
}

fn install_zig(app_root: &Path) -> Result<(), String> {
    println!("installing Zig {ZIG_VERSION}");
    let target = app_root.join("tools/zig");
    reset_dir(app_root, &target)?;
    if cfg!(target_os = "windows") {
        let archive = app_root
            .join(".vapor/downloads")
            .join(format!("zig-x86_64-windows-{ZIG_VERSION}.zip"));
        download(ZIG_X86_64_WINDOWS, &archive)?;
        extract_zip_single_root(&archive, &target)?;
    } else {
        let archive = app_root
            .join(".vapor/downloads")
            .join(format!("zig-x86_64-linux-{ZIG_VERSION}.tar.xz"));
        download(ZIG_X86_64_LINUX, &archive)?;
        extract_tar_xz_strip(&archive, &target)?;
    }
    Ok(())
}

fn install_llvm_mingw(app_root: &Path) -> Result<(), String> {
    println!("installing llvm-mingw {LLVM_MINGW_VERSION}");
    let target = app_root.join("tools/llvm-mingw");
    reset_dir(app_root, &target)?;
    if cfg!(target_os = "windows") {
        let archive = app_root
            .join(".vapor/downloads")
            .join(format!("llvm-mingw-{LLVM_MINGW_VERSION}-msvcrt-x86_64.zip"));
        download(LLVM_MINGW_X86_64_WINDOWS, &archive)?;
        verify_sha256(&archive, LLVM_MINGW_X86_64_WINDOWS_SHA256)?;
        extract_zip_single_root(&archive, &target)?;
    } else {
        let archive = app_root.join(".vapor/downloads").join(format!(
            "llvm-mingw-{LLVM_MINGW_VERSION}-msvcrt-ubuntu-22.04-x86_64.tar.xz"
        ));
        download(LLVM_MINGW_X86_64_LINUX, &archive)?;
        verify_sha256(&archive, LLVM_MINGW_X86_64_LINUX_SHA256)?;
        extract_tar_xz_strip(&archive, &target)?;
    }
    Ok(())
}

fn write_cross_wrappers(app_root: &Path) -> Result<(), String> {
    let bin = app_root.join("tools/cross/bin");
    fs::create_dir_all(&bin).map_err(io("create cross-wrapper bin", &bin))?;
    if cfg!(windows) {
        let path = bin.join("x86_64-unknown-linux-gnu-zig-cc.cmd");
        write(
            &path,
            "@echo off\r\n\"%~dp0\\..\\..\\zig\\zig.exe\" cc -target x86_64-linux-gnu %*\r\n",
        )?;
    } else {
        let path = bin.join("x86_64-unknown-linux-gnu-zig-cc");
        write(
            &path,
            "#!/usr/bin/env sh\nexec \"$(dirname \"$0\")/../../zig/zig\" cc -target x86_64-linux-gnu \"$@\"\n",
        )?;
        make_executable(&path)?;
    }
    println!("wrote app-local cross-linker wrappers");
    Ok(())
}

fn directories_status(app_root: &Path) -> ComponentStatus {
    let missing = PLAYER_DIRS
        .iter()
        .filter(|relative| !app_root.join(relative).is_dir())
        .map(|relative| (*relative).to_owned())
        .collect::<Vec<_>>();
    ComponentStatus {
        label: "Generated directories",
        ready: missing.is_empty(),
        path: app_root.join(".vapor"),
        missing,
    }
}

fn steamcmd_status(app_root: &Path) -> ComponentStatus {
    let path = steamcmd_path(app_root);
    let ready = is_executable(&path);
    ComponentStatus {
        label: "SteamCMD",
        ready,
        path,
        missing: if ready {
            Vec::new()
        } else {
            vec!["steamcmd".to_owned()]
        },
    }
}

fn rust_status(app_root: &Path) -> ComponentStatus {
    let rustup = app_root.join("rustup/bin").join(executable("rustup"));
    let required = ["cargo", "rustc", "rustfmt", "cargo-clippy", "rustdoc"];
    let (path, mut missing) = match expected_rust_bin(app_root) {
        Ok(bin) => {
            let missing = required
                .iter()
                .filter(|name| !is_executable(&bin.join(executable(name))))
                .map(|name| format!("{} at {}", name, bin.join(executable(name)).display()))
                .collect::<Vec<_>>();
            (bin, missing)
        }
        Err(error) => (app_root.join("rustup-home/toolchains"), vec![error]),
    };
    if !is_executable(&rustup) {
        missing.push(format!("rustup at {}", rustup.display()));
    }
    ComponentStatus {
        label: "Rust/Cargo",
        ready: missing.is_empty(),
        path,
        missing,
    }
}

fn cross_status(app_root: &Path) -> ComponentStatus {
    let mut missing = Vec::new();
    let zig = app_root.join("tools/zig").join(executable("zig"));
    if !is_executable(&zig) {
        missing.push(format!("zig at {}", zig.display()));
    }
    let llvm_bin = app_root.join("tools/llvm-mingw/bin");
    for name in [
        executable("x86_64-w64-mingw32-clang"),
        "x86_64-w64-mingw32-dlltool".to_owned(),
        "llvm-dlltool".to_owned(),
    ] {
        let path = llvm_bin.join(&name);
        if !is_executable(&path) {
            missing.push(format!("{name} at {}", path.display()));
        }
    }
    let wrapper = app_root.join("tools/cross/bin").join(if cfg!(windows) {
        "x86_64-unknown-linux-gnu-zig-cc.cmd"
    } else {
        "x86_64-unknown-linux-gnu-zig-cc"
    });
    if !is_executable(&wrapper) {
        missing.push(format!(
            "Linux cross-linker wrapper at {}",
            wrapper.display()
        ));
    }
    ComponentStatus {
        label: "Zig/Cross",
        ready: missing.is_empty(),
        path: zig,
        missing,
    }
}

fn print_component(status: &ComponentStatus) {
    println!(
        "{}: {}",
        status.label,
        if status.ready { "ready" } else { "missing" }
    );
    println!("  path: {}", status.path.display());
    for missing in &status.missing {
        println!("  missing: {missing}");
    }
}

fn super_workspace_root() -> Result<PathBuf, String> {
    if let Some(path) = arg_path("--super-workspace") {
        return canonical_dir(path);
    }
    if let Some(path) = env::var_os("VAPOR_SUPER_WORKSPACE_ROOT") {
        return canonical_dir(PathBuf::from(path));
    }
    let current =
        env::current_dir().map_err(|error| format!("failed to read current dir: {error}"))?;
    find_upward(&current, SUPER_MANIFEST).ok_or_else(|| {
        format!(
            "could not find {SUPER_MANIFEST}; run from inside the Loo-Cast/Vapor super-workspace or set VAPOR_SUPER_WORKSPACE_ROOT"
        )
    })
}

fn find_upward(start: &Path, marker: &str) -> Option<PathBuf> {
    let mut current = start;
    for _ in 0..12 {
        if current.join(marker).is_file() {
            return Some(current.to_path_buf());
        }
        current = current.parent()?;
    }
    None
}

fn resolve_app_root(explicit: Option<PathBuf>) -> Result<PathBuf, String> {
    let mut candidates = Vec::<(String, PathBuf)>::new();
    if let Some(path) = explicit {
        candidates.push(("--app-root".to_owned(), path));
    }
    if let Ok(value) = env::var(APP_ROOT_ENV) {
        if !value.trim().is_empty() {
            candidates.push((APP_ROOT_ENV.to_owned(), PathBuf::from(value.trim())));
        }
    }
    let anchor = app_root_anchor_file();
    if let Ok(value) = fs::read_to_string(&anchor) {
        if !value.trim().is_empty() {
            candidates.push((anchor.display().to_string(), PathBuf::from(value.trim())));
        }
    }
    for path in candidate_app_roots_from_tool_source() {
        candidates.push(("deployed setup script".to_owned(), path));
    }
    if let Some(path) = candidate_app_root_from_exe() {
        candidates.push(("running executable".to_owned(), path));
    }
    for path in steam_app_manifest_root_candidates() {
        candidates.push(("Steam app manifest".to_owned(), path));
    }
    for path in common_steam_app_root_candidates() {
        candidates.push(("common Steam library".to_owned(), path));
    }
    if let Ok(current) = env::current_dir() {
        candidates.push(("current directory".to_owned(), current.clone()));
        if let Some(path) = find_upward(&current, APP_MANIFEST) {
            candidates.push((
                "nearest app manifest above current directory".to_owned(),
                path,
            ));
        }
    }

    let mut rejected = Vec::new();
    for (source, candidate) in dedupe_candidates(candidates) {
        match canonical_dir(candidate.clone()) {
            Ok(path) => match require_app_root(&path) {
                Ok(()) => return Ok(path),
                Err(error) => rejected.push(format!("{source}: {} ({error})", candidate.display())),
            },
            Err(error) => rejected.push(format!("{source}: {} ({error})", candidate.display())),
        }
    }
    let checked = if rejected.is_empty() {
        "(no candidates)".to_owned()
    } else {
        rejected.join("\n  - ")
    };

    Err(format!(
        "could not resolve Vapor app root\nchecked:\n  - {}\npass --app-root /path/to/app or run post_install.rs once",
        checked
    ))
}

fn candidate_app_roots_from_tool_source() -> Vec<PathBuf> {
    let mut candidates = Vec::new();
    for path in tool_source_path_candidates() {
        candidates.extend(candidate_app_roots_from_path(&path));
    }
    dedupe_paths(candidates)
}

fn tool_source_path_candidates() -> Vec<PathBuf> {
    let mut candidates = Vec::new();
    candidates.push(PathBuf::from(file!()));
    for arg in env::args_os().take(3) {
        candidates.push(PathBuf::from(arg));
    }
    let mut expanded = Vec::new();
    for candidate in candidates {
        expanded.push(candidate.clone());
        if candidate.is_relative() {
            if let Ok(current) = env::current_dir() {
                expanded.push(current.join(candidate));
            }
        }
    }
    dedupe_paths(expanded)
}

fn candidate_app_roots_from_path(path: &Path) -> Vec<PathBuf> {
    let mut candidates = Vec::new();
    let Some(path) = existing_absolute_path(path) else {
        return candidates;
    };
    for ancestor in path.ancestors() {
        if is_resources_tools_dir(ancestor) {
            if let Some(app_root) = ancestor
                .parent()
                .and_then(Path::parent)
                .and_then(Path::parent)
            {
                candidates.push(app_root.to_path_buf());
            }
        }
        if ancestor.file_name().is_some_and(|name| name == "tools") {
            if let Some(parent) = ancestor.parent() {
                if parent.join(APP_MANIFEST).is_file() {
                    candidates.push(parent.to_path_buf());
                }
            }
        }
    }
    candidates
}

fn existing_absolute_path(path: &Path) -> Option<PathBuf> {
    if let Ok(path) = fs::canonicalize(path) {
        return Some(path);
    }
    if path.is_relative() {
        if let Ok(current) = env::current_dir() {
            if let Ok(path) = fs::canonicalize(current.join(path)) {
                return Some(path);
            }
        }
    }
    None
}

fn is_resources_tools_dir(path: &Path) -> bool {
    path.file_name().is_some_and(|name| name == "tools")
        && path
            .parent()
            .and_then(Path::file_name)
            .is_some_and(|name| name == "vapor")
        && path
            .parent()
            .and_then(Path::parent)
            .and_then(Path::file_name)
            .is_some_and(|name| name == "resources")
}

fn candidate_app_root_from_exe() -> Option<PathBuf> {
    let executable = env::current_exe().ok()?;
    let directory = executable.parent()?;
    if directory.file_name().is_some_and(|name| name == "bin") {
        return directory.parent().map(Path::to_path_buf);
    }
    if directory
        .parent()
        .and_then(Path::file_name)
        .is_some_and(|name| name == "bin")
    {
        return directory
            .parent()
            .and_then(Path::parent)
            .map(Path::to_path_buf);
    }
    None
}

fn steam_app_manifest_root_candidates() -> Vec<PathBuf> {
    let mut roots = Vec::new();
    for steam_root in steam_root_candidates() {
        let steamapps = steam_root.join("steamapps");
        let mut library_roots = vec![steam_root];
        let library_folders = steamapps.join("libraryfolders.vdf");
        if let Ok(source) = fs::read_to_string(&library_folders) {
            library_roots.extend(vdf_values(&source, "path").into_iter().map(PathBuf::from));
        }
        for library_root in dedupe_paths(library_roots) {
            let steamapps = library_root.join("steamapps");
            let manifest = steamapps.join(format!("appmanifest_{STEAM_APP_ID}.acf"));
            if let Ok(source) = fs::read_to_string(&manifest) {
                if let Some(install_dir) = first_vdf_value(&source, "installdir") {
                    roots.push(steamapps.join("common").join(install_dir));
                }
            }
        }
    }
    dedupe_paths(roots)
}

fn steam_root_candidates() -> Vec<PathBuf> {
    let mut roots = Vec::new();
    if cfg!(windows) {
        for root in [
            env::var_os("PROGRAMFILES(X86)").map(PathBuf::from),
            env::var_os("PROGRAMFILES").map(PathBuf::from),
        ]
        .into_iter()
        .flatten()
        {
            roots.push(root.join("Steam"));
        }
    } else {
        if let Some(path) = env::var_os("XDG_DATA_HOME") {
            roots.push(PathBuf::from(path).join("Steam"));
        }
        if let Some(home) = env::var_os("HOME").map(PathBuf::from) {
            roots.push(home.join(".local/share/Steam"));
            roots.push(home.join(".steam/steam"));
            roots.push(home.join(".steam/root"));
        }
    }
    dedupe_paths(roots)
}

fn first_vdf_value(source: &str, key: &str) -> Option<String> {
    vdf_values(source, key).into_iter().next()
}

fn vdf_values(source: &str, key: &str) -> Vec<String> {
    source
        .lines()
        .filter_map(|line| {
            let tokens = quoted_tokens(line);
            if tokens.len() >= 2 && tokens[0] == key {
                Some(tokens[1].clone())
            } else {
                None
            }
        })
        .collect()
}

fn quoted_tokens(line: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut current = String::new();
    let mut in_quote = false;
    let mut escaped = false;
    for ch in line.chars() {
        if !in_quote {
            if ch == '"' {
                in_quote = true;
                current.clear();
            }
            continue;
        }
        if escaped {
            current.push(ch);
            escaped = false;
            continue;
        }
        match ch {
            '\\' => escaped = true,
            '"' => {
                tokens.push(current.clone());
                current.clear();
                in_quote = false;
            }
            _ => current.push(ch),
        }
    }
    tokens
}

fn common_steam_app_root_candidates() -> Vec<PathBuf> {
    let mut roots = Vec::new();
    if let Some(path) = env::var_os("STEAM_COMPAT_INSTALL_PATH") {
        roots.push(PathBuf::from(path));
    }
    if cfg!(windows) {
        for root in [
            env::var_os("PROGRAMFILES(X86)").map(PathBuf::from),
            env::var_os("PROGRAMFILES").map(PathBuf::from),
        ]
        .into_iter()
        .flatten()
        {
            roots.push(
                root.join("Steam")
                    .join("steamapps")
                    .join("common")
                    .join(STEAM_APP_DIR_NAME),
            );
        }
    } else {
        if let Some(path) = env::var_os("XDG_DATA_HOME") {
            roots.push(
                PathBuf::from(path)
                    .join("Steam")
                    .join("steamapps")
                    .join("common")
                    .join(STEAM_APP_DIR_NAME),
            );
        }
        if let Some(home) = env::var_os("HOME").map(PathBuf::from) {
            roots.push(
                home.join(".local/share/Steam/steamapps/common")
                    .join(STEAM_APP_DIR_NAME),
            );
            roots.push(
                home.join(".steam/steam/steamapps/common")
                    .join(STEAM_APP_DIR_NAME),
            );
        }
    }
    dedupe_paths(roots)
}

fn require_app_root(path: &Path) -> Result<(), String> {
    if path.join(APP_SOURCE_MANIFEST).is_file() {
        return Err(format!(
            "found {APP_SOURCE_MANIFEST}; this looks like the app source root, not the installed app root"
        ));
    }
    let marker = path.join(APP_MANIFEST);
    if !marker.is_file() {
        return Err(format!("missing {APP_MANIFEST}"));
    }
    let source = fs::read_to_string(&marker).map_err(io("read app manifest", &marker))?;
    if source.lines().any(|line| line.trim() == "[root]") {
        Ok(())
    } else {
        Err(format!("{} does not declare [root]", marker.display()))
    }
}

fn dedupe_candidates(candidates: Vec<(String, PathBuf)>) -> Vec<(String, PathBuf)> {
    let mut deduped = Vec::new();
    for (source, path) in candidates {
        let key = fs::canonicalize(&path).unwrap_or_else(|_| path.clone());
        if !deduped.iter().any(|(_, existing): &(String, PathBuf)| {
            fs::canonicalize(existing).unwrap_or_else(|_| existing.clone()) == key
        }) {
            deduped.push((source, path));
        }
    }
    deduped
}

fn dedupe_paths(paths: Vec<PathBuf>) -> Vec<PathBuf> {
    let mut deduped = Vec::new();
    for path in paths {
        let key = fs::canonicalize(&path).unwrap_or_else(|_| path.clone());
        if !deduped.iter().any(|existing: &PathBuf| {
            fs::canonicalize(existing).unwrap_or_else(|_| existing.clone()) == key
        }) {
            deduped.push(path);
        }
    }
    deduped
}

fn write_app_root_anchor(app_root: &Path) -> Result<(), String> {
    let anchor = app_root_anchor_file();
    if let Some(parent) = anchor.parent() {
        fs::create_dir_all(parent).map_err(io("create app-root anchor directory", parent))?;
    }
    fs::write(&anchor, app_root.to_string_lossy().as_bytes())
        .map_err(io("write app-root anchor", &anchor))?;
    if cfg!(windows) {
        let _ = Command::new("setx")
            .arg(APP_ROOT_ENV)
            .arg(app_root.to_string_lossy().as_ref())
            .status();
    }
    Ok(())
}

fn remove_app_root_anchor() -> Result<bool, String> {
    let anchor = app_root_anchor_file();
    if anchor.exists() {
        fs::remove_file(&anchor).map_err(io("remove app-root anchor", &anchor))?;
        Ok(true)
    } else {
        Ok(false)
    }
}

fn app_root_anchor_file() -> PathBuf {
    if cfg!(windows) {
        env::var_os("APPDATA")
            .map(PathBuf::from)
            .or_else(|| {
                env::var_os("USERPROFILE").map(|home| PathBuf::from(home).join("AppData/Roaming"))
            })
            .unwrap_or_else(|| PathBuf::from("."))
            .join("loo_cast/app_root.path")
    } else {
        env::var_os("XDG_CONFIG_HOME")
            .map(PathBuf::from)
            .or_else(|| env::var_os("HOME").map(|home| PathBuf::from(home).join(".config")))
            .unwrap_or_else(|| PathBuf::from(".config"))
            .join("loo_cast/app_root.path")
    }
}

fn arg_path(flag: &str) -> Option<PathBuf> {
    let mut args = env::args().skip(1);
    while let Some(arg) = args.next() {
        if arg == flag {
            return args.next().map(PathBuf::from);
        }
    }
    None
}

fn arg_value(flag: &str) -> Option<String> {
    let mut args = env::args().skip(1);
    while let Some(arg) = args.next() {
        if arg == flag {
            return args.next();
        }
    }
    None
}

fn has_flag(flag: &str) -> bool {
    env::args().skip(1).any(|arg| arg == flag)
}

fn positional_path() -> Option<PathBuf> {
    positional_values().first().map(PathBuf::from)
}

fn positional_values() -> Vec<String> {
    let mut values = Vec::new();
    let mut args = env::args().skip(1);
    while let Some(arg) = args.next() {
        if arg.starts_with("--") {
            if matches!(
                arg.as_str(),
                "--path"
                    | "--app-root"
                    | "--super-workspace"
                    | "--workflow"
                    | "--workdir"
                    | "--config-name"
                    | "--open-konsole"
            ) {
                let _ = args.next();
            }
            continue;
        }
        values.push(arg);
    }
    values
}

fn prompt(label: &str) -> Result<String, String> {
    print!("{label}");
    stdout()
        .flush()
        .map_err(|error| format!("failed to flush prompt: {error}"))?;
    let mut value = String::new();
    stdin()
        .read_line(&mut value)
        .map_err(|error| format!("failed to read prompt input: {error}"))?;
    Ok(value.trim_end_matches(['\r', '\n']).to_owned())
}

fn is_yes(value: &str) -> bool {
    matches!(
        value.trim(),
        "y" | "Y" | "yes" | "YES" | "Yes" | "ye" | "YE" | "Ye"
    )
}

fn absolute_from_current(path: PathBuf) -> Result<PathBuf, String> {
    if path.is_absolute() {
        return Ok(path);
    }
    let current =
        env::current_dir().map_err(|error| format!("failed to read current dir: {error}"))?;
    Ok(current.join(path))
}

fn tool_command(relative: &str, app_root: Option<&Path>) -> String {
    let path = find_tool_script(relative, app_root)
        .unwrap_or_else(|| app_root_tool_path(app_root, relative));
    format!("rust-script --force {}", shell_arg(&path))
}

fn find_tool_script(relative: &str, app_root: Option<&Path>) -> Option<PathBuf> {
    let relative = Path::new(relative);
    tool_root_candidates(app_root)
        .into_iter()
        .map(|root| root.join(relative))
        .find(|path| path.is_file())
}

fn tool_root_candidates(app_root: Option<&Path>) -> Vec<PathBuf> {
    let mut candidates = Vec::new();
    if let Some(app_root) = app_root {
        candidates.push(app_root.join("resources/vapor/tools"));
    } else if let Ok(app_root) = resolve_app_root(None) {
        candidates.push(app_root.join("resources/vapor/tools"));
    }
    dedupe_paths(candidates)
}

fn app_root_tool_path(app_root: Option<&Path>, relative: &str) -> PathBuf {
    app_root
        .map(Path::to_path_buf)
        .or_else(|| resolve_app_root(None).ok())
        .unwrap_or_else(|| PathBuf::from("<app-root>"))
        .join("resources/vapor/tools")
        .join(relative)
}

fn expected_rust_bin(app_root: &Path) -> Result<PathBuf, String> {
    Ok(app_root
        .join("rustup-home/toolchains")
        .join(format!("{RUST_TOOLCHAIN}-{}", host_rust_target()?))
        .join("bin"))
}

fn steamcmd_path(app_root: &Path) -> PathBuf {
    let directory = app_root.join("tools/steamcmd");
    let candidates = if cfg!(windows) {
        vec![directory.join("steamcmd.exe")]
    } else {
        vec![directory.join("steamcmd"), directory.join("steamcmd.sh")]
    };
    candidates
        .into_iter()
        .find(|path| is_executable(path))
        .unwrap_or_else(|| directory.join(executable("steamcmd")))
}

fn rustup_init_url() -> Result<&'static str, String> {
    match (env::consts::ARCH, env::consts::OS) {
        ("x86_64", "linux") => Ok(RUSTUP_INIT_X86_64_LINUX),
        ("aarch64", "linux") => Ok(RUSTUP_INIT_AARCH64_LINUX),
        ("x86_64", "windows") => Ok(RUSTUP_INIT_X86_64_WINDOWS),
        _ => Err(format!(
            "rustup bootstrap is not configured for {}-{}",
            env::consts::ARCH,
            env::consts::OS
        )),
    }
}

fn host_rust_target() -> Result<&'static str, String> {
    match (env::consts::ARCH, env::consts::OS) {
        ("x86_64", "linux") => Ok("x86_64-unknown-linux-gnu"),
        ("aarch64", "linux") => Ok("aarch64-unknown-linux-gnu"),
        ("x86_64", "windows") => Ok("x86_64-pc-windows-gnu"),
        _ => Err(format!(
            "Rust host target is not configured for {}-{}",
            env::consts::ARCH,
            env::consts::OS
        )),
    }
}

fn download(url: &str, destination: &Path) -> Result<(), String> {
    if let Some(parent) = destination.parent() {
        fs::create_dir_all(parent).map_err(io("create download directory", parent))?;
    }
    println!("download {url}");
    if cfg!(windows) {
        run(Command::new("powershell")
            .args(["-NoProfile", "-Command"])
            .arg(format!(
                "Invoke-WebRequest -Uri '{}' -OutFile '{}'",
                powershell_quote(url),
                powershell_quote(&destination.to_string_lossy())
            )))
    } else {
        run(Command::new("curl")
            .args(["--fail", "--location", "--show-error", "--output"])
            .arg(destination)
            .arg(url))
    }
}

fn verify_sha256(path: &Path, expected: &str) -> Result<(), String> {
    if cfg!(windows) {
        let output = Command::new("powershell")
            .args(["-NoProfile", "-Command"])
            .arg(format!(
                "(Get-FileHash -Algorithm SHA256 '{}').Hash",
                powershell_quote(&path.to_string_lossy())
            ))
            .output()
            .map_err(|error| format!("failed to start PowerShell sha256 check: {error}"))?;
        let actual = String::from_utf8_lossy(&output.stdout)
            .trim()
            .to_ascii_lowercase();
        if output.status.success() && actual == expected {
            return Ok(());
        }
    } else {
        let output = Command::new("sha256sum")
            .arg(path)
            .output()
            .map_err(|error| format!("failed to start sha256sum: {error}"))?;
        let actual = String::from_utf8_lossy(&output.stdout)
            .split_whitespace()
            .next()
            .unwrap_or_default()
            .to_ascii_lowercase();
        if output.status.success() && actual == expected {
            return Ok(());
        }
    }
    Err(format!(
        "checksum mismatch for {}; expected {expected}",
        path.display()
    ))
}

fn extract_tar_xz_strip(archive: &Path, target: &Path) -> Result<(), String> {
    fs::create_dir_all(target).map_err(io("create extract target", target))?;
    run(Command::new("tar")
        .arg("-xJf")
        .arg(archive)
        .arg("-C")
        .arg(target)
        .arg("--strip-components=1"))
}

fn extract_zip(archive: &Path, target: &Path) -> Result<(), String> {
    fs::create_dir_all(target).map_err(io("create extract target", target))?;
    run(Command::new("powershell")
        .args(["-NoProfile", "-Command"])
        .arg(format!(
            "Expand-Archive -LiteralPath '{}' -DestinationPath '{}' -Force",
            powershell_quote(&archive.to_string_lossy()),
            powershell_quote(&target.to_string_lossy())
        )))
}

fn extract_zip_single_root(archive: &Path, target: &Path) -> Result<(), String> {
    let staging = target.with_extension("extracting");
    reset_plain_dir(&staging)?;
    extract_zip(archive, &staging)?;
    let mut entries = fs::read_dir(&staging)
        .map_err(io("read zip staging", &staging))?
        .filter_map(Result::ok)
        .collect::<Vec<_>>();
    entries.sort_by_key(|entry| entry.path());
    if entries.len() == 1 && entries[0].file_type().is_ok_and(|kind| kind.is_dir()) {
        let single_root = entries[0].path();
        reset_plain_dir(target)?;
        for entry in fs::read_dir(&single_root).map_err(io("read zip single root", &single_root))? {
            let entry = entry.map_err(|error| format!("failed to read zip entry: {error}"))?;
            fs::rename(entry.path(), target.join(entry.file_name()))
                .map_err(|error| format!("failed to move extracted file: {error}"))?;
        }
        remove_path(&staging).map_err(io("remove zip staging", &staging))?;
        Ok(())
    } else {
        if target.exists() {
            remove_path(target).map_err(io("remove old extract target", target))?;
        }
        fs::rename(&staging, target).map_err(io("promote zip staging", target))
    }
}

fn run(command: &mut Command) -> Result<(), String> {
    let rendered = format!("{command:?}");
    let status = command
        .stdin(Stdio::null())
        .status()
        .map_err(|error| format!("failed to start {rendered}: {error}"))?;
    if status.success() {
        Ok(())
    } else {
        Err(format!("{rendered} exited with {status}"))
    }
}

fn run_status(command: &mut Command) -> Result<(), String> {
    let rendered = format!("{command:?}");
    let status = command
        .status()
        .map_err(|error| format!("failed to start {rendered}: {error}"))?;
    if status.success() {
        Ok(())
    } else {
        Err(format!("{rendered} exited with {status}"))
    }
}

fn run_tool(relative: &str, app_root: &Path, args: &[&str]) -> Result<(), String> {
    let script = find_tool_script(relative, Some(app_root))
        .ok_or_else(|| format!("missing app-root tool: {relative}"))?;
    run_status(
        Command::new("rust-script")
            .arg("--force")
            .arg(script)
            .args(args),
    )
}

fn require_executable(path: &Path, label: &str) -> Result<(), String> {
    if is_executable(path) {
        Ok(())
    } else {
        Err(format!(
            "missing {label}: {}\nrun the app build/publish-local workflow to refresh installed binaries",
            path.display()
        ))
    }
}

fn command_exists(name: &str) -> bool {
    env::var_os("PATH").is_some_and(|paths| {
        env::split_paths(&paths).any(|path| is_executable(&path.join(executable(name))))
    })
}

fn prepend_path(paths: &[PathBuf]) {
    let mut all = paths
        .iter()
        .filter(|path| path.exists())
        .cloned()
        .collect::<Vec<_>>();
    if let Some(existing) = env::var_os("PATH") {
        all.extend(env::split_paths(&existing));
    }
    if let Ok(joined) = env::join_paths(all) {
        set_env_var("PATH", joined);
    }
}

fn set_env_var<K, V>(key: K, value: V)
where
    K: AsRef<std::ffi::OsStr>,
    V: AsRef<std::ffi::OsStr>,
{
    // These tools are single-threaded command-line entrypoints. Environment
    // mutation happens before child commands are spawned so app-root tools and
    // toolchains are resolved consistently.
    unsafe {
        env::set_var(key, value);
    }
}

fn command_stdout(command: &mut Command) -> Result<String, String> {
    let rendered = format!("{command:?}");
    let output = command
        .stdin(Stdio::null())
        .output()
        .map_err(|error| format!("failed to start {rendered}: {error}"))?;
    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).into_owned())
    } else {
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(format!(
            "{rendered} exited with {}\nstdout:\n{}\nstderr:\n{}",
            output.status, stdout, stderr
        ))
    }
}

fn rust_stdlib_source(rustc: &Path) -> Result<PathBuf, String> {
    let output = Command::new(rustc)
        .args(["--print", "sysroot"])
        .output()
        .map_err(|error| format!("failed to query app-local rustc sysroot: {error}"))?;
    if !output.status.success() {
        return Err(format!(
            "rustc --print sysroot exited with {}",
            output.status
        ));
    }
    let sysroot = PathBuf::from(String::from_utf8_lossy(&output.stdout).trim().to_owned());
    let source = sysroot.join("lib/rustlib/src/rust/library");
    if source.is_dir() {
        Ok(source)
    } else {
        Err(format!(
            "rust-src is missing at {}; run setup_development.rs",
            source.display()
        ))
    }
}

fn discover_cargo_manifests(root: &Path) -> Result<Vec<PathBuf>, String> {
    let mut manifests = Vec::new();
    collect_cargo_manifests(root, root, &mut manifests)?;
    manifests.sort();
    Ok(manifests)
}

fn discover_cargo_project_registrations(
    root: &Path,
) -> Result<Vec<CargoProjectRegistration>, String> {
    let source_root = root.join(SOURCES_DIR);
    if !source_root.is_dir() {
        return Ok(Vec::new());
    }
    let manifests = discover_cargo_manifests(&source_root)?
        .into_iter()
        .map(|manifest| PathBuf::from(SOURCES_DIR).join(manifest))
        .collect::<Vec<_>>();
    let workspace_dirs = manifests
        .iter()
        .filter_map(|manifest| {
            let full_path = root.join(manifest);
            if cargo_manifest_has_section(&full_path, "workspace") {
                manifest.parent().map(Path::to_path_buf)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    let mut projects = Vec::new();
    for manifest in &manifests {
        let manifest_dir = manifest.parent().unwrap_or_else(|| Path::new(""));
        if workspace_dirs.iter().any(|workspace_dir| {
            workspace_dir != manifest_dir && path_starts_with(manifest_dir, workspace_dir)
        }) {
            continue;
        }

        projects.push(CargoProjectRegistration {
            manifest: manifest.clone(),
        });
    }
    projects.sort_by(|left, right| left.manifest.cmp(&right.manifest));
    Ok(projects)
}

fn collect_cargo_manifests(
    root: &Path,
    dir: &Path,
    manifests: &mut Vec<PathBuf>,
) -> Result<(), String> {
    for entry in fs::read_dir(dir).map_err(io("read source directory", dir))? {
        let entry = entry.map_err(|error| format!("failed to read directory entry: {error}"))?;
        let path = entry.path();
        let name = entry.file_name();
        let name = name.to_string_lossy();
        if name == ".git" || name == "target" || name == ".vapor" || name == "Loo-Cast-Legacy" {
            continue;
        }
        if entry.file_type().is_ok_and(|kind| kind.is_dir()) {
            collect_cargo_manifests(root, &path, manifests)?;
        } else if name == "Cargo.toml" {
            manifests.push(relative_path(root, &path)?);
        }
    }
    Ok(())
}

fn cargo_manifest_has_section(path: &Path, section: &str) -> bool {
    let Ok(source) = fs::read_to_string(path) else {
        return false;
    };
    let expected = format!("[{section}]");
    source.lines().any(|line| line.trim() == expected)
}

fn path_starts_with(path: &Path, parent: &Path) -> bool {
    if parent.as_os_str().is_empty() {
        !path.as_os_str().is_empty()
    } else {
        path.starts_with(parent)
    }
}

fn cargo_projects_xml(projects: &[CargoProjectRegistration]) -> String {
    format!(
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<project version=\"4\">\n{}</project>\n",
        cargo_projects_component(projects)
    )
}

fn cargo_projects_component(projects: &[CargoProjectRegistration]) -> String {
    let mut cargo_projects = String::new();
    cargo_projects.push_str("  <component name=\"CargoProjects\">\n");
    for project in projects {
        cargo_projects.push_str("    <cargoProject FILE=\"$PROJECT_DIR$/");
        cargo_projects.push_str(&xml_escape(&project.manifest.to_string_lossy()));
        cargo_projects.push_str("\" />\n");
    }
    cargo_projects.push_str("  </component>\n");
    cargo_projects
}

fn rust_xml(rust_bin: &Path, stdlib: &Path) -> String {
    format!(
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<project version=\"4\">\n  <component name=\"RsProjectSettings\">\n    <option name=\"toolchainHomeDirectory\" value=\"{}\" />\n    <option name=\"explicitPathToStdlib\" value=\"{}\" />\n  </component>\n  <component name=\"RustProjectSettingsService\">\n    <option name=\"explicitSetExplicitly\" value=\"true\" />\n    <option name=\"toolchainHomeDirectory\" value=\"{}\" />\n    <option name=\"stdlibSourceDirectory\" value=\"{}\" />\n  </component>\n</project>\n",
        xml_escape(&rust_bin.to_string_lossy()),
        xml_escape(&stdlib.to_string_lossy()),
        xml_escape(&rust_bin.to_string_lossy()),
        xml_escape(&stdlib.to_string_lossy()),
    )
}

fn rust_workspace_component(rust_bin: &Path, stdlib: &Path) -> String {
    format!(
        "  <component name=\"RustProjectSettings\">\n    <option name=\"explicitPathToStdlib\" value=\"{}\" />\n    <option name=\"toolchainHomeDirectory\" value=\"{}\" />\n  </component>\n",
        xml_escape(&stdlib.to_string_lossy()),
        xml_escape(&rust_bin.to_string_lossy()),
    )
}

#[derive(Debug, Clone)]
struct RustRoverToolchainShim {
    bin: PathBuf,
    cargo: PathBuf,
    rustc: PathBuf,
    rustup: PathBuf,
}

fn install_rustrover_toolchain_shim(
    idea: &Path,
    app_root: &Path,
    rust_bin: &Path,
    rustup: &Path,
) -> Result<RustRoverToolchainShim, String> {
    #[cfg(not(unix))]
    {
        Ok(RustRoverToolchainShim {
            bin: rust_bin.to_path_buf(),
            cargo: rust_bin.join(executable("cargo")),
            rustc: rust_bin.join(executable("rustc")),
            rustup: rustup.to_path_buf(),
        })
    }

    #[cfg(unix)]
    {
        let shim_bin = idea.join("vapor-toolchain/bin");
        reset_plain_dir(&shim_bin)?;

        let wrappers = [
            ("cargo", rust_bin.join(executable("cargo"))),
            ("rustc", rust_bin.join(executable("rustc"))),
            ("rustup", rustup.to_path_buf()),
            ("rustfmt", rust_bin.join(executable("rustfmt"))),
            ("cargo-clippy", rust_bin.join(executable("cargo-clippy"))),
            ("rustdoc", rust_bin.join(executable("rustdoc"))),
        ];

        for (name, target) in wrappers {
            let path = shim_bin.join(name);
            write_toolchain_shim(&path, app_root, rust_bin, rustup, name, &target)?;
            make_executable(&path)?;
        }

        Ok(RustRoverToolchainShim {
            bin: shim_bin.clone(),
            cargo: shim_bin.join("cargo"),
            rustc: shim_bin.join("rustc"),
            rustup: shim_bin.join("rustup"),
        })
    }
}

fn write_toolchain_shim(
    path: &Path,
    app_root: &Path,
    rust_bin: &Path,
    rustup: &Path,
    tool_name: &str,
    target: &Path,
) -> Result<(), String> {
    let source = format!(
        "#!/usr/bin/env bash\nset -euo pipefail\n\nexport {}={}\nexport RUSTUP_HOME={}\nexport CARGO_HOME={}\nexport PATH={}:{}:{}:\"$PATH\"\n\nTOOL_NAME={}\nREAL_TOOL={}\nREAL_RUSTUP={}\n\ncase \"${{1:-}}\" in\n    +*)\n        REQUESTED_TOOLCHAIN=\"${{1#+}}\"\n        shift\n        exec \"$REAL_RUSTUP\" run \"$REQUESTED_TOOLCHAIN\" \"$TOOL_NAME\" \"$@\"\n        ;;\nesac\n\nexec \"$REAL_TOOL\" \"$@\"\n",
        APP_ROOT_ENV,
        shell_string(app_root),
        shell_string(&app_root.join("rustup-home")),
        shell_string(&app_root.join("cargo-home")),
        shell_string(rust_bin),
        shell_string(&app_root.join("rustup/bin")),
        shell_string(&app_root.join("cargo-home/bin")),
        shell_string_text(tool_name),
        shell_string(target),
        shell_string(rustup),
    );
    write(path, &source)
}

fn verify_rustrover_setup(
    super_root: &Path,
    idea: &Path,
    app_root: &Path,
    toolchain_shim: &RustRoverToolchainShim,
    cargo_projects: &[CargoProjectRegistration],
) -> Result<Vec<String>, String> {
    let mut checks = Vec::new();

    for executable_path in [
        &toolchain_shim.cargo,
        &toolchain_shim.rustc,
        &toolchain_shim.rustup,
    ] {
        if !is_executable(executable_path) {
            return Err(format!(
                "RustRover toolchain shim is missing executable: {}",
                executable_path.display()
            ));
        }
    }
    checks.push("toolchain shim executables exist".to_owned());

    let cargo_wrapper = read_required(&toolchain_shim.cargo)?;
    require_contains(
        &cargo_wrapper,
        &format!(
            "export CARGO_HOME={}",
            shell_string(&app_root.join("cargo-home"))
        ),
        "cargo shim must export app-local CARGO_HOME",
    )?;
    require_contains(
        &cargo_wrapper,
        &format!(
            "export RUSTUP_HOME={}",
            shell_string(&app_root.join("rustup-home"))
        ),
        "cargo shim must export app-local RUSTUP_HOME",
    )?;
    checks.push("toolchain shim exports app-local Cargo/Rustup homes".to_owned());

    let rust_xml_path = idea.join("rust.xml");
    let rust_xml_source = read_required(&rust_xml_path)?;
    require_contains(
        &rust_xml_source,
        &toolchain_shim.bin.to_string_lossy(),
        "rust.xml must point RustRover at the generated toolchain shim",
    )?;
    checks.push("RustRover settings point at the shim toolchain".to_owned());

    verify_app_root_centered_run_configurations(idea, app_root)?;
    checks.push("generated operational workflows use the app-root Vapor binary".to_owned());

    let iml_path = idea.join("Loo-Cast-Repos.iml");
    let iml_source = read_required(&iml_path)?;
    if iml_source.contains("<sourceFolder url=") {
        return Err(format!(
            "{} still contains explicit source roots; this can detach files from Cargo dependency context",
            iml_path.display()
        ));
    }
    checks.push("module file has no hand-written Rust source roots".to_owned());

    for project in cargo_projects {
        let manifest = super_root.join(&project.manifest);
        if !manifest.is_file() {
            return Err(format!(
                "registered Cargo manifest does not exist: {}",
                manifest.display()
            ));
        }
    }
    checks.push(format!(
        "{} registered Cargo manifests exist",
        cargo_projects.len()
    ));

    let rustc_version = command_stdout(
        Command::new(&toolchain_shim.rustc)
            .arg("-Vv")
            .current_dir(super_root),
    )?;
    require_contains(
        &rustc_version,
        &format!("release: {RUST_TOOLCHAIN}"),
        "RustRover shim rustc must be the pinned Vapor toolchain",
    )?;
    checks.push(format!("shim rustc reports Rust {RUST_TOOLCHAIN}"));

    let host_toolchain = format!("{RUST_TOOLCHAIN}-{}", host_rust_target()?);
    let cargo_plus_version = command_stdout(
        Command::new(&toolchain_shim.cargo)
            .arg(format!("+{host_toolchain}"))
            .arg("-Vv")
            .current_dir(super_root),
    )?;
    require_contains(
        &cargo_plus_version,
        &format!("release: {RUST_TOOLCHAIN}"),
        "RustRover shim cargo must accept rustup-style +toolchain directives",
    )?;
    checks.push("shim cargo accepts rustup-style +toolchain directives".to_owned());

    let launcher_manifest = "sources/Vapor-Root/Vapor-Launcher/Cargo.toml";
    if super_root.join(launcher_manifest).is_file() {
        let launcher_metadata =
            cargo_metadata_no_deps(super_root, &toolchain_shim.cargo, launcher_manifest)?;
        require_metadata_names(
            "Vapor-Launcher",
            &launcher_metadata,
            &[
                "vapor_launcher_cli",
                "vapor_launcher_core",
                "vapor_core",
                "clap",
                "dialoguer",
                "owo-colors",
            ],
        )?;
        checks.push("shim cargo metadata sees launcher registry and path dependencies".to_owned());
    } else {
        checks.push("launcher source not mounted; skipped launcher metadata check".to_owned());
    }

    let shell_manifest = "sources/Vapor-Root/Vapor-Shell/Cargo.toml";
    if super_root.join(shell_manifest).is_file() {
        let shell_metadata =
            cargo_metadata_no_deps(super_root, &toolchain_shim.cargo, shell_manifest)?;
        require_metadata_names(
            "Vapor-Shell",
            &shell_metadata,
            &["vapor_shell", "clap", "clap-repl"],
        )?;
        checks.push("shim cargo metadata sees shell workspace dependencies".to_owned());
    } else {
        checks.push("shell source not mounted; skipped shell metadata check".to_owned());
    }

    Ok(checks)
}

fn cargo_metadata_no_deps(
    super_root: &Path,
    cargo: &Path,
    manifest: &str,
) -> Result<String, String> {
    command_stdout(
        Command::new(cargo)
            .args(["metadata", "--format-version", "1", "--manifest-path"])
            .arg(super_root.join(manifest))
            .arg("--no-deps")
            .current_dir(super_root),
    )
}

fn verify_app_root_centered_run_configurations(idea: &Path, app_root: &Path) -> Result<(), String> {
    let generated_scripts = idea.join("vapor-run");
    if generated_scripts.exists() {
        return Err(format!(
            "generated IDE workflow script directory still exists: {}",
            generated_scripts.display()
        ));
    }
    let app_root_workflow_stems = [
        "10_Source_Workspaces_Create_Basic_Content_Workspace",
        "20_Development_Workflows_Open_Vapor_Shell",
        "20_Development_Workflows_Check_App_Binaries",
        "20_Development_Workflows_Check_Content",
        "20_Development_Workflows_Build_App",
        "20_Development_Workflows_Deploy_App",
        "20_Development_Workflows_Build_Content",
        "20_Development_Workflows_Deploy_Content",
        "30_Publishing_Publish_App",
        "30_Publishing_Publish_Content",
    ];

    for stem in app_root_workflow_stems {
        let xml_path = idea.join("runConfigurations").join(format!("{stem}.xml"));
        let xml = read_required(&xml_path)?;
        require_contains(
            &xml,
            &xml_escape(
                &app_root
                    .join("resources/vapor/tools/development/ide_run/run.rs")
                    .to_string_lossy(),
            ),
            "generated workflow must launch the app-root IDE runner",
        )?;
        require_contains(
            &xml,
            "rust-script --force",
            "generated workflow must execute the app-root runner through rust-script",
        )?;
        require_contains(
            &xml,
            &format!(
                "<option name=\"SCRIPT_WORKING_DIRECTORY\" value=\"{}\" />",
                xml_escape(&app_root.to_string_lossy())
            ),
            "app-root workflow run configuration must start in the app root",
        )?;
        if xml.contains("$PROJECT_DIR$/tools")
            || xml.contains(".idea/vapor-run")
            || xml.contains("cargo run --package vapor_shell")
            || xml.contains("cargo run --package vapor_launcher_cli")
        {
            return Err(format!(
                "{} still depends on source-local tools or generated workflow scripts",
                xml_path.display()
            ));
        }
    }

    Ok(())
}

fn require_metadata_names(
    label: &str,
    metadata: &str,
    required_names: &[&str],
) -> Result<(), String> {
    let missing = required_names
        .iter()
        .filter(|name| {
            let needle = format!("\"name\":\"{name}\"");
            !metadata.contains(&needle)
        })
        .copied()
        .collect::<Vec<_>>();
    if missing.is_empty() {
        Ok(())
    } else {
        Err(format!(
            "{label} Cargo metadata through RustRover shim is missing dependency/package names: {}",
            missing.join(", ")
        ))
    }
}

fn read_required(path: &Path) -> Result<String, String> {
    fs::read_to_string(path).map_err(io("read generated RustRover file", path))
}

fn require_contains(source: &str, needle: &str, message: &str) -> Result<(), String> {
    if source.contains(needle) {
        Ok(())
    } else {
        Err(format!("{message}: missing {needle:?}"))
    }
}

fn run_manager_component(configurations: &[RunConfiguration]) -> String {
    let selected = configurations
        .first()
        .map(run_configuration_item_value)
        .unwrap_or_else(|| "Shell Script.Patch RustRover".to_owned());
    format!(
        "  <component name=\"RunManager\" selected=\"{}\" />\n",
        xml_escape(&selected)
    )
}

fn run_configuration_item_value(configuration: &RunConfiguration) -> String {
    let prefix = match configuration.kind {
        RunConfigurationKind::Cargo => "Cargo",
        RunConfigurationKind::Shell | RunConfigurationKind::Konsole => "Shell Script",
    };
    format!("{prefix}.{}", configuration.name)
}

fn rustrover_run_configurations() -> Vec<RunConfiguration> {
    use RunConfigurationKind::{Konsole, Shell};
    vec![
        RunConfiguration {
            name: "Check app environment",
            folder: "00 Initial setup / Environment",
            file_stem: "00_Initial_Setup_Check_App_Environment",
            kind: Shell,
            working_directory: ".",
            command: "app-status",
        },
        RunConfiguration {
            name: "Install developer tools",
            folder: "00 Initial setup / Tooling",
            file_stem: "00_Initial_Setup_Install_Developer_Tools",
            kind: Konsole,
            working_directory: ".",
            command: "setup-development",
        },
        RunConfiguration {
            name: "Check source checkout",
            folder: "10 Source workspaces / Status",
            file_stem: "10_Source_Workspaces_Check_Source_Checkout",
            kind: Shell,
            working_directory: ".",
            command: "source-status",
        },
        RunConfiguration {
            name: "Clone source: Vapor Root",
            folder: "10 Source workspaces / Clone",
            file_stem: "10_Source_Workspaces_Clone_Vapor_Root",
            kind: Konsole,
            working_directory: ".",
            command: "source-clone:Vapor-Root",
        },
        RunConfiguration {
            name: "Clone source: Loo-Cast",
            folder: "10 Source workspaces / Clone",
            file_stem: "10_Source_Workspaces_Clone_Loo_Cast",
            kind: Konsole,
            working_directory: ".",
            command: "source-clone:Loo-Cast",
        },
        RunConfiguration {
            name: "Clone source: Vapor Registry",
            folder: "10 Source workspaces / Clone",
            file_stem: "10_Source_Workspaces_Clone_Vapor_Registry",
            kind: Konsole,
            working_directory: ".",
            command: "source-clone:Vapor-Registry",
        },
        RunConfiguration {
            name: "Clone source: Vapor Server Root",
            folder: "10 Source workspaces / Clone",
            file_stem: "10_Source_Workspaces_Clone_Vapor_Server_Root",
            kind: Konsole,
            working_directory: ".",
            command: "source-clone:Vapor-Server-Root",
        },
        RunConfiguration {
            name: "Create basic content workspace",
            folder: "10 Source workspaces / Create",
            file_stem: "10_Source_Workspaces_Create_Basic_Content_Workspace",
            kind: Konsole,
            working_directory: "$APP_ROOT",
            command: "source-init-basic-content",
        },
        RunConfiguration {
            name: "Open Vapor Shell",
            folder: "20 Development workflows / Shell",
            file_stem: "20_Development_Workflows_Open_Vapor_Shell",
            kind: Konsole,
            working_directory: "$APP_ROOT",
            command: "vapor-shell-interactive",
        },
        RunConfiguration {
            name: "Check app binaries",
            folder: "20 Development workflows / App",
            file_stem: "20_Development_Workflows_Check_App_Binaries",
            kind: Shell,
            working_directory: "$APP_ROOT",
            command: "app-binaries-status",
        },
        RunConfiguration {
            name: "Check content",
            folder: "20 Development workflows / Content",
            file_stem: "20_Development_Workflows_Check_Content",
            kind: Shell,
            working_directory: "$APP_ROOT",
            command: "content-status",
        },
        RunConfiguration {
            name: "Build app",
            folder: "20 Development workflows / App",
            file_stem: "20_Development_Workflows_Build_App",
            kind: Konsole,
            working_directory: "$APP_ROOT",
            command: "root-build-host",
        },
        RunConfiguration {
            name: "Deploy app",
            folder: "20 Development workflows / App",
            file_stem: "20_Development_Workflows_Deploy_App",
            kind: Konsole,
            working_directory: "$APP_ROOT",
            command: "root-deploy-host",
        },
        RunConfiguration {
            name: "Build content",
            folder: "20 Development workflows / Content",
            file_stem: "20_Development_Workflows_Build_Content",
            kind: Konsole,
            working_directory: "$APP_ROOT",
            command: "content-build-host",
        },
        RunConfiguration {
            name: "Deploy content",
            folder: "20 Development workflows / Content",
            file_stem: "20_Development_Workflows_Deploy_Content",
            kind: Konsole,
            working_directory: "$APP_ROOT",
            command: "content-deploy-artifact",
        },
        RunConfiguration {
            name: "Publish app",
            folder: "30 Publishing / Steam app",
            file_stem: "30_Publishing_Publish_App",
            kind: Konsole,
            working_directory: "$APP_ROOT",
            command: "root-publish",
        },
        RunConfiguration {
            name: "Publish content",
            folder: "30 Publishing / Workshop content",
            file_stem: "30_Publishing_Publish_Content",
            kind: Konsole,
            working_directory: "$APP_ROOT",
            command: "content-publish",
        },
    ]
}

fn module_xml(root: &Path, projects: &[CargoProjectRegistration]) -> String {
    let mut exclude_folders = Vec::new();

    for project in projects {
        if let Some(project_dir) = project.manifest.parent() {
            exclude_folders.push(project_dir.join("target"));
        }
    }

    exclude_folders.sort();
    exclude_folders.dedup();

    let mut body = String::new();
    body.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
    body.push_str("<module type=\"EMPTY_MODULE\" version=\"4\">\n");
    body.push_str("  <component name=\"NewModuleRootManager\">\n");
    body.push_str("    <content url=\"file://$MODULE_DIR$\">\n");
    for folder in exclude_folders {
        body.push_str("      <excludeFolder url=\"file://$MODULE_DIR$/");
        body.push_str(&xml_escape(&folder.to_string_lossy()));
        body.push_str("\" />\n");
    }
    body.push_str("    </content>\n");
    body.push_str("    <orderEntry type=\"inheritedJdk\" />\n");
    body.push_str("    <orderEntry type=\"sourceFolder\" forTests=\"false\" />\n");
    body.push_str("  </component>\n");
    body.push_str("</module>\n");
    body
}

fn patch_workspace_xml(
    path: &Path,
    cargo_projects: &str,
    rust_settings: &str,
    run_manager: &str,
) -> Result<(), String> {
    let source = match fs::read_to_string(path) {
        Ok(source) => source,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<project version=\"4\">\n</project>\n"
                .to_owned()
        }
        Err(error) => return Err(io("read RustRover workspace file", path)(error)),
    };
    let source = upsert_project_component(&source, "CargoProjects", cargo_projects)?;
    let source = upsert_project_component(&source, "RustProjectSettings", rust_settings)?;
    let source = upsert_project_component(&source, "RunManager", run_manager)?;
    write(path, &source)
}

fn upsert_project_component(source: &str, name: &str, replacement: &str) -> Result<String, String> {
    let marker = format!("<component name=\"{name}\"");
    if let Some(component_start) = source.find(&marker) {
        let start = source[..component_start]
            .rfind('\n')
            .map(|index| index + 1)
            .unwrap_or(component_start);
        let tag_end = source[component_start..]
            .find('>')
            .map(|index| component_start + index + 1)
            .ok_or_else(|| format!("malformed workspace.xml component {name}: missing '>'"))?;
        let mut end = if source[component_start..tag_end].trim_end().ends_with("/>") {
            tag_end
        } else {
            source[tag_end..]
                .find("</component>")
                .map(|index| tag_end + index + "</component>".len())
                .ok_or_else(|| {
                    format!("malformed workspace.xml component {name}: missing </component>")
                })?
        };
        if source[end..].starts_with('\n') {
            end += 1;
        }
        let mut updated = String::new();
        updated.push_str(&source[..start]);
        updated.push_str(replacement);
        updated.push_str(&source[end..]);
        return Ok(updated);
    }

    let project_start = source
        .find("<project version=\"4\">")
        .ok_or_else(|| "workspace.xml does not contain <project version=\"4\">".to_owned())?;
    let insert = source[project_start..]
        .find('\n')
        .map(|index| project_start + index + 1)
        .ok_or_else(|| "workspace.xml project element is not line-oriented".to_owned())?;
    let mut updated = String::new();
    updated.push_str(&source[..insert]);
    updated.push_str(replacement);
    updated.push_str(&source[insert..]);
    Ok(updated)
}

fn write_run_configurations(
    idea: &Path,
    super_root: &Path,
    app_root: &Path,
    configurations: &[RunConfiguration],
) -> Result<(), String> {
    let run_config_dir = idea.join("runConfigurations");
    fs::create_dir_all(&run_config_dir)
        .map_err(io("create run configuration directory", &run_config_dir))?;
    let generated_script_dir = idea.join("vapor-run");
    if generated_script_dir.exists() {
        fs::remove_dir_all(&generated_script_dir).map_err(io(
            "remove generated IDE workflow scripts",
            &generated_script_dir,
        ))?;
    }
    remove_stale_generated_run_configurations(&run_config_dir, configurations)?;

    for configuration in configurations {
        let xml_path = run_config_dir.join(format!("{}.xml", configuration.file_stem));
        match configuration.kind {
            RunConfigurationKind::Cargo => {
                write(
                    &xml_path,
                    &cargo_run_configuration_xml(configuration, app_root),
                )?;
            }
            RunConfigurationKind::Shell | RunConfigurationKind::Konsole => {
                write(
                    &xml_path,
                    &shell_run_configuration_xml(configuration, super_root, app_root),
                )?;
            }
        }
    }

    Ok(())
}

fn remove_stale_generated_run_configurations(
    run_config_dir: &Path,
    configurations: &[RunConfiguration],
) -> Result<(), String> {
    for stem in GENERATED_RUN_CONFIGURATION_STEMS {
        if configurations
            .iter()
            .any(|configuration| configuration.file_stem == *stem)
        {
            continue;
        }
        let xml_path = run_config_dir.join(format!("{stem}.xml"));
        if xml_path.is_file() {
            fs::remove_file(&xml_path).map_err(io("remove stale run configuration", &xml_path))?;
        }
    }
    Ok(())
}

fn cargo_run_configuration_xml(configuration: &RunConfiguration, app_root: &Path) -> String {
    format!(
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<component name=\"ProjectRunConfigurationManager\">\n  <configuration default=\"false\" name=\"{}\" type=\"CargoCommandRunConfiguration\" factoryName=\"Cargo Command\">\n    <folderName value=\"{}\" />\n    <option name=\"command\" value=\"{}\" />\n    <option name=\"workingDirectory\" value=\"file://$PROJECT_DIR$/{}\" />\n{}\n    <option name=\"emulateTerminal\" value=\"true\" />\n    <option name=\"channel\" value=\"DEFAULT\" />\n    <option name=\"requiredFeatures\" value=\"true\" />\n    <option name=\"allFeatures\" value=\"false\" />\n    <option name=\"withSudo\" value=\"false\" />\n    <option name=\"buildTarget\" value=\"REMOTE\" />\n    <option name=\"backtrace\" value=\"SHORT\" />\n    <option name=\"isRedirectInput\" value=\"false\" />\n    <option name=\"redirectInputPath\" value=\"\" />\n    <method v=\"2\">\n      <option name=\"CARGO.BUILD_TASK_PROVIDER\" enabled=\"true\" />\n    </method>\n  </configuration>\n</component>\n",
        xml_escape(configuration.name),
        xml_escape(configuration.folder),
        xml_escape(configuration.command),
        xml_escape(configuration.working_directory),
        run_config_envs_xml(app_root),
    )
}

fn shell_run_configuration_xml(
    configuration: &RunConfiguration,
    super_root: &Path,
    app_root: &Path,
) -> String {
    let working_directory = run_configuration_xml_working_directory(configuration, app_root);
    let script_path = app_root.join("resources/vapor/tools/development/ide_run/run.rs");
    let options = format!(
        "--workflow {} --super-workspace {} --app-root {} --workdir {} --config-name {} --open-konsole {}",
        shell_arg_text(configuration.command),
        shell_arg(super_root),
        shell_arg(app_root),
        shell_arg(Path::new(&working_directory)),
        shell_arg_text(configuration.name),
        if matches!(configuration.kind, RunConfigurationKind::Konsole) {
            "true"
        } else {
            "false"
        },
    );
    format!(
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<component name=\"ProjectRunConfigurationManager\">\n  <configuration default=\"false\" name=\"{}\" type=\"ShConfigurationType\" factoryName=\"Shell Script\">\n    <folderName value=\"{}\" />\n    <option name=\"SCRIPT_TEXT\" value=\"\" />\n    <option name=\"INDEPENDENT_SCRIPT_PATH\" value=\"true\" />\n    <option name=\"SCRIPT_PATH\" value=\"{}\" />\n    <option name=\"SCRIPT_OPTIONS\" value=\"{}\" />\n    <option name=\"INDEPENDENT_SCRIPT_WORKING_DIRECTORY\" value=\"true\" />\n    <option name=\"SCRIPT_WORKING_DIRECTORY\" value=\"{}\" />\n    <option name=\"INDEPENDENT_INTERPRETER_PATH\" value=\"true\" />\n    <option name=\"INTERPRETER_PATH\" value=\"/usr/bin/env\" />\n    <option name=\"INTERPRETER_OPTIONS\" value=\"rust-script --force\" />\n    <option name=\"EXECUTE_IN_TERMINAL\" value=\"true\" />\n    <option name=\"EXECUTE_SCRIPT_FILE\" value=\"true\" />\n    <envs />\n    <method v=\"2\" />\n  </configuration>\n</component>\n",
        xml_escape(configuration.name),
        xml_escape(configuration.folder),
        xml_escape(&script_path.to_string_lossy()),
        xml_escape(&options),
        xml_escape(&working_directory),
    )
}

fn run_configuration_xml_working_directory(
    configuration: &RunConfiguration,
    app_root: &Path,
) -> String {
    if configuration.working_directory == "$APP_ROOT" {
        app_root.to_string_lossy().into_owned()
    } else if configuration.working_directory == "." {
        "$PROJECT_DIR$".to_owned()
    } else {
        format!("$PROJECT_DIR$/{}", configuration.working_directory)
    }
}

fn run_config_envs_xml(app_root: &Path) -> String {
    format!(
        "    <envs>\n      <env name=\"{}\" value=\"{}\" />\n      <env name=\"RUSTUP_HOME\" value=\"{}\" />\n      <env name=\"CARGO_HOME\" value=\"{}\" />\n    </envs>",
        APP_ROOT_ENV,
        xml_escape(&app_root.to_string_lossy()),
        xml_escape(&app_root.join("rustup-home").to_string_lossy()),
        xml_escape(&app_root.join("cargo-home").to_string_lossy()),
    )
}

fn vapor_xml(
    super_root: &Path,
    app_root: &Path,
    cargo: &Path,
    rustc: &Path,
    rustup: &Path,
    stdlib: &Path,
    toolchain_shim: &RustRoverToolchainShim,
) -> String {
    format!(
        "<project version=\"4\">\n  <component name=\"VaporSuperWorkspaceSettings\">\n    <option name=\"schemaVersion\" value=\"1\" />\n    <option name=\"superWorkspaceRoot\" value=\"$PROJECT_DIR$\" />\n    <option name=\"superWorkspaceManifest\" value=\"{}\" />\n    <option name=\"appRoot\" value=\"{}\" />\n    <option name=\"cargoHome\" value=\"{}\" />\n    <option name=\"rustupHome\" value=\"{}\" />\n    <option name=\"cargoPath\" value=\"{}\" />\n    <option name=\"rustcPath\" value=\"{}\" />\n    <option name=\"rustupPath\" value=\"{}\" />\n    <option name=\"rustStdlibSource\" value=\"{}\" />\n    <option name=\"rustRoverToolchainBin\" value=\"{}\" />\n    <option name=\"rustRoverCargoPath\" value=\"{}\" />\n    <option name=\"rustRoverRustcPath\" value=\"{}\" />\n    <option name=\"rustRoverRustupPath\" value=\"{}\" />\n  </component>\n</project>\n",
        xml_escape(&super_root.join(SUPER_MANIFEST).to_string_lossy()),
        xml_escape(&app_root.to_string_lossy()),
        xml_escape(&app_root.join("cargo-home").to_string_lossy()),
        xml_escape(&app_root.join("rustup-home").to_string_lossy()),
        xml_escape(&cargo.to_string_lossy()),
        xml_escape(&rustc.to_string_lossy()),
        xml_escape(&rustup.to_string_lossy()),
        xml_escape(&stdlib.to_string_lossy()),
        xml_escape(&toolchain_shim.bin.to_string_lossy()),
        xml_escape(&toolchain_shim.cargo.to_string_lossy()),
        xml_escape(&toolchain_shim.rustc.to_string_lossy()),
        xml_escape(&toolchain_shim.rustup.to_string_lossy()),
    )
}

fn reset_dir(app_root: &Path, path: &Path) -> Result<(), String> {
    ensure_contained(app_root, path)?;
    reset_plain_dir(path)
}

fn reset_plain_dir(path: &Path) -> Result<(), String> {
    if path.exists() {
        remove_path(path).map_err(io("remove directory", path))?;
    }
    fs::create_dir_all(path).map_err(io("create directory", path))
}

fn remove_path(path: &Path) -> std::io::Result<()> {
    let Ok(metadata) = fs::symlink_metadata(path) else {
        return Ok(());
    };
    if metadata.file_type().is_dir() && !metadata.file_type().is_symlink() {
        fs::remove_dir_all(path)
    } else {
        fs::remove_file(path)
    }
}

fn remove_empty_dir(root: &Path, path: &Path) -> Result<(), String> {
    ensure_contained(root, path)?;
    let Ok(metadata) = fs::symlink_metadata(path) else {
        return Ok(());
    };
    if !metadata.file_type().is_dir() || metadata.file_type().is_symlink() {
        return Ok(());
    }
    match fs::remove_dir(path) {
        Ok(()) => Ok(()),
        Err(error)
            if matches!(
                error.kind(),
                std::io::ErrorKind::NotFound | std::io::ErrorKind::DirectoryNotEmpty
            ) =>
        {
            Ok(())
        }
        Err(error) => Err(format!(
            "failed to remove empty directory '{}': {error}",
            path.display()
        )),
    }
}

fn write_receipt(app_root: &Path, name: &str, state: &str) -> Result<(), String> {
    let path = app_root
        .join(".vapor/state/installer")
        .join(format!("{name}.toml"));
    let body = format!(
        "schema = 1\nstate = \"{state}\"\ntool = \"resources/vapor/tools/production/app_setup/{name}\"\n"
    );
    write(&path, &body)
}

fn write(path: &Path, source: &str) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(io("create parent directory", parent))?;
    }
    let mut file = fs::File::create(path).map_err(io("create file", path))?;
    file.write_all(source.as_bytes())
        .map_err(io("write file", path))
}

fn make_executable(path: &Path) -> Result<(), String> {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut permissions = fs::metadata(path)
            .map_err(io("read executable metadata", path))?
            .permissions();
        permissions.set_mode(permissions.mode() | 0o755);
        fs::set_permissions(path, permissions).map_err(io("set executable bit", path))?;
    }
    Ok(())
}

fn is_executable(path: &Path) -> bool {
    if !path.is_file() {
        return false;
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::metadata(path).is_ok_and(|metadata| metadata.permissions().mode() & 0o111 != 0)
    }
    #[cfg(not(unix))]
    {
        true
    }
}

fn executable(name: &str) -> String {
    format!("{name}{}", env::consts::EXE_SUFFIX)
}

fn canonical_dir(path: PathBuf) -> Result<PathBuf, String> {
    let path = fs::canonicalize(&path)
        .map_err(|error| format!("failed to resolve '{}': {error}", path.display()))?;
    if path.is_dir() {
        Ok(path)
    } else {
        Err(format!("not a directory: {}", path.display()))
    }
}

fn ensure_contained(root: &Path, path: &Path) -> Result<(), String> {
    let root = fs::canonicalize(root).map_err(io("resolve containment root", root))?;
    let candidate = normalized_absolute(path)?;
    if candidate.starts_with(&root) {
        Ok(())
    } else {
        Err(format!(
            "path '{}' escapes root '{}'",
            candidate.display(),
            root.display()
        ))
    }
}

fn normalized_absolute(path: &Path) -> Result<PathBuf, String> {
    use std::path::Component;

    let absolute = if path.is_absolute() {
        path.to_path_buf()
    } else {
        env::current_dir()
            .map_err(|error| format!("failed to read current dir: {error}"))?
            .join(path)
    };
    let mut normalized = PathBuf::new();
    for component in absolute.components() {
        match component {
            Component::Prefix(prefix) => normalized.push(prefix.as_os_str()),
            Component::RootDir => normalized.push(component.as_os_str()),
            Component::CurDir => {}
            Component::ParentDir => {
                normalized.pop();
            }
            Component::Normal(part) => normalized.push(part),
        }
    }
    Ok(normalized)
}

fn relative_path(root: &Path, path: &Path) -> Result<PathBuf, String> {
    path.strip_prefix(root)
        .map(Path::to_path_buf)
        .map_err(|_| format!("path '{}' is outside '{}'", path.display(), root.display()))
}

fn shell_arg(path: &Path) -> String {
    shell_arg_text(&path.to_string_lossy())
}

fn shell_arg_text(text: &str) -> String {
    if text
        .chars()
        .all(|ch| ch.is_ascii_alphanumeric() || "/._-".contains(ch))
    {
        text.to_owned()
    } else {
        format!("'{}'", text.replace('\'', "'\\''"))
    }
}

fn shell_string(path: &Path) -> String {
    shell_string_text(&path.to_string_lossy())
}

fn shell_string_text(value: &str) -> String {
    format!("'{}'", value.replace('\'', "'\\''"))
}

fn powershell_quote(value: &str) -> String {
    value.replace('\'', "''")
}

fn xml_escape(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('"', "&quot;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

fn io<'a>(action: &'a str, path: &'a Path) -> impl Fn(std::io::Error) -> String + 'a {
    move |error| format!("failed to {action} '{}': {error}", path.display())
}
