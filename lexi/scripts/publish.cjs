const fs = require('fs');
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

  console.log(`🎉 Done!`);
} catch (err) {
  console.error('❌ Error during tagging or pushing:', err.message);
  process.exit(1);
}