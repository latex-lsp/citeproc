const cp = require('child_process');

cp.execSync('npm install && npm run dist', { cwd: __dirname, stdio: 'inherit'});
