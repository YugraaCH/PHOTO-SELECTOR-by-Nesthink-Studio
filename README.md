# Nesthink Photo Selector (v0.5.0)

**Seleksi ratusan foto hasil pemotretan tanpa pusing. Gratis, offline, ringan.**

Setelah selesai motret, sering kali kita pulang dengan ratusan sampai ribuan foto. Untuk memilih mana yang
dipakai, kita biasanya membuka satu per satu di Explorer atau Lightroom, memindahkan file ke folder manual,
lalu lupa mana yang sudah dicek dan mana yang belum. Foto RAW yang berat juga bikin proses ini lambat dan
melelahkan.

Nesthink Photo Selector dibuat untuk satu hal itu: **menyeleksi foto dengan cepat.** Satu foto tampil besar,
kamu tinggal tekan satu tombol (**Terpilih**, **Buangan**, **Masih mikir**, atau **Skip**), dan aplikasi yang
mengurus pemindahan file-nya. Foto berikutnya sudah dimuat lebih dulu, jadi alurnya mengalir tanpa menunggu.

---

## Fitur

**Seleksi cepat**
- Satu foto per layar, keputusan lewat satu tombol: Terpilih, Buangan, Masih mikir, atau Skip.
- Foto berikutnya dimuat di latar belakang (preload), jadi nggak ada jeda saat pindah foto.
- Urungkan dan Ulangi (Ctrl+Z / Ctrl+Y) tanpa batas langkah sampai kamu tutup project.
- Zoom langsung ke titik kursor dengan scroll mouse, untuk cek fokus dan ketajaman.
- Detail info foto: nama file, kamera, lensa, focal length, bukaan, shutter speed, ISO.

**Mendukung foto RAW**
- Canon (CR2, CR3), Nikon (NEF), Sony (ARW), Fujifilm (RAF), Olympus, Panasonic, Pentax, Leica, DNG, dan format RAW lain, plus JPEG.
- File pendamping ikut diurus otomatis: file `.xmp` dan JPEG pasangan RAW ikut pindah/tersalin bersama fotonya.

**Grid gaya Lightroom**
- Foto portrait maupun landscape tampil utuh, nggak terpotong.
- Ukuran thumbnail diatur dengan Ctrl + scroll atau slider.
- Urutkan berdasarkan nama file, waktu pengambilan, ukuran file, ISO, bukaan, focal length, shutter speed, atau status.
- Pilih banyak foto sekaligus (klik, Shift+klik, Ctrl+klik, Ctrl+A), lalu kirim ke Terpilih / Buangan / Masih mikir dengan satu tombol.
- Foto yang dipilih berlatar lebih terang, jadi jelas mana yang sedang dipilih.
- Panel metadata di sisi kanan.
- **Kembali ke antrian:** foto yang sudah dikategorikan bisa dikembalikan jadi belum diputuskan. Mode Salin: salinan di folder hasil dihapus. Mode Pindah: foto dipindah balik ke folder asalnya.

**Aman untuk foto aslimu**
- **Mode Salin:** foto asli nggak disentuh, hasil seleksi disalin ke folder tujuan.
- **Mode Pindah:** foto dipindah ke folder tujuan. Bisa diurungkan.
- Folder Buangan bisa dikosongkan ke Recycle Bin (bukan hapus permanen).
- Setiap project tersimpan otomatis, jadi bisa dilanjutkan kapan saja.

**Kontrol**
- Hotkey bisa diubah sesuai keinginan di Pengaturan > Hotkey.
- Mendukung controller Xbox / PlayStation / standar PC, jadi bisa menyeleksi sambil bersantai.
- Tema terang dan gelap otomatis mengikuti sistem.

**Performa**
- Atur pemakaian RAM, jumlah foto yang di-preload, dan drive tempat cache thumbnail disimpan.
- Tombol Buka folder cache dan Hapus cache (hanya thumbnail, foto aslimu aman).
- Proses pindah/salin foto yang lama punya layar loading dengan progress, jadi aplikasi nggak terlihat hang.

**Update langsung dari aplikasi**
- Ada versi baru? Muncul notifikasi, klik **Update sekarang**, selesai. Nggak perlu download dan instal ulang manual.

---

## Cara instal (pakai installer)

### Windows 10 / 11 (64-bit)
1. Download file installer terbaru, bentuknya `NESTHINK PHOTO SELECTOR_x.x.x_x64-setup.exe`, dari halaman
   **Releases**: https://github.com/YugraaCH/NESTHINK-PHOTO-SELECTOR/releases
