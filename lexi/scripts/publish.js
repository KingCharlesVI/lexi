const fs = require('fs');
const { execSync } = require('child_process');

const configPath = 'src-tauri/tauri.conf.json';
const config = JSON.parse(fs.readFileSync(configPath, 'utf8'));

const version = config.version;
if (!version) {
  console.error('❌ No version found in tauri.conf.json');
  process.exit(1);
}

const tag = `v${version}`;

try {
  console.log(`🔖 Tagging commit as ${tag}...`);
  execSync(`git tag ${tag}`, { stdio: 'inherit' });
  execSync(`git push origin ${tag}`, { stdio: 'inherit' });
  console.log(`✅ Tag pushed. GitHub Actions will now build and release your app.`);
} catch (err) {
  console.error('❌ Failed to create or push git tag:', err);
  process.exit(1);
}