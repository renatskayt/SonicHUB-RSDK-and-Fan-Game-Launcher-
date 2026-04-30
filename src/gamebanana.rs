use serde::Deserialize;
use std::io::Read;
use std::path::Path;

// GameBanana game IDs mapped by engine/game name
pub fn game_id_for_engine(engine: &str) -> Option<u64> {
    match engine {
        "RSDKv3" | "Sonic CD" => Some(6108),            // Sonic CD (2011)
        "RSDKv4" | "Sonic 1 (2013)" => Some(6620),       // Sonic the Hedgehog (2013)
        "Sonic 2 (2013)" => Some(6526),                   // Sonic 2 (2013)
        "RSDKv5" | "Sonic Mania" => Some(6045),          // Sonic Mania
        "Sonic 1 Forever" => Some(10601),                  // Sonic 1 Forever
        "Sonic 2 Absolute" => Some(15019),                 // Sonic 2 Absolute
        "Sonic 3 AIR" | "Sonic 3 A.I.R." => Some(6878),  // Sonic 3 A.I.R.
        _ => None,
    }
}

/// Emoji icon for engine type in sidebar
pub fn engine_icon(engine: &str) -> &'static str {
    match engine {
        "RSDKv3" => "💿",
        "RSDKv4" => "🎮",
        "RSDKv5" => "⚡",
        "Sonic 1 Forever" => "🏃",
        "Sonic 2 Absolute" => "🌀",
        "Sonic 3 AIR" => "✈️",
        _ => "🎮",
    }
}

