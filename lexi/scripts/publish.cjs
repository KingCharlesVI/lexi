const fs = require('fs');
const path = require('path');
const { execSync } = require('child_process');

const config = JSON.parse(fs.readFileSync('src-tauri/tauri.conf.json', 'utf8'));
const version = config.version;
const tag = `v${version}`;

console.log('🔍 Checking version from tauri.conf.json...');
if (!version) {
  console.error('❌ No version found in tauri.conf.json');
  process.exit(1);
}
console.log(`✅ Found version: ${version}`);

console.log(`🔖 Creating Git tag: ${tag}`);
try {
  execSync(`git tag ${tag}`, { stdio: 'inherit' });
  console.log(`✅ Tag created`);

  console.log(`🚀 Pushing tag to GitHub...`);
  execSync(`git push origin ${tag}`, { stdio: 'inherit' });

  console.log(`🔧 Running Tauri build...`);
  execSync(`npm run tauri build`, { stdio: 'inherit' });

  const bundleDir = path.join(__dirname, 'src-tauri', 'target', 'release', 'bundle');
  const foundFiles = [];

  const searchBundleFiles = (dir) => {
    if (fs.existsSync(dir)) {
      const files = fs.readdirSync(dir);
      for (const file of files) {
        const filePath = path.join(dir, file);
        if (fs.statSync(filePath).isFile() && (file.endsWith('.msi') || file.endsWith('.exe'))) {
          foundFiles.push(filePath);
        }
      }
    }
  };

  // Look inside bundle root and subfolders
  searchBundleFiles(bundleDir);
  ['msi', 'nsis', 'appimage', 'dmg', 'deb'].forEach((sub) =>
    searchBundleFiles(path.join(bundleDir, sub))
  );

  if (foundFiles.length > 0) {
    console.log('📦 Built installer(s):');
    foundFiles.forEach((f) => console.log(`  → ${f}`));
  } else {
    console.warn('⚠️ No installer files found. Check the /bundle directory manually.');
  }

  console.log(`🎉 Done!`);
} catch (err) {
  console.error('❌ Error:', err.message);
  process.exit(1);
}