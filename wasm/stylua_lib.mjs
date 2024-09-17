import fs from "node:fs";
import { initSync } from "./stylua.web/stylua_lib.js";

const wasm = new URL("./stylua.web/stylua_lib_bg.wasm", import.meta.url);
initSync(fs.readFileSync(wasm));

export * from "./stylua.web/stylua_lib.js";
