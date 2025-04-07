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

  // Determine expected setup file path
  const setupPath = path.join(__dirname, 'src-tauri', 'target', 'release', 'bundle', 'msi', `${config.package.productName}_${version}_x64_en-US.msi`);

  if (fs.existsSync(setupPath)) {
    console.log(`📦 Setup binary created at: ${setupPath}`);
  } else {
    console.warn('⚠️ Could not find setup binary automatically. Please check the /src-tauri/target/release/bundle directory.');
  }

  console.log(`🎉 Done!`);
} catch (err) {
  console.error('❌ Error:', err.message);
  process.exit(1);
}