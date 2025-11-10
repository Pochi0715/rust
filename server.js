const http = require('http');
const { spawn } = require('child_process');

const port = process.env.PORT || 8080;

// ヘルスチェック用HTTPサーバー
const server = http.createServer((req, res) => {
  res.statusCode = 200;
  res.setHeader('Content-Type', 'text/plain');
  res.end('Discord Bot is running in background.');
});

server.listen(port, () => {
  console.log(`Render Health Check Server listening on port ${port}`);
});

// Rust Discord botを起動
const bot = spawn('./target/release/qiita_poise');

bot.stdout.on('data', (data) => {
  console.log(`[Bot] ${data}`);
});

bot.stderr.on('data', (data) => {
  console.error(`[Bot Error] ${data}`);
});

bot.on('close', (code) => {
  console.log(`Bot process exited with code ${code}`);
  process.exit(code);
});
