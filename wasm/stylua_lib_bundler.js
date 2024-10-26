import * as wasm from "./stylua.web/stylua_lib_bg.wasm";

import { __finalizeInit } from "./stylua.web/stylua_lib.js";

__finalizeInit({ exports: wasm });

export * from "./stylua.web/stylua_lib.js";
