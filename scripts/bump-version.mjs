// Naikkan versi aplikasi di semua file sekaligus.
// Pakai:  npm run bump 0.5.1
import fs from 'node:fs';

const v = process.argv[2];
if (!/^\d+\.\d+\.\d+$/.test(v || '')) {
  console.error('Format versi salah. Contoh pemakaian:  npm run bump 0.5.1');
  process.exit(1);
}
const edit = (file, fn) => {
  const t = fs.readFileSync(file, 'utf8');
  const n = fn(t);
  if (n === t) console.warn('  (tidak ada perubahan) ' + file);
  else console.log('  ok  ' + file);
  fs.writeFileSync(file, n);
};
const json = t => t.replace(/"version": "[^"]+"/, `"version": "${v}"`);
console.log('Versi baru: ' + v);
edit('package.json', json);
edit('src-tauri/tauri.conf.json', json);
edit('src-tauri/Cargo.toml', t => t.replace(/^version = ".*"$/m, `version = "${v}"`));
edit('README.md', t => t.replace(/\(v\d+\.\d+\.\d+\)/, `(v${v})`));
console.log('\nSelanjutnya:  git add -A && git commit -m "v' + v + '" && git tag v' + v + ' && git push && git push --tags');
