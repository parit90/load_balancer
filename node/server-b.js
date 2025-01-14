const express = require("express");
const cors = require("cors");

const app = express();
app.use(cors());

app.get("/apiB", (req, res) => {
      console.log("I am B")
  res.send("Hello World from API-B!");
});

const PORT = 3002;
app.listen(PORT, () => {
  console.log(`API-B Server running at http://localhost:${PORT}`);
});
