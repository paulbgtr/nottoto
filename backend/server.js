import express from "express";
import router from "./router.js";
const app = express();
const port = 3000;

app.use(express.json());

app.use(router);

app.get("/", (req, res) => {
  res.send("Hello World!")
});

app.listen(port, () => {
  console.log(`Example app listening at http://localhost:${port}`)
});
