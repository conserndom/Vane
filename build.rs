use std::{env, fs, io, path::Path};

fn main() -> io::Result<()> {
    println!("cargo:rerun-if-changed=web/dist");

    let out_dir = env::var("OUT_DIR").expect("OUT_DIR is set by Cargo");
    let dest = Path::new(&out_dir).join("embedded_assets.rs");
    let dist = Path::new("web/dist");

    let mut entries = Vec::new();
    if dist.is_dir() {
        collect_assets(dist, dist, &mut entries)?;
    }
    entries.sort_by(|a, b| a.0.cmp(&b.0));

    let mut generated = String::from("pub static EMBEDDED_ASSETS: &[(&str, &[u8], &str)] = &[\n");
    for (route, path, content_type) in entries {
        generated.push_str(&format!(
            "    ({route:?}, include_bytes!({path:?}), {content_type:?}),\n",
            route = route,
            path = path,
            content_type = content_type,
        ));
    }
    generated.push_str("];\n");

    fs::write(dest, generated)
}

fn collect_assets(
    root: &Path,
    dir: &Path,
    entries: &mut Vec<(String, String, &'static str)>,
) -> io::Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            collect_assets(root, &path, entries)?;
        } else if path.is_file() {
            println!("cargo:rerun-if-changed={}", path.display());
            let route = path
                .strip_prefix(root)
                .expect("asset path starts with root")
                .to_string_lossy()
                .replace('\\', "/");
            entries.push((
                route,
                fs::canonicalize(&path)?.to_string_lossy().into_owned(),
                content_type(&path),
            ));
        }
    }
    Ok(())
}

fn content_type(path: &Path) -> &'static str {
    match path.extension().and_then(|ext| ext.to_str()).unwrap_or("") {
        "css" => "text/css; charset=utf-8",
        "html" => "text/html; charset=utf-8",
        "js" | "mjs" => "application/javascript; charset=utf-8",
        "json" | "webmanifest" => "application/json; charset=utf-8",
        "png" => "image/png",
        "svg" => "image/svg+xml",
        "ico" => "image/x-icon",
        "jpg" | "jpeg" => "image/jpeg",
        "gif" => "image/gif",
        "wasm" => "application/wasm",
        "woff" => "font/woff",
        "woff2" => "font/woff2",
        "txt" => "text/plain; charset=utf-8",
        _ => "application/octet-stream",
    }
}
