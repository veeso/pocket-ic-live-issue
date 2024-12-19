const express = require("express");
const app = express();

// Middleware to parse JSON request bodies
app.use(express.json());

app.all("*", (req, res) => {
  console.log(req.method, req.body);

  res.json(req.body);
});

// Start the server
const PORT = 3000;
app.listen(PORT, () => {
  console.log(`Echo server running on http://localhost:${PORT}`);
});
