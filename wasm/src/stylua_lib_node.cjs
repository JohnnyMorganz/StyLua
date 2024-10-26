export * from "../stylua.web/stylua_lib.js";
import { initSync } from "../stylua.web/stylua_lib.js";

const path = require("path").join(__dirname, "stylua.web/stylua_lib_bg.wasm");
const bytes = require("fs").readFileSync(path);

initSync(bytes);