#[derive(Debug, Deserialize)]
pub struct GBModFile {
    #[serde(rename = "_idRow")]
    pub id: u64,
    #[serde(rename = "_sFile")]
    pub filename: String,
    #[serde(rename = "_nFilesize")]
    pub filesize: u64,
    #[serde(rename = "_sDownloadUrl")]
    pub download_url: String,
    #[serde(rename = "_sDescription", default)]
    pub description: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GBSubmitter {
    #[serde(rename = "_sName")]
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct GBMod {
    #[serde(rename = "_sName")]
    pub name: String,
    #[serde(rename = "_nViewCount", default)]
    pub views: u64,
    #[serde(rename = "_nLikeCount", default)]
    pub likes: u64,
    #[serde(rename = "_aFiles", default)]
    pub files: Vec<GBModFile>,
    #[serde(rename = "_aSubmitter", default)]
    pub submitter: Option<GBSubmitter>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GBPreviewImage {
    #[serde(rename = "_sBaseUrl", default)]
    pub base_url: String,
    #[serde(rename = "_sFile", default)]
    pub file: String,
    #[serde(rename = "_sFile220", default)]
    pub file_220: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GBPreviewMedia {
    #[serde(rename = "_aImages", default)]
    pub images: Vec<GBPreviewImage>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GBSearchRecord {
    #[serde(rename = "_idRow")]
    pub id: u64,
    #[serde(rename = "_sName")]
    pub name: String,
    #[serde(rename = "_sText", default)]
    pub description: Option<String>,
    #[serde(rename = "_nViewCount", default)]
    pub views: u64,
    #[serde(rename = "_nLikeCount", default)]
    pub likes: u64,
    #[serde(rename = "_aSubmitter", default)]
    pub submitter: Option<GBSubmitter>,
    #[serde(rename = "_aPreviewMedia", default)]
    pub preview_media: Option<GBPreviewMedia>,
}

impl GBSearchRecord {
    pub fn thumb_url(&self) -> Option<String> {
        self.preview_media.as_ref().and_then(|pm| {
            pm.images.first().and_then(|img| {
                img.file_220.as_ref().map(|f220| {
                    format!("{}/{}", img.base_url, f220)
                })
            })
        })
    }
}

#[derive(Debug, Deserialize)]
pub struct GBSubfeedMeta {
    #[serde(rename = "_nRecordCount", default)]
    pub record_count: u64,
    #[serde(rename = "_nPerpage", default)]
    pub per_page: u64,
}

#[derive(Debug, Deserialize)]
pub struct GBSubfeedResponse {
    #[serde(rename = "_aMetadata")]
    pub metadata: GBSubfeedMeta,
    #[serde(rename = "_aRecords")]
    pub records: Vec<GBSearchRecord>,
}

/// Fetch mods list for a game from GameBanana
pub fn fetch_mods_list(game_id: u64, page: u32, sort: &str) -> Result<(Vec<GBSearchRecord>, u64), String> {
    let mut url = format!(
        "https://gamebanana.com/apiv11/Mod/Index?_nPage={}&_nPerpage=15&_aFilters[Generic_Game]={}",
        page, game_id
    );
    if !sort.is_empty() {
        url.push_str(&format!("&_sSort={}", sort));
    }
    let resp: GBSubfeedResponse = ureq::get(&url)
        .call()
        .map_err(|e| format!("Request failed: {}", e))?
        .body_mut()
        .read_json()
        .map_err(|e| format!("Parse failed: {}", e))?;
    Ok((resp.records, resp.metadata.record_count))
}

/// Search mods on GameBanana
pub fn search_mods(game_id: u64, query: &str, page: u32) -> Result<(Vec<GBSearchRecord>, u64), String> {
    let url = format!(
        "https://gamebanana.com/apiv11/Util/Search/Results?_sSearchString={}&_nPage={}&_nPerpage=15&_idGameRow={}&_sModelName=Mod",
        urlencoded(query), page, game_id
    );
    let resp: GBSubfeedResponse = ureq::get(&url)
        .call()
        .map_err(|e| format!("Search failed: {}", e))?
        .body_mut()
        .read_json()
        .map_err(|e| format!("Parse failed: {}", e))?;
    Ok((resp.records, resp.metadata.record_count))
}

fn urlencoded(s: &str) -> String {
    s.chars().map(|c| match c {
        'a'..='z' | 'A'..='Z' | '0'..='9' | '-' | '_' | '.' | '~' => c.to_string(),
        ' ' => "+".to_string(),
        _ => format!("%{:02X}", c as u8),
    }).collect()
}

/// Download a thumbnail image to cache, return local path
pub fn download_thumbnail(url: &str, cache_dir: &str) -> Option<String> {
    // FNV-1a hash for unique cache keys
    let mut hash: u64 = 0xcbf29ce484222325;
    for b in url.bytes() {
        hash ^= b as u64;
        hash = hash.wrapping_mul(0x100000001b3);
    }
    let filename = format!("thumb_{:016x}.jpg", hash);
    let path = Path::new(cache_dir).join(&filename);
    if path.exists() {
        return Some(path.to_string_lossy().to_string());
    }
    std::fs::create_dir_all(cache_dir).ok()?;
    let resp = ureq::get(url).call().ok()?;
    let mut data = Vec::new();
    resp.into_body().as_reader().read_to_end(&mut data).ok()?;
    std::fs::write(&path, &data).ok()?;
    Some(path.to_string_lossy().to_string())
}

/// Fetch mod details including files
pub fn fetch_mod_details(mod_id: u64) -> Result<GBMod, String> {
    let url = format!(
        "https://gamebanana.com/apiv11/Mod/{}?_csvProperties=_sName,_aFiles,_nViewCount,_nLikeCount,_aSubmitter",
        mod_id
    );
    let body: GBMod = ureq::get(&url)
        .call()
        .map_err(|e| format!("Request failed: {}", e))?
        .body_mut()
        .read_json()
        .map_err(|e| format!("Parse failed: {}", e))?;
    Ok(body)
}

/// Download a file from GameBanana and extract to mods folder
pub fn download_and_install_mod(
    download_url: &str,
    mods_folder: &str,
) -> Result<String, String> {
    std::fs::create_dir_all(mods_folder)
        .map_err(|e| format!("Cannot create mods dir: {}", e))?;

    let tmp_path = Path::new(mods_folder).join(".download_tmp");

    // Download
    let response = ureq::get(download_url)
        .call()
        .map_err(|e| format!("Download failed: {}", e))?;

    let mut data = Vec::new();
    response
        .into_body()
        .as_reader()
        .read_to_end(&mut data)
        .map_err(|e| format!("Read failed: {}", e))?;
    std::fs::write(&tmp_path, &data)
        .map_err(|e| format!("Write failed: {}", e))?;

    // Try zip first, then 7z as fallback
    let mod_name = match extract_zip(&tmp_path, mods_folder) {
        Ok(name) => name,
        Err(_) => extract_with_7z(&tmp_path, mods_folder)?,
    };

    let _ = std::fs::remove_file(&tmp_path);

    // Post-install: copy DLL/SO files to game directory (needed for Mania mods)
    let mod_dir = Path::new(mods_folder).join(&mod_name);
    if let Some(game_dir) = Path::new(mods_folder).parent() {
        if let Ok(entries) = std::fs::read_dir(&mod_dir) {
            for entry in entries.flatten() {
                let name = entry.file_name().to_string_lossy().to_string();
                let lower = name.to_lowercase();
                if lower.ends_with(".dll") || lower.ends_with(".so") {
                    let dest = game_dir.join(&name);
                    let _ = std::fs::copy(entry.path(), &dest);
                    eprintln!("Copied {} to game dir", name);
                }
            }
        }
    }

    Ok(mod_name)
}

fn extract_zip(zip_path: &Path, mods_folder: &str) -> Result<String, String> {
    let file = std::fs::File::open(zip_path)
        .map_err(|e| format!("Open zip failed: {}", e))?;
    let mut archive = zip::ZipArchive::new(file)
        .map_err(|e| format!("Invalid zip: {}", e))?;

    let mod_name = if let Some(name) = archive.file_names().next() {
        name.split('/').next().unwrap_or("unknown_mod").to_string()
    } else {
        "unknown_mod".to_string()
    };

    for i in 0..archive.len() {
        let mut entry = archive.by_index(i)
            .map_err(|e| format!("Zip entry error: {}", e))?;
        let outpath = Path::new(mods_folder).join(entry.name());

        if entry.is_dir() {
            std::fs::create_dir_all(&outpath).ok();
        } else {
            if let Some(parent) = outpath.parent() {
                std::fs::create_dir_all(parent).ok();
            }
            let mut outfile = std::fs::File::create(&outpath)
                .map_err(|e| format!("Create file failed: {}", e))?;
            std::io::copy(&mut entry, &mut outfile)
                .map_err(|e| format!("Extract failed: {}", e))?;
        }
    }
    Ok(mod_name)
}

fn extract_with_7z(archive_path: &Path, mods_folder: &str) -> Result<String, String> {
    // Try 7z command (p7zip-full package)
    let output = std::process::Command::new("7z")
        .args(["x", "-y", &archive_path.to_string_lossy(), &format!("-o{}", mods_folder)])
        .output()
        .map_err(|_| "7z not found. Install p7zip-full:\nsudo apt install p7zip-full".to_string())?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("7z extract failed: {}", stderr));
    }

    // Try to guess mod name from extracted dirs
    let stdout = String::from_utf8_lossy(&output.stdout);
    for line in stdout.lines() {
        if line.contains("- ") {
            let name = line.trim().trim_start_matches("- ");
            if let Some(dir) = name.split('/').next() {
                if !dir.is_empty() && Path::new(mods_folder).join(dir).is_dir() {
                    return Ok(dir.to_string());
                }
            }
        }
    }

    // Fallback: find newest directory
    if let Ok(entries) = std::fs::read_dir(mods_folder) {
        let mut newest: Option<(String, std::time::SystemTime)> = None;
        for entry in entries.flatten() {
            if entry.path().is_dir() && !entry.file_name().to_string_lossy().starts_with('.') {
                if let Ok(meta) = entry.metadata() {
                    if let Ok(modified) = meta.modified() {
                        if newest.as_ref().map_or(true, |(_, t)| modified > *t) {
                            newest = Some((entry.file_name().to_string_lossy().to_string(), modified));
                        }
                    }
                }
            }
        }
        if let Some((name, _)) = newest {
            return Ok(name);
        }
    }
    Ok("unknown_mod".to_string())
}