2. Klik dua kali file installer, lalu ikuti petunjuknya sampai selesai.
3. Kalau muncul peringatan biru **"Windows protected your PC"** (SmartScreen), klik **More info** lalu **Run anyway**.
   Ini muncul karena aplikasi belum memakai sertifikat code signing berbayar, bukan karena ada masalah.
4. Buka **NESTHINK PHOTO SELECTOR** dari Start Menu atau shortcut di Desktop.

Catatan: aplikasi memakai Microsoft WebView2. Windows 11 sudah punya, dan installer akan memasangnya sendiri kalau
belum ada (butuh internet saat instal).

### macOS (12 atau lebih baru)
1. Download file `.dmg` dari halaman Releases yang sama.
2. Buka file `.dmg`, lalu seret aplikasinya ke folder **Applications**.
3. Kalau macOS memblokir saat pertama dibuka: klik kanan aplikasinya, pilih **Open**, lalu **Open** lagi.

### Update ke versi baru
Cukup buka aplikasinya. Kalau ada versi baru akan muncul notifikasi. Bisa juga cek manual lewat
**Pengaturan > Umum > Cek update**. Project dan pengaturanmu tetap aman.

---

## Cara pakai singkat
1. Di beranda, buat project baru, lalu pilih folder berisi foto kamu.
2. Pilih mode **Salin** (foto asli tetap aman) atau **Pindah**.
3. Seleksi satu per satu dengan hotkey. Tekan **G** untuk membuka Grid dan menyeleksi banyak foto sekaligus.
4. Selesai. Foto sudah rapi di folder **terpilih**, **buangan**, dan **masih mikir**.

Hotkey bawaan: panah kiri atau A = Buangan, panah kanan atau D = Terpilih, panah atas atau W = Masih mikir,
panah bawah atau S = Skip, Ctrl+Z = urungkan, Ctrl+Y = ulangi, G = Grid, I = detail info, Space = zoom.
Semuanya bisa diubah di Pengaturan > Hotkey.

**Controller:** hubungkan controller, tekan satu tombol supaya terdeteksi. Default: D-pad atau stick kiri =
Buangan / Terpilih / Masih mikir / Skip, LB/L1 = urungkan, RB/R1 = ulangi, Select = info, Start = grid, R3 = zoom.

---

## Dukung developer
Aplikasi ini gratis dan dibuat oleh **Nesthink Studio**. Kalau aplikasi ini membantu pekerjaanmu dan kamu ingin
berterima kasih, klik tombol **Support us** di dalam aplikasi, atau langsung ke https://saweria.co/Nesthinkstudio.
Dukunganmu membantu kami mengembangkan fitur-fitur baru.

Ada pertanyaan atau menemukan bug? Klik tombol **?** di pojok kanan bawah aplikasi untuk menghubungi developer lewat WhatsApp.

---

## Untuk developer

Tauri 2 (Rust) + UI HTML murni.

- `src-tauri/src/main.rs`: logika (scan, pindah/salin file, undo, preview RAW, EXIF, project.json)
- `ui/index.html`: seluruh tampilan dan hotkey
- `PANDUAN-AUTO-UPDATE.txt`: setup update otomatis dan cara merilis patch baru
- `npm run bump 0.5.1`: naikkan versi di semua file sekaligus

**Build di Windows:** install Rust (rustup.rs, dengan Visual Studio C++ Build Tools) dan Node.js LTS, lalu
`npm install` dan `npm run dev` (coba) atau `npm run build:win`. Hasil: `src-tauri/target/release/bundle/nsis/`.
Build lokal butuh kunci tanda tangan update, lihat bagian "Masalah umum" di PANDUAN-AUTO-UPDATE.txt.

**Build di macOS** (harus di Mac): `xcode-select --install`, install Rust dan Node.js, lalu `npm install` dan
`npm run build:mac` (atau `npm run build:mac:universal` untuk Intel + Apple Silicon, setelah
`rustup target add aarch64-apple-darwin x86_64-apple-darwin`). Kalau macOS memblokir hasil build:
`xattr -dr com.apple.quarantine "/Applications/NESTHINK PHOTO SELECTOR.app"`.

**Tanpa Mac:** push ke GitHub dan beri tag `vX.Y.Z`. GitHub Actions membangun Windows dan macOS otomatis,
lalu membuat Release berisi installer dan `latest.json`.
