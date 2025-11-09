const http = require('http');
const port = process.env.PORT || 8080; 

const server = http.createServer((req, res) => {
  res.statusCode = 200;
  res.setHeader('Content-Type', 'text/plain');
  res.end('Discord Bot is running in background.');
});

server.listen(port, () => {
  console.log(`Render Health Check Server listening on port ${port}`);
});
