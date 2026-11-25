#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet, VecDeque};
use std::path::{Path, PathBuf};
use std::{fs, sync::Mutex};
use std::io::Read;
use tauri::{ipc::Response, Emitter, State};

const RAW: &[&str] = &["dng","cr2","cr3","crw","nef","nrw","arw","srf","sr2","raf","orf","rw2","pef","srw","3fr","iiq","erf","kdc","mrw","x3f","rwl","gpr"];
const IMG: &[&str] = &["jpg", "jpeg", "png", "tif", "tiff", "heic"];

// st: 0 belum, 1 buangan, 2 terpilih, 3 masih mikir, 4 skip
#[derive(Serialize, Deserialize, Clone)]
struct Photo { src: String, cur: String, st: u8, #[serde(default)] dst: Vec<String> }
#[derive(Serialize, Deserialize, Clone)]
struct Entry { i: usize, from: String, prev: u8, #[serde(default)] grp: u32, #[serde(default = "yes")] inq: bool }
fn yes() -> bool { true }
#[derive(Serialize, Deserialize)]
struct Project { #[serde(default)] copy: bool, name: String, dir: String, names: [String; 3], photos: Vec<Photo>, queue: VecDeque<usize>, journal: Vec<Entry>, #[serde(skip)] redo: Vec<Vec<(usize, u8)>> }
struct App(Mutex<Option<Project>>);

#[derive(Serialize)]
struct View { copy: bool, name: String, dir: String, folders: Vec<String>, total: usize, done: usize, c: [usize; 4], cur: Option<usize>, cur_name: String, next: Vec<usize> }

fn view(p: &Project) -> View {
    let mut c = [0usize; 4];
    for ph in &p.photos { if ph.st > 0 { c[ph.st as usize - 1] += 1; } }
    let cur = p.queue.front().copied();
    let cur_name = cur.map(|i| Path::new(&p.photos[i].cur).file_name().unwrap_or_default().to_string_lossy().to_string()).unwrap_or_default();
    View { copy: p.copy, name: p.name.clone(), dir: p.dir.clone(), folders: p.names.to_vec(), total: p.photos.len(), done: c[0] + c[1] + c[2], c, cur, cur_name, next: p.queue.iter().skip(1).take(8).copied().collect() }
}
fn save(p: &Project) { let _ = fs::write(Path::new(&p.dir).join("project.json"), serde_json::to_vec(p).unwrap_or_default()); }
fn ext(p: &Path) -> String { p.extension().map(|e| e.to_string_lossy().to_lowercase()).unwrap_or_default() }

// Kumpulkan foto yang didukung. RAW+JPEG dengan nama sama dihitung 1 foto (RAW).
fn collect(paths: &[String]) -> Vec<String> {
    let mut all: Vec<PathBuf> = vec![];
    for p in paths {
        let p = Path::new(p);
        if p.is_dir() { all.extend(walkdir::WalkDir::new(p).into_iter().flatten().filter(|e| e.file_type().is_file()).map(|e| e.into_path())); }
        else { all.push(p.to_path_buf()); }
    }
    let key = |p: &Path| (p.parent().map(|x| x.to_path_buf()), p.file_stem().map(|s| s.to_string_lossy().to_lowercase()));
    let raws: HashSet<_> = all.iter().filter(|p| RAW.contains(&ext(p).as_str())).map(|p| key(p.as_path())).collect();
    let (mut seen, mut out) = (HashSet::new(), vec![]);
    for p in all {
        let e = ext(&p);
        let is_raw = RAW.contains(&e.as_str());
        if !is_raw && !IMG.contains(&e.as_str()) { continue; }
        if !is_raw && raws.contains(&key(p.as_path())) { continue; }
        let s = p.to_string_lossy().to_string();
        if seen.insert(s.clone()) { out.push(s); }
    }
    out.sort();
    out
}

// Nama file tidak pernah menimpa: tambah akhiran _1, _2, ...
fn unique(dir: &Path, name: &std::ffi::OsStr) -> PathBuf {
    let n = Path::new(name);
    let stem = n.file_stem().unwrap_or_default().to_string_lossy().to_string();
    let e = n.extension().map(|e| format!(".{}", e.to_string_lossy())).unwrap_or_default();
    let mut t = dir.join(name);
    let mut k = 1;
    while t.exists() { t = dir.join(format!("{stem}_{k}{e}")); k += 1; }
    t
}
fn mv(from: &Path, to: &Path) -> Result<(), String> {
    if fs::rename(from, to).is_ok() { return Ok(()); }
    fs::copy(from, to).map_err(|e| format!("Gagal memindahkan {}: {e}", from.display()))?;
    fs::remove_file(from).map_err(|e| e.to_string())
}
// File pendamping: .xmp, dan JPEG pasangan bila fotonya RAW
fn companions(from: &Path) -> Vec<PathBuf> {
    let raw = RAW.contains(&ext(from).as_str());
    let stem = from.file_stem().map(|s| s.to_string_lossy().to_lowercase());
    let Ok(rd) = fs::read_dir(from.parent().unwrap_or(Path::new("."))) else { return vec![] };
    rd.flatten().map(|e| e.path()).filter(|p| p != from && p.is_file() && p.file_stem().map(|s| s.to_string_lossy().to_lowercase()) == stem
        && (ext(p) == "xmp" || (raw && matches!(ext(p).as_str(), "jpg" | "jpeg")))).collect()
}
fn relocate(from: &Path, to: PathBuf) -> Result<PathBuf, String> {
    let dir = to.parent().unwrap().to_path_buf();
    mv(from, &to)?;
    for c in companions(from) {
        let mut t = dir.join(to.file_stem().unwrap());
        t.set_extension(c.extension().unwrap());
        if !t.exists() { let _ = mv(&c, &t); }
    }
    Ok(to)
}

// Preview RAW: ambil JPEG tertanam terbesar (semua merek menyimpannya). Tanpa decode RAW penuh, jadi cepat.
fn jpeg_end(d: &[u8], s: usize) -> usize {
    let mut i = s + 2;
    while i + 4 <= d.len() {
        if d[i] != 0xFF { return s + 3; }
        let m = d[i + 1];
        if m == 0xFF { i += 1; continue; }
        if m == 0xD9 { return i + 2; }
        if m == 0x01 || (0xD0..=0xD8).contains(&m) { i += 2; continue; }
        let l = ((d[i + 2] as usize) << 8) | d[i + 3] as usize;
        i += 2 + l;
        if m == 0xDA {
            while i + 1 < d.len() && !(d[i] == 0xFF && d[i + 1] != 0 && !(0xD0..=0xD7).contains(&d[i + 1])) { i += 1; }
        }
    }
    s + 3
}
fn load_preview(p: &Path) -> Result<Vec<u8>, String> {
    let d = fs::read(p).map_err(|e| e.to_string())?;
    if matches!(ext(p).as_str(), "jpg" | "jpeg" | "png") { return Ok(d); }
    let (mut best, mut i) = ((0usize, 0usize), 0usize);
    while i + 3 < d.len() {
        if d[i] == 0xFF && d[i + 1] == 0xD8 && d[i + 2] == 0xFF {
            let e = jpeg_end(&d, i);
            if e - i > best.1 - best.0 { best = (i, e); }
            i = e.max(i + 3);
        } else { i += 1; }
    }
    if best.1 - best.0 > 20_000 { Ok(d[best.0..best.1].to_vec()) } else { Err("Preview tidak ditemukan di file ini".into()) }
}

// ---- Cache thumbnail: beberapa disk berurutan, seperti Scratch Disks di Photoshop ----
static CACHE_BASES: Mutex<Vec<PathBuf>> = Mutex::new(Vec::new());
static DISK_SNAP: Mutex<Option<(std::time::Instant, Vec<(PathBuf, u64)>)>> = Mutex::new(None);
fn is_startup(m: &Path) -> bool {
    if cfg!(target_os = "windows") {
        let sd = std::env::var("SystemDrive").unwrap_or_else(|_| "C:".into()).to_lowercase();
        m.to_string_lossy().to_lowercase().starts_with(&sd)
    } else { m == Path::new("/") }
}
fn default_cache_base() -> PathBuf {
    std::env::var_os("LOCALAPPDATA").map(PathBuf::from)
        .or_else(|| std::env::var_os("HOME").map(|h| PathBuf::from(h).join("Library/Caches")))
        .unwrap_or_else(std::env::temp_dir)
        .join("NESTHINK PHOTO SELECTOR")
}
fn base_for(m: &Path) -> PathBuf { if is_startup(m) { default_cache_base() } else { m.join("NESTHINK PHOTO SELECTOR") } }
fn cache_dirs() -> Vec<PathBuf> {
    let v = CACHE_BASES.lock().unwrap().clone();
    let v = if v.is_empty() { vec![default_cache_base()] } else { v };
    v.into_iter().map(|b| b.join("thumbs-cache")).collect()
}
fn free_bytes(p: &Path) -> Option<u64> {
    let mut g = DISK_SNAP.lock().unwrap();
    if g.as_ref().map_or(true, |(t, _)| t.elapsed().as_secs() > 30) {
        let d = sysinfo::Disks::new_with_refreshed_list();
        *g = Some((std::time::Instant::now(), d.iter().map(|x| (x.mount_point().to_path_buf(), x.available_space())).collect()));
    }
    g.as_ref()?.1.iter().filter(|(m, _)| p.starts_with(m)).max_by_key(|(m, _)| m.as_os_str().len()).map(|(_, f)| *f)
}
fn dir_size(d: &Path) -> u64 {
    fs::read_dir(d).map(|r| r.flatten().filter_map(|e| e.metadata().ok()).map(|m| m.len()).sum()).unwrap_or(0)
}
#[tauri::command]
fn set_cache_disks(mounts: Vec<String>) {
    *CACHE_BASES.lock().unwrap() = mounts.iter().map(|m| base_for(Path::new(m))).collect();
}
#[derive(Serialize)]
struct DiskInfo { mount: String, label: String, free_mb: u64, total_mb: u64, startup: bool, cache_mb: f64, cache_path: String }
#[tauri::command(async)]
fn disk_list() -> Vec<DiskInfo> {
    let ds = sysinfo::Disks::new_with_refreshed_list();
    let mut out: Vec<DiskInfo> = vec![];
    for d in ds.iter() {
        let m = d.mount_point().to_path_buf();
        let ms = m.to_string_lossy().to_string();
        if d.total_space() == 0 || out.iter().any(|x| x.mount == ms) { continue; }
        if cfg!(target_os = "macos") && !(ms == "/" || ms.starts_with("/Volumes/")) { continue; }
        let st = is_startup(&m);
        let cp = base_for(&m).join("thumbs-cache");
        let cm = dir_size(&cp) as f64 / 1048576.0;
        out.push(DiskInfo { mount: ms, label: d.name().to_string_lossy().into(), free_mb: d.available_space() / 1048576, total_mb: d.total_space() / 1048576, startup: st, cache_mb: cm, cache_path: cp.to_string_lossy().to_string() });
    }
    out.sort_by(|a, b| b.startup.cmp(&a.startup).then(a.mount.cmp(&b.mount)));
    out
}
#[tauri::command(async)]
fn clear_cache() -> usize {
    let mut dirs = cache_dirs();
    for d in sysinfo::Disks::new_with_refreshed_list().iter() { dirs.push(base_for(d.mount_point()).join("thumbs-cache")); }
    let mut n = 0;
    for dir in dirs {
        if let Ok(r) = fs::read_dir(&dir) {
            for e in r.flatten() { let p = e.path(); if ext(&p) == "jpg" && fs::remove_file(&p).is_ok() { n += 1; } }
        }
    }
    n
}
#[derive(Serialize)]
struct SysInfo { total_mb: u64, free_mb: u64 }
#[tauri::command(async)]
fn sys_info() -> SysInfo {
    let mut s = sysinfo::System::new();
    s.refresh_memory();
    SysInfo { total_mb: s.total_memory() / 1048576, free_mb: s.available_memory() / 1048576 }
}
fn thumb_key(p: &Path) -> Option<String> {
    use std::hash::{Hash, Hasher};
    let m = fs::metadata(p).ok()?;
    let mut h = std::collections::hash_map::DefaultHasher::new();
    p.file_name()?.hash(&mut h);
    m.len().hash(&mut h);
    m.modified().ok()?.duration_since(std::time::UNIX_EPOCH).ok()?.as_secs().hash(&mut h);
    Some(format!("{:016x}.jpg", h.finish()))
}
// tulis ke disk aktif pertama yang ruangnya masih >= 2 GB; kalau penuh/gagal, pindah ke disk berikutnya
fn write_cache(key: &str, data: &[u8]) {
    for d in cache_dirs() {
        if free_bytes(&d).map_or(false, |f| f < 2_000_000_000) { continue; }
        if fs::create_dir_all(&d).is_ok() && fs::write(d.join(key), data).is_ok() { return; }
    }
}
// Thumbnail grid: ambil JPEG tertanam terkecil (>= 40 KB) dari 10 MB pertama file, lalu simpan di cache disk.
fn load_thumb(p: &Path) -> Result<Vec<u8>, String> {
    let key = thumb_key(p);
    if let Some(k) = &key {
        for d in cache_dirs() { if let Ok(b) = fs::read(d.join(k)) { if !b.is_empty() { return Ok(b); } } }
    }
    let out = if matches!(ext(p).as_str(), "jpg" | "jpeg" | "png") {
        fs::read(p).map_err(|e| e.to_string())?
    } else {
        const LIM: u64 = 10_000_000;
        let mut d = Vec::new();
        fs::File::open(p).map_err(|e| e.to_string())?.take(LIM).read_to_end(&mut d).map_err(|e| e.to_string())?;
        let trunc = d.len() as u64 >= LIM;
        let mut cands: Vec<(usize, usize)> = vec![];
        let mut i = 0usize;
        while i + 3 < d.len() {
            if d[i] == 0xFF && d[i + 1] == 0xD8 && d[i + 2] == 0xFF {
                let e = jpeg_end(&d, i).min(d.len());
                cands.push((i, e));
                i = e.max(i + 3);
            } else { i += 1; }
        }
        match cands.into_iter().filter(|c| c.1 - c.0 >= 40_000 && (!trunc || c.1 + 4 < d.len())).min_by_key(|c| c.1 - c.0) {
            Some((a, b)) => d[a..b].to_vec(),
            None => return load_preview(p),
        }
    };
    if let Some(k) = key { write_cache(&k, &out); }
    Ok(out)
}

// Kirim progres ke UI (jumlah selesai, total). total 0 = tidak diketahui
fn prog(h: &tauri::AppHandle, done: usize, total: usize) { let _ = h.emit("op-progress", (done, total)); }
fn with<T>(app: &State<App>, f: impl FnOnce(&mut Project) -> Result<T, String>) -> Result<T, String> {
    let mut g = app.0.lock().unwrap();
    f(g.as_mut().ok_or("Belum ada project yang dibuka")?)
}

#[tauri::command(async)]
fn new_project(app: State<App>, name: String, base: String, names: Vec<String>, paths: Vec<String>, copy: bool) -> Result<View, String> {
    let clean = |s: &str| s.replace(|c: char| "<>:\"/\\|?*".contains(c), "_").trim().to_string();
    let name = clean(&name);
    if name.is_empty() { return Err("Isi nama project dulu".into()); }
    if base.is_empty() { return Err("Pilih lokasi project dulu".into()); }
    let files = collect(&paths);
    if files.is_empty() { return Err("Tidak ada foto yang didukung di sumber ini".into()); }
    let dir = Path::new(&base).join(&name);
    let mut n: [String; 3] = ["buangan".into(), "terpilih".into(), "masih mikir".into()];
    for (i, x) in names.iter().take(3).enumerate() { let c = clean(&x.replace("{project}", &name)); if !c.is_empty() { n[i] = c; } }
    for x in &n { fs::create_dir_all(dir.join(x)).map_err(|e| e.to_string())?; }
    let photos: Vec<Photo> = files.into_iter().map(|s| Photo { cur: s.clone(), src: s, st: 0, dst: vec![] }).collect();
    let queue = (0..photos.len()).collect();
    let p = Project { copy, name, dir: dir.to_string_lossy().into(), names: n, photos, queue, journal: vec![], redo: vec![] };
    save(&p);
    let v = view(&p);
    *app.0.lock().unwrap() = Some(p);
    Ok(v)
}
#[tauri::command(async)]
fn open_project(app: State<App>, dir: String) -> Result<View, String> {
    let b = fs::read(Path::new(&dir).join("project.json")).map_err(|_| "Folder ini bukan project Nesthink (project.json tidak ada)")?;
    let mut p: Project = serde_json::from_slice(&b).map_err(|e| e.to_string())?;
    p.dir = dir;
    let v = view(&p);
    *app.0.lock().unwrap() = Some(p);
    Ok(v)
}
#[tauri::command(async)]
fn add_photos(app: State<App>, paths: Vec<String>) -> Result<View, String> {
    with(&app, |p| {
        let have: HashSet<String> = p.photos.iter().map(|x| x.src.clone()).collect();
        for s in collect(&paths).into_iter().filter(|s| !have.contains(s)) {
            p.photos.push(Photo { cur: s.clone(), src: s, st: 0, dst: vec![] });
            p.queue.push_back(p.photos.len() - 1);
        }
        save(p);
        Ok(view(p))
    })
}
// Mode Pindahkan: file asli dipindah. Mode Salin: file asli tidak disentuh, hanya salinannya yang ditaruh/dipindah antar folder hasil.
fn duplicate(from: &Path, to: PathBuf) -> Result<Vec<String>, String> {
    let dir = to.parent().unwrap().to_path_buf();
    fs::copy(from, &to).map_err(|e| format!("Gagal menyalin {}: {e}", from.display()))?;
    let mut out = vec![to.to_string_lossy().to_string()];
    for c in companions(from) {
        let mut t = dir.join(to.file_stem().unwrap());
        t.set_extension(c.extension().unwrap());
        if !t.exists() && fs::copy(&c, &t).is_ok() { out.push(t.to_string_lossy().to_string()); }
    }
    Ok(out)
}
fn drop_copy(v: &[String]) { for f in v { let _ = fs::remove_file(f); } } // hanya file salinan buatan aplikasi
fn put(p: &mut Project, i: usize, act: u8) -> Result<(), String> {
    let d = Path::new(&p.dir).join(&p.names[act as usize - 1]);
    fs::create_dir_all(&d).map_err(|e| e.to_string())?;
    if p.copy {
        drop_copy(&p.photos[i].dst);
        let from = PathBuf::from(&p.photos[i].src);
        let to = unique(&d, from.file_name().unwrap());
        p.photos[i].dst = duplicate(&from, to)?;
    } else {
        let from = p.photos[i].cur.clone();
        let to = unique(&d, Path::new(&from).file_name().unwrap());
        p.photos[i].cur = relocate(Path::new(&from), to)?.to_string_lossy().into();
    }
    Ok(())
}
// Kembalikan ke antrian seleksi. Mode salin: salinan di folder hasil dihapus. Mode pindah: foto dipindah balik ke folder asalnya.
fn unput(p: &mut Project, i: usize) -> Result<(), String> {
    if p.copy {
        drop_copy(&p.photos[i].dst);
        p.photos[i].dst.clear();
    } else {
        let f = PathBuf::from(&p.photos[i].src);
        let cur = PathBuf::from(&p.photos[i].cur);
        if cur.parent() != f.parent() {
            let dir = f.parent().unwrap_or(Path::new(".")).to_path_buf();
            fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
            let to = unique(&dir, f.file_name().unwrap());
            p.photos[i].cur = relocate(&cur, to)?.to_string_lossy().into();
        }
    }
    Ok(())
}
// act: 0 = kembali ke antrian, 1..=3 = folder hasil, 4 = skip
fn apply(p: &mut Project, i: usize, act: u8, grp: u32) -> Result<(), String> {
    let (prev, from) = (p.photos[i].st, p.photos[i].cur.clone());
    let inq = p.queue.contains(&i);
    if prev != act {
        if (1..=3).contains(&act) { put(p, i, act)?; }
        else if act == 0 { unput(p, i)?; }
        p.photos[i].st = act;
        p.journal.push(Entry { i, from, prev, grp, inq });
    }
    if act == 0 {
        if !p.queue.contains(&i) { p.queue.push_back(i); }
    } else {
        p.queue.retain(|&x| x != i);
    }
    Ok(())
}
fn undo_one(p: &mut Project, e: &Entry) -> Result<(), String> {
    let st = p.photos[e.i].st;
    if p.copy {
        if (1..=3).contains(&st) { drop_copy(&p.photos[e.i].dst); p.photos[e.i].dst.clear(); }
        if (1..=3).contains(&e.prev) { put(p, e.i, e.prev)?; }
    } else {
        let cur = PathBuf::from(&p.photos[e.i].cur);
        let f = PathBuf::from(&e.from);
        if cur.parent() != f.parent() {
            let dir = f.parent().unwrap_or(Path::new(".")).to_path_buf();
            fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
            let to = unique(&dir, f.file_name().unwrap());
            p.photos[e.i].cur = relocate(&cur, to)?.to_string_lossy().into();
        }
    }
    p.photos[e.i].st = e.prev;
    if e.inq { if !p.queue.contains(&e.i) { p.queue.push_front(e.i); } }
    else { p.queue.retain(|&x| x != e.i); }
    Ok(())
}

#[tauri::command(async)]
fn decide(app: State<App>, act: u8) -> Result<View, String> {
    with(&app, |p| { let i = *p.queue.front().ok_or("Antrian kosong")?; apply(p, i, act, 0)?; p.redo.clear(); save(p); Ok(view(p)) })
}
// Ubah keputusan banyak foto sekaligus (Grid). Satu Ctrl+Z membatalkan satu grup.
#[tauri::command(async)]
fn set_status(h: tauri::AppHandle, app: State<App>, idx: Vec<usize>, act: u8) -> Result<View, String> {
    with(&app, |p| {
        if act > 3 { return Err("Aksi tidak valid".into()); }
        let grp = p.journal.iter().map(|e| e.grp).max().unwrap_or(0) + 1;
        p.redo.clear();
        let mut r = Ok(());
        let total = idx.len();
        prog(&h, 0, total);
        for (n, i) in idx.into_iter().enumerate() {
            if i < p.photos.len() { if let Err(e) = apply(p, i, act, grp) { r = Err(e); break; } }
            prog(&h, n + 1, total);
        }
        save(p); r?; Ok(view(p))
    })
}
#[tauri::command(async)]
fn undo(h: tauri::AppHandle, app: State<App>) -> Result<View, String> {
    with(&app, |p| {
        let last = p.journal.last().cloned().ok_or("Belum ada yang bisa diurungkan")?;
        let total = if last.grp == 0 { 1 } else { p.journal.iter().filter(|e| e.grp == last.grp).count() };
        prog(&h, 0, total);
        let mut first = true;
        let mut done: Vec<(usize, u8)> = vec![];
        while let Some(e) = p.journal.last().cloned() {
            if !first && (last.grp == 0 || e.grp != last.grp) { break; }
            done.push((e.i, p.photos[e.i].st));
            undo_one(p, &e)?; p.journal.pop(); first = false;
            prog(&h, done.len(), total);
            if last.grp == 0 { break; }
        }
        done.reverse(); p.redo.push(done);
        save(p); Ok(view(p))
    })
}
// Ulangi keputusan yang baru diurungkan (satu grup Grid = satu langkah)
#[tauri::command(async)]
fn redo(h: tauri::AppHandle, app: State<App>) -> Result<View, String> {
    with(&app, |p| {
        let g = p.redo.pop().ok_or("Belum ada yang bisa diulang")?;
        let total = g.len();
        prog(&h, 0, total);
        let grp = if g.len() > 1 { p.journal.iter().map(|e| e.grp).max().unwrap_or(0) + 1 } else { 0 };
        for (n, (i, act)) in g.into_iter().enumerate() { apply(p, i, act, grp)?; prog(&h, n + 1, total); }
        save(p); Ok(view(p))
    })
}
// Ganti mode Pindahkan/Salin project yang sedang dibuka. Hanya boleh sebelum ada foto yang diputuskan, supaya file tidak tercampur.
#[tauri::command]
fn set_mode(app: State<App>, copy: bool) -> Result<View, String> {
    with(&app, |p| {
        if p.photos.iter().any(|x| x.st != 0) { return Err("Mode tidak bisa diubah setelah ada foto yang diputuskan. Buat project baru untuk mode lain.".into()); }
        p.copy = copy; save(p); Ok(view(p))
    })
}
// Buka link di browser bawaan (hanya link tetap milik developer, bukan alamat bebas dari UI)
fn open_link(url: &str) -> Result<(), String> {
    let mut c = if cfg!(windows) { let mut c = std::process::Command::new("rundll32"); c.arg("url.dll,FileProtocolHandler"); c }
        else if cfg!(target_os = "macos") { std::process::Command::new("open") } else { std::process::Command::new("xdg-open") };
    c.arg(url).spawn().map(|_| ()).map_err(|e| e.to_string())
}
// Chat WhatsApp developer
#[tauri::command]
fn open_wa() -> Result<(), String> {
    open_link("https://wa.me/6281222484008?text=Halo%20Nesthink%2C%20saya%20mau%20tanya%20atau%20lapor%20bug%20di%20Photo%20Selector")
}
// Halaman donasi / support developer
#[tauri::command]
fn open_support() -> Result<(), String> {
    open_link("https://saweria.co/Nesthinkstudio")
}
#[tauri::command]
fn goto(app: State<App>, i: usize) -> Result<View, String> {
    with(&app, |p| { if i >= p.photos.len() { return Err("Foto tidak ada".into()); } p.queue.retain(|&x| x != i); p.queue.push_front(i); p.redo.clear(); save(p); Ok(view(p)) })
}
fn requeue(app: &State<App>, st: u8) -> Result<View, String> {
    with(app, |p| { p.queue = p.photos.iter().enumerate().filter(|(_, x)| x.st == st).map(|(i, _)| i).collect(); p.redo.clear(); save(p); Ok(view(p)) })
}
#[tauri::command]
fn continue_skips(app: State<App>) -> Result<View, String> { requeue(&app, 4) }
#[tauri::command]
fn review_mikir(app: State<App>) -> Result<View, String> { requeue(&app, 3) }

#[derive(Serialize)]
struct Item { n: String, st: u8, d: String }
#[tauri::command]
fn list(app: State<App>) -> Result<Vec<Item>, String> {
    with(&app, |p| Ok(p.photos.iter().map(|x| {
        let c = Path::new(&x.cur);
        Item {
            n: c.file_name().unwrap_or_default().to_string_lossy().into(),
            st: x.st,
            d: c.parent().and_then(|d| d.file_name()).unwrap_or_default().to_string_lossy().into(),
        }
    }).collect()))
}
#[tauri::command(async)]
fn preview(app: State<App>, i: usize) -> Result<Response, String> {
    let path = with(&app, |p| p.photos.get(i).map(|x| x.cur.clone()).ok_or("Foto tidak ada".to_string()))?;
    load_preview(Path::new(&path)).map(Response::new)
}

#[tauri::command(async)]
fn thumb(app: State<App>, i: usize) -> Result<Response, String> {
    let path = with(&app, |p| p.photos.get(i).map(|x| x.cur.clone()).ok_or("Foto tidak ada".to_string()))?;
    load_thumb(Path::new(&path)).map(Response::new)
}

// Ukuran piksel dari header JPEG (SOF)
fn jpeg_dims(d: &[u8]) -> Option<(u32, u32)> {
    let mut i = 2usize;
    while i + 9 < d.len() {
        if d[i] != 0xFF { i += 1; continue; }
        let m = d[i + 1];
        if m == 0xFF { i += 1; continue; }
        if (0xC0..=0xCF).contains(&m) && m != 0xC4 && m != 0xC8 && m != 0xCC {
            let h = ((d[i + 5] as u32) << 8) | d[i + 6] as u32;
            let w = ((d[i + 7] as u32) << 8) | d[i + 8] as u32;
            return Some((w, h));
        }
        if m == 0x01 || (0xD0..=0xD9).contains(&m) { i += 2; continue; }
        let l = ((d[i + 2] as usize) << 8) | d[i + 3] as usize;
        i += 2 + l;
    }
    None
}
#[derive(Serialize, Default)]
struct Info { cam: String, lens: String, focal: String, fnum: String, shutter: String, iso: String, date: String, size: String, dim: String }
#[tauri::command(async)]
fn info(app: State<App>, i: usize) -> Result<Info, String> {
    let path = with(&app, |p| p.photos.get(i).map(|x| x.cur.clone()).ok_or("Foto tidak ada".to_string()))?;
    let path = Path::new(&path);
    let mut o = Info::default();
    o.size = fs::metadata(path).map(|m| format!("{:.1} MB", m.len() as f64 / 1048576.0)).unwrap_or_default();
    // EXIF biasanya ada di awal file: baca 6 MB pertama saja supaya cepat
    let head = fs::File::open(path).ok().and_then(|f| { let mut b = vec![]; f.take(6_000_000).read_to_end(&mut b).ok()?; Some(b) });
    let rd = |b: Vec<u8>| exif::Reader::new().read_from_container(&mut std::io::Cursor::new(b)).ok();
    if matches!(ext(path).as_str(), "jpg" | "jpeg") {
        if let Some((w, h)) = head.as_ref().and_then(|b| jpeg_dims(b)) { o.dim = format!("{} × {}", w, h); }
    }
    if let Some(e) = head.and_then(rd).or_else(|| load_preview(path).ok().and_then(rd)) {
        let g = |t: exif::Tag| e.get_field(t, exif::In::PRIMARY).map(|f| f.display_value().with_unit(&e).to_string().trim_matches('"').to_string()).unwrap_or_default();
        o.cam = g(exif::Tag::Model); o.lens = g(exif::Tag::LensModel); o.focal = g(exif::Tag::FocalLength);
        o.fnum = g(exif::Tag::FNumber); o.shutter = g(exif::Tag::ExposureTime); o.iso = g(exif::Tag::PhotographicSensitivity); o.date = g(exif::Tag::DateTimeOriginal);
        if o.dim.is_empty() {
            let u = |t: exif::Tag| e.get_field(t, exif::In::PRIMARY).and_then(|f| f.value.get_uint(0)).unwrap_or(0);
            let (w, h) = (u(exif::Tag::PixelXDimension), u(exif::Tag::PixelYDimension));
            let (w, h) = if w > 0 && h > 0 { (w, h) } else { (u(exif::Tag::ImageWidth), u(exif::Tag::ImageLength)) };
            if w > 0 && h > 0 { o.dim = format!("{} × {}", w, h); }
        }
    }
    Ok(o)
}
// ---- Data untuk mengurutkan Grid (waktu ambil, ISO, bukaan, dll). Dihitung sekali per foto lalu disimpan di memori ----
#[derive(Serialize, Clone, Default)]
struct SortMeta { date: String, size: u64, mtime: u64, f: f64, fl: f64, sh: f64, iso: u32 }
static SORTM: Mutex<Option<HashMap<String, SortMeta>>> = Mutex::new(None);
fn exif_for_sort(path: &Path) -> Option<exif::Exif> {
    let rd = |lim: u64| -> Option<exif::Exif> {
        let mut b = Vec::new();
        fs::File::open(path).ok()?.take(lim).read_to_end(&mut b).ok()?;
        exif::Reader::new().read_from_container(&mut std::io::Cursor::new(b)).ok()
    };
    let ok = |e: &exif::Exif| e.get_field(exif::Tag::DateTimeOriginal, exif::In::PRIMARY).is_some() || e.get_field(exif::Tag::FNumber, exif::In::PRIMARY).is_some();
    if let Some(e) = rd(1_500_000) { if ok(&e) { return Some(e); } }
    if let Some(e) = rd(6_000_000) { return Some(e); }
    load_preview(path).ok().and_then(|b| exif::Reader::new().read_from_container(&mut std::io::Cursor::new(b)).ok())
}
fn rat(e: &exif::Exif, t: exif::Tag) -> f64 {
    match e.get_field(t, exif::In::PRIMARY).map(|f| &f.value) {
        Some(exif::Value::Rational(v)) => v.first().map(|r| r.to_f64()).unwrap_or(0.0),
        _ => 0.0,
    }
}
fn sort_meta_of(path: &Path) -> SortMeta {
    let mut m = SortMeta::default();
    if let Ok(md) = fs::metadata(path) {
        m.size = md.len();
        m.mtime = md.modified().ok().and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok()).map(|d| d.as_secs()).unwrap_or(0);
    }
    if let Some(e) = exif_for_sort(path) {
        m.date = e.get_field(exif::Tag::DateTimeOriginal, exif::In::PRIMARY).map(|f| f.display_value().to_string().trim_matches('"').to_string()).unwrap_or_default();
        m.f = rat(&e, exif::Tag::FNumber);
        m.fl = rat(&e, exif::Tag::FocalLength);
        m.sh = rat(&e, exif::Tag::ExposureTime);
        m.iso = e.get_field(exif::Tag::PhotographicSensitivity, exif::In::PRIMARY).and_then(|f| f.value.get_uint(0)).unwrap_or(0);
    }
    if m.date.is_empty() { m.date = format!("~{:020}", m.mtime); } // tidak ada EXIF: pakai waktu modifikasi file, diurutkan setelah yang ber-EXIF
    m
}
#[tauri::command(async)]
fn sort_meta(app: State<App>) -> Result<Vec<SortMeta>, String> {
    let items: Vec<(String, String)> = with(&app, |p| Ok(p.photos.iter().map(|x| (x.src.clone(), x.cur.clone())).collect()))?;
    let have: HashMap<String, SortMeta> = SORTM.lock().unwrap().clone().unwrap_or_default();
    let todo: Vec<&(String, String)> = items.iter().filter(|(s, _)| !have.contains_key(s)).collect();
    let mut fresh: HashMap<String, SortMeta> = HashMap::new();
    if !todo.is_empty() {
        let n = std::thread::available_parallelism().map(|x| x.get()).unwrap_or(4).min(8).max(1);
        let size = (todo.len() + n - 1) / n;
        let parts: Vec<Vec<(String, SortMeta)>> = std::thread::scope(|sc| {
            let hs: Vec<_> = todo.chunks(size.max(1)).map(|ch| sc.spawn(move || {
                ch.iter().map(|(s, c)| (s.clone(), sort_meta_of(Path::new(c)))).collect::<Vec<(String, SortMeta)>>()
            })).collect();
            hs.into_iter().filter_map(|h| h.join().ok()).collect()
        });
        for part in parts { for (k, v) in part { fresh.insert(k, v); } }
        let mut g = SORTM.lock().unwrap();
        let m = g.get_or_insert_with(HashMap::new);
        for (k, v) in &fresh { m.insert(k.clone(), v.clone()); }
    }
    Ok(items.iter().map(|(s, _)| fresh.get(s).or_else(|| have.get(s)).cloned().unwrap_or_default()).collect())
}
// Buka folder cache thumbnail di Explorer/Finder. mount = drive yang dipilih (kosong: cache pertama yang ada)
#[tauri::command]
fn open_cache(mount: Option<String>) -> Result<String, String> {
    let dirs = cache_dirs();
    let d = match mount.filter(|m| !m.is_empty()) {
        Some(m) => base_for(Path::new(&m)).join("thumbs-cache"),
        None => dirs.iter().find(|x| x.exists()).cloned().unwrap_or_else(|| dirs[0].clone()),
    };
    fs::create_dir_all(&d).map_err(|e| e.to_string())?;
    std::process::Command::new(if cfg!(windows) { "explorer" } else if cfg!(target_os = "macos") { "open" } else { "xdg-open" }).arg(&d).spawn().map_err(|e| e.to_string())?;
    Ok(d.to_string_lossy().to_string())
}
// Kosongkan folder buangan ke Recycle Bin (bukan hapus permanen)
#[tauri::command(async)]
fn empty_buangan(app: State<App>) -> Result<usize, String> {
    let dir = with(&app, |p| Ok(Path::new(&p.dir).join(&p.names[0])))?;
    let files: Vec<PathBuf> = fs::read_dir(&dir).map_err(|e| e.to_string())?.flatten().map(|e| e.path()).filter(|p| p.is_file()).collect();
    let n = files.len();
    trash::delete_all(files).map_err(|e| e.to_string())?;
    with(&app, |p| { p.journal.clear(); p.redo.clear(); save(p); Ok(()) })?;
    Ok(n)
}
#[tauri::command]
fn open_folder(app: State<App>, which: usize) -> Result<(), String> {
    let d = with(&app, |p| Ok(Path::new(&p.dir).join(&p.names[which.min(2)])))?;
    std::process::Command::new(if cfg!(windows) { "explorer" } else if cfg!(target_os = "macos") { "open" } else { "xdg-open" }).arg(d).spawn().map(|_| ()).map_err(|e| e.to_string())
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .manage(App(Mutex::new(None)))
        .invoke_handler(tauri::generate_handler![new_project, open_project, add_photos, decide, set_status, undo, redo, set_mode, open_wa, open_support, goto, continue_skips, review_mikir, list, preview, thumb, set_cache_disks, disk_list, clear_cache, sys_info, info, sort_meta, open_cache, empty_buangan, open_folder])
        .run(tauri::generate_context!())
        .expect("gagal menjalankan aplikasi");
}
