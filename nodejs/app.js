const express = require("express");
const app = express();

app.get("/", async (_, res) => {
  res.status(200).send("Hello world from NodeJS!");
});

const port = process.env.PORT || 8080;
app.listen(port);
