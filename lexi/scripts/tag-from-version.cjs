const fs = require('fs');
const config = JSON.parse(fs.readFileSync('src-tauri/tauri.conf.json', 'utf8'));
const version = config.version;

console.log(`Creating git tag v${version}...`);
require('child_process').execSync(`git tag v${version} && git push origin v${version}`, { stdio: 'inherit' });