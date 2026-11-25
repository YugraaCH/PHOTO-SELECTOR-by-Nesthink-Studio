# Nesthink Photo Selector (v0.5.0)
App seleksi foto RAW, offline. Windows + macOS. Tauri 2 (Rust) + UI HTML murni.

## Windows
1. Install Rust (rustup.rs, dengan Visual Studio C++ Build Tools) dan Node.js LTS.
2. `npm install` lalu `npm run dev` (coba) atau `npm run build:win`
3. Hasil: src-tauri/target/release/bundle/nsis/ (installer) dan target/release/NESTHINK PHOTO SELECTOR.exe

## macOS (harus dibuild di Mac, minimal macOS 12)
1. `xcode-select --install`, install Rust (rustup.rs) dan Node.js LTS.
2. Untuk Mac ini saja:  `npm install` lalu `npm run build:mac`
   Untuk Intel + Apple Silicon sekaligus:
   `rustup target add aarch64-apple-darwin x86_64-apple-darwin` lalu `npm run build:mac:universal`
3. Hasil: .app dan .dmg di src-tauri/target/release/bundle/ (atau .../universal-apple-darwin/release/bundle/)
4. Aplikasi belum ditandatangani Apple. Kalau macOS memblokir: klik kanan app -> Open,
   atau jalankan `xattr -dr com.apple.quarantine "/Applications/NESTHINK PHOTO SELECTOR.app"`

## Tanpa Mac? Build otomatis di GitHub
Push folder ini ke repo GitHub -> tab Actions -> "build" -> Run workflow.
Hasil Windows (.exe) dan macOS (.dmg universal) bisa didownload dari Artifacts. (.github/workflows/build.yml)

## Struktur
- src-tauri/src/main.rs : logika (scan, pindah/salin file, undo, preview RAW, EXIF, project.json)
- ui/index.html         : seluruh tampilan + hotkey

## Controller (Xbox / PlayStation / standar PC)
Hubungkan controller, tekan satu tombol supaya terdeteksi. Default: D-pad atau stick kiri = Buangan/Terpilih/Masih mikir/Skip (kiri/kanan/atas/bawah),
tombol X/□ Y/△ B/○ A/✕ sebagai cadangan, LB/L1 = urungkan, RB/R1 = ulangi, Select = info, Start = grid, R3 = zoom. Bisa diubah di Pengaturan > Hotkey.

## Zoom
Scroll mouse (atau LT/RT di controller) langsung zoom ke titik kursor, tanpa menekan apa pun dulu. Space / R3 = zoom instan ke level yang diatur di Pengaturan > Umum.

## Grid (gaya Lightroom)
- Sel persegi, foto portrait maupun landscape tampil utuh (tidak terpotong).
- Ukuran thumbnail: Ctrl + scroll di area grid, atau slider "Ukuran" di bawah.
- Urutkan (toolbar bawah): nama file, waktu pengambilan, urutan ditambahkan, ukuran file, ISO, bukaan, focal length, shutter speed, status. Tombol panah = naik/turun.
- Foto terpilih berlatar lebih terang (putih) + garis biru; yang tidak dipilih abu-abu. Ctrl+A pilih semua, Esc batal pilih.
- Hotkey Terpilih / Masih mikir / Buangan / Kembali ke antrian berlaku untuk semua foto yang dipilih (pakai tombol yang sama seperti di layar seleksi; bisa diubah di Pengaturan > Hotkey). Controller juga bisa.
- "Kembali ke antrian" membuat foto jadi belum diputuskan lagi: mode Salin = salinan di folder hasil dihapus (file asli tidak disentuh); mode Pindah = foto dipindah balik ke folder asalnya. Bisa di-Urungkan.
- Panel kanan = detail metadata foto (nama, folder, status, kamera, lensa, focal length, bukaan, shutter, ISO, tanggal, dimensi, ukuran). Tombol "Metadata" di bawah untuk sembunyikan/tampilkan.

## Performa
Pengaturan > Performa: tombol "Buka folder cache" (membuka cache drive yang dipilih) dan "Hapus cache" (hanya thumbnail, foto asli aman).

## Update di dalam aplikasi
Pengaturan > Umum > Cek update (juga otomatis saat aplikasi dibuka). Setup dan cara rilis patch: lihat PANDUAN-AUTO-UPDATE.txt. Naikkan versi dengan `npm run bump 0.5.1`.
