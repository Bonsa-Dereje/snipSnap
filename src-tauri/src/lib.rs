use std::fs;
use std::path::PathBuf;
use std::thread;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

/// Tauri sometimes hands us an asset URL like
///   asset://localhost/C:/Users/foo/bar.srt   (Windows)
///   asset://localhost/home/foo/bar.srt        (Linux/macOS)
/// Strip that prefix so we get a real filesystem path.
fn normalize_path(raw: &str) -> PathBuf {
    let stripped = raw
        .trim()
        .strip_prefix("asset://localhost")
        .unwrap_or(raw);

    // On Windows the path will start with /C:/... — remove the leading slash
    #[cfg(target_os = "windows")]
    let stripped = stripped.strip_prefix('/').unwrap_or(stripped);

    // URL-decode any percent-encoded characters (%20 → space, etc.)
    let decoded = percent_encoding::percent_decode_str(stripped)
        .decode_utf8_lossy()
        .into_owned();

    PathBuf::from(decoded)
}

/// Returns the app data directory base: ~/Documents/donalds/projects/<project_name>
fn project_base(project_name: &str) -> PathBuf {
    dirs::document_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("donalds")
        .join("projects")
        .join(project_name)
}

#[tauri::command]
fn import_project_files(
    srt_path: String,
    video_path: String,
    project_name: String,
) {
    thread::spawn(move || {
        let base = project_base(&project_name);

        // ── Copy SRT ──────────────────────────────────────────
        if !srt_path.is_empty() {
            let srt_src = normalize_path(&srt_path);
            println!("[snipSnap] SRT resolved path: {}", srt_src.display());

            let srt_dir = base.join("srt");
            match fs::create_dir_all(&srt_dir) {
                Ok(_) => {
                    let file_name = srt_src.file_name().unwrap_or_default();
                    let srt_dst = srt_dir.join(file_name);
                    match fs::copy(&srt_src, &srt_dst) {
                        Ok(_)  => println!("[snipSnap] SRT copied → {}", srt_dst.display()),
                        Err(e) => eprintln!("[snipSnap] SRT copy failed: {e}"),
                    }
                }
                Err(e) => eprintln!("[snipSnap] Could not create srt dir: {e}"),
            }
        }

        // ── Copy Video ────────────────────────────────────────
        if !video_path.is_empty() {
            let vid_src = normalize_path(&video_path);
            println!("[snipSnap] Video resolved path: {}", vid_src.display());

            let vid_dir = base.join("video");
            match fs::create_dir_all(&vid_dir) {
                Ok(_) => {
                    let file_name = vid_src.file_name().unwrap_or_default();
                    let vid_dst = vid_dir.join(file_name);
                    match fs::copy(&vid_src, &vid_dst) {
                        Ok(_)  => println!("[snipSnap] Video copied → {}", vid_dst.display()),
                        Err(e) => eprintln!("[snipSnap] Video copy failed: {e}"),
                    }
                }
                Err(e) => eprintln!("[snipSnap] Could not create video dir: {e}"),
            }
        }
    });
}

/// Called when the user picks or drops a file.
/// The frontend reads the raw bytes via FileReader and passes them here,
/// because Tauri v2 webviews never expose real OS paths from <input type="file">
/// or drag-and-drop — only the filename is available.
///
/// Returns the final destination path so the frontend can display it.
#[tauri::command]
fn import_file_bytes(
    file_kind:    String,   // "srt" or "video"
    file_name:    String,
    bytes:        Vec<u8>,
    project_name: String,
) -> Result<String, String> {
    let base = project_base(&project_name);
    let sub_dir = if file_kind == "srt" { "srt" } else { "video" };
    let dest_dir = base.join(sub_dir);

    fs::create_dir_all(&dest_dir)
        .map_err(|e| format!("Could not create {} dir: {e}", sub_dir))?;

    let dest = dest_dir.join(&file_name);
    fs::write(&dest, &bytes)
        .map_err(|e| format!("{} write failed: {e}", file_kind))?;

    let dest_str = dest.to_string_lossy().into_owned();
    println!("[snipSnap] {} saved → {}", file_kind, dest_str);
    Ok(dest_str)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, import_project_files, import_file_bytes])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}