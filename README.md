# Nesthink Photo Selector (v0.5.0)

**Cull hundreds of photos from a shoot without the headache. Free, offline, lightweight.**

After a shoot, it's common to come home with hundreds or even thousands of photos. To pick the keepers, we
usually open them one by one in Explorer or Lightroom, move files into folders by hand, and then lose track of
which ones we've already checked. Heavy RAW files make the whole process slow and exhausting.

Nesthink Photo Selector is built for exactly that: **culling photos fast.** One photo is shown large, you press
a single key (**Selected**, **Reject**, **Maybe**, or **Skip**), and the app takes care of moving the files. The
next photo is already preloaded, so the flow never waits.

---

## Features

**Fast culling**
- One photo per screen, decided with a single key: Selected, Reject, Maybe, or Skip.
- The next photos are preloaded in the background, so there's no delay when moving on.
- Unlimited Undo and Redo (Ctrl+Z / Ctrl+Y) until you close the project.
- Zoom straight to the cursor with the mouse wheel to check focus and sharpness.
- Photo details: file name, camera, lens, focal length, aperture, shutter speed, ISO.

**RAW support**
- Canon (CR2, CR3), Nikon (NEF), Sony (ARW), Fujifilm (RAF), Olympus, Panasonic, Pentax, Leica, DNG and other RAW formats, plus JPEG.
- Sidecar files are handled automatically: `.xmp` files and the JPEG paired with a RAW move or copy together with the photo.

**Lightroom-style grid**
- Portrait and landscape photos are shown in full, never cropped.
- Thumbnail size is adjusted with Ctrl + scroll or the slider.
- Sort by file name, capture time, file size, ISO, aperture, focal length, shutter speed, or status.
- Select many photos at once (click, Shift+click, Ctrl+click, Ctrl+A), then send them to Selected / Reject / Maybe with one key.
- Selected photos get a lighter background, so it's always clear what is selected.
- Metadata panel on the right.
- **Return to queue:** a photo you already sorted can go back to "undecided". Copy mode: the copy in the result folder is deleted. Move mode: the photo is moved back to its original folder.

**Safe for your originals**
- **Copy mode:** your originals are never touched; selected photos are copied to the destination folder.
- **Move mode:** photos are moved to the destination folder. Can be undone.
- The Reject folder can be emptied to the Recycle Bin (not permanently deleted).
- Every project is saved automatically, so you can continue any time.

**Controls**
- Hotkeys can be changed in Settings > Hotkey.
- Supports Xbox / PlayStation / standard PC controllers, so you can cull from the couch.
- Light and dark theme follow your system automatically.

**Performance**
- Set the RAM usage, how many photos are preloaded, and which drive stores the thumbnail cache.
- Buttons to open the cache folder and clear the cache (thumbnails only, your photos are safe).
- Long copy/move operations show a loading screen with progress, so the app never looks frozen.

**Update from inside the app**
- New version available? You get a notification, click **Update now**, done. No manual download or reinstall.

---

## Installation (using the installer)

### Windows 10 / 11 (64-bit)
1. Download the latest installer, named like `NESTHINK PHOTO SELECTOR_x.x.x_x64-setup.exe`, from the
   **Releases** page: https://github.com/YugraaCH/PHOTO-SELECTOR-by-Nesthink-Studio/releases
2. Double-click the installer and follow the steps until it finishes.
3. If a blue **"Windows protected your PC"** warning (SmartScreen) appears, click **More info**, then **Run anyway**.
   It shows up because the app doesn't use a paid code signing certificate yet, not because anything is wrong.
4. Open **NESTHINK PHOTO SELECTOR** from the Start Menu or the Desktop shortcut.

Note: the app uses Microsoft WebView2. Windows 11 already includes it, and the installer will install it
automatically if it's missing (internet required during installation).

### macOS (12 or later)
1. Download the `.dmg` file from the same Releases page.
2. Open the `.dmg` and drag the app into the **Applications** folder.
3. If macOS blocks it on first launch: right-click the app, choose **Open**, then **Open** again.

### Updating to a new version
Just open the app. If a new version is available, a notification appears. You can also check manually in
**Settings > General > Check for updates**. Your projects and settings stay safe.

---

## Quick start
1. On the home screen, create a new project and pick the folder that contains your photos.
2. Choose **Copy** mode (originals stay safe) or **Move** mode.
3. Cull one by one with the hotkeys. Press **G** to open the Grid and sort many photos at once.
4. Done. Your photos are neatly organized into the **selected**, **reject**, and **maybe** folders.

Default hotkeys: Left arrow or A = Reject, Right arrow or D = Selected, Up arrow or W = Maybe, Down arrow or S = Skip,
Ctrl+Z = undo, Ctrl+Y = redo, G = Grid, I = photo info, Space = zoom. Everything can be changed in Settings > Hotkey.

**Controller:** connect your controller and press any button so it gets detected. Defaults: D-pad or left stick =
Reject / Selected / Maybe / Skip, LB/L1 = undo, RB/R1 = redo, Select = info, Start = grid, R3 = zoom.

---

## Support the developer
This app is free and made by **Nesthink Studio**. If it helps your work and you'd like to say thanks, click the
**Support us** button inside the app, or go straight to https://saweria.co/Nesthinkstudio. Your support helps us
build new features.

Questions or found a bug? Click the **?** button in the bottom-right corner of the app to reach the developer on
WhatsApp, or open an issue on GitHub.

---

## For developers

Tauri 2 (Rust) + plain HTML UI.

- `src-tauri/src/main.rs`: logic (scan, move/copy files, undo, RAW preview, EXIF, project.json)
- `ui/index.html`: the entire UI and hotkeys
- `PANDUAN-AUTO-UPDATE.txt`: auto-update setup and how to ship new patches (written in Indonesian)
- `npm run bump 0.5.1`: bump the version in all files at once

**Build on Windows:** install Rust (rustup.rs, with Visual Studio C++ Build Tools) and Node.js LTS, then run
`npm install` and `npm run dev` (to try it) or `npm run build:win`. Output: `src-tauri/target/release/bundle/nsis/`.
Local builds need the update signing key; see the troubleshooting section in PANDUAN-AUTO-UPDATE.txt.

**Build on macOS** (must be built on a Mac): `xcode-select --install`, install Rust and Node.js, then `npm install`
and `npm run build:mac` (or `npm run build:mac:universal` for Intel + Apple Silicon, after
`rustup target add aarch64-apple-darwin x86_64-apple-darwin`). If macOS blocks the build:
`xattr -dr com.apple.quarantine "/Applications/NESTHINK PHOTO SELECTOR.app"`.

**No Mac?** Push to GitHub and add a tag `vX.Y.Z`. GitHub Actions builds Windows and macOS automatically and
creates a Release containing the installers and `latest.json`.
