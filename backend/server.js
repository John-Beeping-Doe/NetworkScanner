// backend/server.js

const express = require('express');
const { exec } = require('child_process');
const dns = require('dns');
const app = express();
const PORT = 3001;

// DNS Lookup
app.get('/api/dns', (req, res) => {
  const host = req.query.host;
  if (!host) return res.status(400).json({ error: 'Host required' });
  dns.lookup(host, (err, address, family) => {
    if (err) return res.status(500).json({ error: err.message });
    res.json({ host, address, family });
  });
});

// Ping
app.get('/api/ping', (req, res) => {
  const host = req.query.host;
  if (!host) return res.status(400).json({ error: 'Host required' });
  exec(`ping -c 4 ${host}`, (err, stdout, stderr) => {
    if (err) return res.status(500).json({ error: stderr.trim() });
    res.json({ host, result: stdout });
  });
});

// Traceroute
app.get('/api/traceroute', (req, res) => {
  const host = req.query.host;
  if (!host) return res.status(400).json({ error: 'Host required' });
  exec(`traceroute ${host}`, (err, stdout, stderr) => {
    if (err) return res.status(500).json({ error: stderr.trim() });
    res.json({ host, result: stdout });
  });
});

app.listen(PORT, () => {
  console.log(`Server running on http://localhost:${PORT}`);
});
