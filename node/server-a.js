const express = require("express");
const cors = require("cors");

const app = express();
app.use(cors());

app.get("/apiA", (req, res) => {
  console.log("I am A")
  res.send("Hello World from API-A!");
});

const PORT = 3001;
app.listen(PORT, () => {
  console.log(`API-A Server running at http://localhost:${PORT}`);
});
