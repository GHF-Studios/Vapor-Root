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
    path == parent || path.starts_with(parent)
}
