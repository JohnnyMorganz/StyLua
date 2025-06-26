import { readFileSync } from 'node:fs';
import { dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';

let wasm;

const cachedTextDecoder = (typeof TextDecoder !== 'undefined' ? new TextDecoder('utf-8', { ignoreBOM: true, fatal: true }) : { decode: () => { throw Error('TextDecoder not available') } } );

if (typeof TextDecoder !== 'undefined') { cachedTextDecoder.decode(); }
let cachedUint8Memory0 = null;

function getUint8Memory0() {
    if (cachedUint8Memory0 === null || cachedUint8Memory0.byteLength === 0) {
        cachedUint8Memory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8Memory0;
}

function getStringFromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));
}

const heap = new Array(128).fill(undefined);

heap.push(undefined, null, true, false);

let heap_next = heap.length;

function addHeapObject(obj) {
    if (heap_next === heap.length) heap.push(heap.length + 1);
    const idx = heap_next;
    heap_next = heap[idx];

    heap[idx] = obj;
    return idx;
}

let cachedInt32Memory0 = null;

function getInt32Memory0() {
    if (cachedInt32Memory0 === null || cachedInt32Memory0.byteLength === 0) {
        cachedInt32Memory0 = new Int32Array(wasm.memory.buffer);
    }
    return cachedInt32Memory0;
}

function isLikeNone(x) {
    return x === undefined || x === null;
}

function _assertClass(instance, klass) {
    if (!(instance instanceof klass)) {
        throw new Error(`expected instance of ${klass.name}`);
    }
    return instance.ptr;
}

let WASM_VECTOR_LEN = 0;

const cachedTextEncoder = (typeof TextEncoder !== 'undefined' ? new TextEncoder('utf-8') : { encode: () => { throw Error('TextEncoder not available') } } );

const encodeString = (typeof cachedTextEncoder.encodeInto === 'function'
    ? function (arg, view) {
    return cachedTextEncoder.encodeInto(arg, view);
}
    : function (arg, view) {
    const buf = cachedTextEncoder.encode(arg);
    view.set(buf);
    return {
        read: arg.length,
        written: buf.length
    };
});

function passStringToWasm0(arg, malloc, realloc) {

    if (realloc === undefined) {
        const buf = cachedTextEncoder.encode(arg);
        const ptr = malloc(buf.length, 1) >>> 0;
        getUint8Memory0().subarray(ptr, ptr + buf.length).set(buf);
        WASM_VECTOR_LEN = buf.length;
        return ptr;
    }

    let len = arg.length;
    let ptr = malloc(len, 1) >>> 0;

    const mem = getUint8Memory0();

    let offset = 0;

    for (; offset < len; offset++) {
        const code = arg.charCodeAt(offset);
        if (code > 0x7F) break;
        mem[ptr + offset] = code;
    }

    if (offset !== len) {
        if (offset !== 0) {
            arg = arg.slice(offset);
        }
        ptr = realloc(ptr, len, len = offset + arg.length * 3, 1) >>> 0;
        const view = getUint8Memory0().subarray(ptr + offset, ptr + len);
        const ret = encodeString(arg, view);

        offset += ret.written;
    }

    WASM_VECTOR_LEN = offset;
    return ptr;
}

function getObject(idx) { return heap[idx]; }

function dropObject(idx) {
    if (idx < 132) return;
    heap[idx] = heap_next;
    heap_next = idx;
}

function takeObject(idx) {
    const ret = getObject(idx);
    dropObject(idx);
    return ret;
}
/**
* @param {string} code
* @param {Config} config
* @param {Range | undefined} range
* @param {OutputVerification} verify_output
* @returns {string}
*/
function formatCode(code, config, range, verify_output) {
    let deferred5_0;
    let deferred5_1;
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        const ptr0 = passStringToWasm0(code, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        _assertClass(config, Config);
        var ptr1 = config.__destroy_into_raw();
        let ptr2 = 0;
        if (!isLikeNone(range)) {
            _assertClass(range, Range);
            ptr2 = range.__destroy_into_raw();
        }
        wasm.formatCode(retptr, ptr0, len0, ptr1, ptr2, verify_output);
        var r0 = getInt32Memory0()[retptr / 4 + 0];
        var r1 = getInt32Memory0()[retptr / 4 + 1];
        var r2 = getInt32Memory0()[retptr / 4 + 2];
        var r3 = getInt32Memory0()[retptr / 4 + 3];
        var ptr4 = r0;
        var len4 = r1;
        if (r3) {
            ptr4 = 0; len4 = 0;
            throw takeObject(r2);
        }
        deferred5_0 = ptr4;
        deferred5_1 = len4;
        return getStringFromWasm0(ptr4, len4);
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
        wasm.__wbindgen_free(deferred5_0, deferred5_1, 1);
    }
}

/**
* What mode to use if we want to collapse simple functions / guard statements
*/
const CollapseSimpleStatement = Object.freeze({
/**
* Never collapse
*/
Never:0,"0":"Never",
/**
* Collapse simple functions onto a single line
*/
FunctionOnly:1,"1":"FunctionOnly",
/**
* Collapse simple if guards onto a single line
*/
ConditionalOnly:2,"2":"ConditionalOnly",
/**
* Collapse all simple statements onto a single line
*/
Always:3,"3":"Always", });
/**
* When to use call parentheses
*/
const CallParenType = Object.freeze({
/**
* Use call parentheses all the time
*/
Always:0,"0":"Always",
/**
* Skip call parentheses when only a string argument is used.
*/
NoSingleString:1,"1":"NoSingleString",
/**
* Skip call parentheses when only a table argument is used.
*/
NoSingleTable:2,"2":"NoSingleTable",
/**
* Skip call parentheses when only a table or string argument is used.
*/
None:3,"3":"None",
/**
* Keep call parentheses based on its presence in input code.
*/
Input:4,"4":"Input", });
/**
* When to use spaces after function names
*/
const SpaceAfterFunctionNames = Object.freeze({
/**
* Never use spaces after function names.
*/
Never:0,"0":"Never",
/**
* Use spaces after function names only for function definitions.
*/
Definitions:1,"1":"Definitions",
/**
* Use spaces after function names only for function calls.
*/
Calls:2,"2":"Calls",
/**
* Use spaces after function names in definitions and calls.
*/
Always:3,"3":"Always", });
/**
* The style of quotes to use within string literals
*/
const QuoteStyle = Object.freeze({
/**
* Use double quotes where possible, but change to single quotes if it produces less escapes
*/
AutoPreferDouble:0,"0":"AutoPreferDouble",
/**
* Use single quotes where possible, but change to double quotes if it produces less escapes
*/
AutoPreferSingle:1,"1":"AutoPreferSingle",
/**
* Always use double quotes in all strings
*/
ForceDouble:2,"2":"ForceDouble",
/**
* Always use single quotes in all strings
*/
ForceSingle:3,"3":"ForceSingle", });
/**
* The Lua syntax version to use
*/
const LuaVersion = Object.freeze({
/**
* Parse all syntax versions at the same time. This allows most general usage.
* For overlapping syntaxes (e.g., Lua5.2 label syntax and Luau type assertions), select a
* specific syntax version
*/
All:0,"0":"All",
/**
* Parse Lua 5.1 code
*/
Lua51:1,"1":"Lua51",
/**
* Parse Lua 5.2 code
*/
Lua52:2,"2":"Lua52",
/**
* Parse Lua 5.3 code
*/
Lua53:3,"3":"Lua53",
/**
* Parse Lua 5.4 code
*/
Lua54:4,"4":"Lua54",
/**
* Parse Luau code
*/
Luau:5,"5":"Luau",
/**
* Parse LuaJIT code
*/
LuaJIT:6,"6":"LuaJIT",
/**
* Parse Cfx Lua code
*/
CfxLua:7,"7":"CfxLua", });
/**
* The type of line endings to use at the end of a line
*/
const LineEndings = Object.freeze({
/**
* Unix Line Endings (LF) - `\n`
*/
Unix:0,"0":"Unix",
/**
* Windows Line Endings (CRLF) - `\r\n`
*/
Windows:1,"1":"Windows", });
/**
* The type of indents to use when indenting
*/
const IndentType = Object.freeze({
/**
* Indent using tabs (`\t`)
*/
Tabs:0,"0":"Tabs",
/**
* Indent using spaces (` `)
*/
Spaces:1,"1":"Spaces", });
/**
* The type of verification to perform to validate that the output AST is still correct.
*/
const OutputVerification = Object.freeze({
/**
* Reparse the generated output to detect any changes to code correctness.
*/
Full:0,"0":"Full",
/**
* Perform no verification of the output.
*/
None:1,"1":"None", });
/**
* The configuration to use when formatting.
*/
class Config {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(Config.prototype);
        obj.__wbg_ptr = ptr;

        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_config_free(ptr);
    }
    /**
    * The type of Lua syntax to parse.
    * @returns {LuaVersion}
    */
    get syntax() {
        const ret = wasm.__wbg_get_config_syntax(this.__wbg_ptr);
        return ret;
    }
    /**
    * The type of Lua syntax to parse.
    * @param {LuaVersion} arg0
    */
    set syntax(arg0) {
        wasm.__wbg_set_config_syntax(this.__wbg_ptr, arg0);
    }
    /**
    * The approximate line length to use when printing the code.
    * This is used as a guide to determine when to wrap lines, but note
    * that this is not a hard upper bound.
    * @returns {number}
    */
    get column_width() {
        const ret = wasm.__wbg_get_config_column_width(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
    * The approximate line length to use when printing the code.
    * This is used as a guide to determine when to wrap lines, but note
    * that this is not a hard upper bound.
    * @param {number} arg0
    */
    set column_width(arg0) {
        wasm.__wbg_set_config_column_width(this.__wbg_ptr, arg0);
    }
    /**
    * The type of line endings to use.
    * @returns {LineEndings}
    */
    get line_endings() {
        const ret = wasm.__wbg_get_config_line_endings(this.__wbg_ptr);
        return ret;
    }
    /**
    * The type of line endings to use.
    * @param {LineEndings} arg0
    */
    set line_endings(arg0) {
        wasm.__wbg_set_config_line_endings(this.__wbg_ptr, arg0);
    }
    /**
    * The type of indents to use.
    * @returns {IndentType}
    */
    get indent_type() {
        const ret = wasm.__wbg_get_config_indent_type(this.__wbg_ptr);
        return ret;
    }
    /**
    * The type of indents to use.
    * @param {IndentType} arg0
    */
    set indent_type(arg0) {
        wasm.__wbg_set_config_indent_type(this.__wbg_ptr, arg0);
    }
    /**
    * The width of a single indentation level.
    * If `indent_type` is set to [`IndentType::Spaces`], then this is the number of spaces to use.
    * If `indent_type` is set to [`IndentType::Tabs`], then this is used as a heuristic to guide when to wrap lines.
    * @returns {number}
    */
    get indent_width() {
        const ret = wasm.__wbg_get_config_indent_width(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
    * The width of a single indentation level.
    * If `indent_type` is set to [`IndentType::Spaces`], then this is the number of spaces to use.
    * If `indent_type` is set to [`IndentType::Tabs`], then this is used as a heuristic to guide when to wrap lines.
    * @param {number} arg0
    */
    set indent_width(arg0) {
        wasm.__wbg_set_config_indent_width(this.__wbg_ptr, arg0);
    }
    /**
    * The style of quotes to use in string literals.
    * @returns {QuoteStyle}
    */
    get quote_style() {
        const ret = wasm.__wbg_get_config_quote_style(this.__wbg_ptr);
        return ret;
    }
    /**
    * The style of quotes to use in string literals.
    * @param {QuoteStyle} arg0
    */
    set quote_style(arg0) {
        wasm.__wbg_set_config_quote_style(this.__wbg_ptr, arg0);
    }
    /**
    * Whether to omit parentheses around function calls which take a single string literal or table.
    * This is added for adoption reasons only, and is not recommended for new work.
    * @returns {boolean}
    */
    get no_call_parentheses() {
        const ret = wasm.__wbg_get_config_no_call_parentheses(this.__wbg_ptr);
        return ret !== 0;
    }
    /**
    * Whether to omit parentheses around function calls which take a single string literal or table.
    * This is added for adoption reasons only, and is not recommended for new work.
    * @param {boolean} arg0
    */
    set no_call_parentheses(arg0) {
        wasm.__wbg_set_config_no_call_parentheses(this.__wbg_ptr, arg0);
    }
    /**
    * When to use call parentheses.
    * if call_parentheses is set to [`CallParenType::Always`] call parentheses is always applied.
    * if call_parentheses is set to [`CallParenType::NoSingleTable`] call parentheses is omitted when
    * function is called with only one string argument.
    * if call_parentheses is set to [`CallParenType::NoSingleTable`] call parentheses is omitted when
    * function is called with only one table argument.
    * if call_parentheses is set to [`CallParenType::None`] call parentheses is omitted when
    * function is called with only one table or string argument (same as no_call_parentheses).
    * @returns {CallParenType}
    */
    get call_parentheses() {
        const ret = wasm.__wbg_get_config_call_parentheses(this.__wbg_ptr);
        return ret;
    }
    /**
    * When to use call parentheses.
    * if call_parentheses is set to [`CallParenType::Always`] call parentheses is always applied.
    * if call_parentheses is set to [`CallParenType::NoSingleTable`] call parentheses is omitted when
    * function is called with only one string argument.
    * if call_parentheses is set to [`CallParenType::NoSingleTable`] call parentheses is omitted when
    * function is called with only one table argument.
    * if call_parentheses is set to [`CallParenType::None`] call parentheses is omitted when
    * function is called with only one table or string argument (same as no_call_parentheses).
    * @param {CallParenType} arg0
    */
    set call_parentheses(arg0) {
        wasm.__wbg_set_config_call_parentheses(this.__wbg_ptr, arg0);
    }
    /**
    * Whether we should collapse simple structures like functions or guard statements
    * if set to [`CollapseSimpleStatement::None`] structures are never collapsed.
    * if set to [`CollapseSimpleStatement::FunctionOnly`] then simple functions (i.e., functions with a single laststmt) can be collapsed
    * @returns {CollapseSimpleStatement}
    */
    get collapse_simple_statement() {
        const ret = wasm.__wbg_get_config_collapse_simple_statement(this.__wbg_ptr);
        return ret;
    }
    /**
    * Whether we should collapse simple structures like functions or guard statements
    * if set to [`CollapseSimpleStatement::None`] structures are never collapsed.
    * if set to [`CollapseSimpleStatement::FunctionOnly`] then simple functions (i.e., functions with a single laststmt) can be collapsed
    * @param {CollapseSimpleStatement} arg0
    */
    set collapse_simple_statement(arg0) {
        wasm.__wbg_set_config_collapse_simple_statement(this.__wbg_ptr, arg0);
    }
    /**
    * Configuration for the sort requires codemod
    * @returns {SortRequiresConfig}
    */
    get sort_requires() {
        const ret = wasm.__wbg_get_config_sort_requires(this.__wbg_ptr);
        return SortRequiresConfig.__wrap(ret);
    }
    /**
    * Configuration for the sort requires codemod
    * @param {SortRequiresConfig} arg0
    */
    set sort_requires(arg0) {
        _assertClass(arg0, SortRequiresConfig);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_config_sort_requires(this.__wbg_ptr, ptr0);
    }
    /**
    * Whether we should include a space between the function name and arguments.
    * * if space_after_function_names is set to [`SpaceAfterFunctionNames::Never`] a space is never used.
    * * if space_after_function_names is set to [`SpaceAfterFunctionNames::Definitions`] a space is used only for definitions.
    * * if space_after_function_names is set to [`SpaceAfterFunctionNames::Calls`] a space is used only for calls.
    * * if space_after_function_names is set to [`SpaceAfterFunctionNames::Always`] a space is used for both definitions and calls.
    * @returns {SpaceAfterFunctionNames}
    */
    get space_after_function_names() {
        const ret = wasm.__wbg_get_config_space_after_function_names(this.__wbg_ptr);
        return ret;
    }
    /**
    * Whether we should include a space between the function name and arguments.
    * * if space_after_function_names is set to [`SpaceAfterFunctionNames::Never`] a space is never used.
    * * if space_after_function_names is set to [`SpaceAfterFunctionNames::Definitions`] a space is used only for definitions.
    * * if space_after_function_names is set to [`SpaceAfterFunctionNames::Calls`] a space is used only for calls.
    * * if space_after_function_names is set to [`SpaceAfterFunctionNames::Always`] a space is used for both definitions and calls.
    * @param {SpaceAfterFunctionNames} arg0
    */
    set space_after_function_names(arg0) {
        wasm.__wbg_set_config_space_after_function_names(this.__wbg_ptr, arg0);
    }
    /**
    * Creates a new Config with the default values
    * @returns {Config}
    */
    static new() {
        const ret = wasm.config_new();
        return Config.__wrap(ret);
    }
}
/**
* An optional formatting range.
* If provided, only content within these boundaries (inclusive) will be formatted.
* Both boundaries are optional, and are given as byte offsets from the beginning of the file.
*/
class Range {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(Range.prototype);
        obj.__wbg_ptr = ptr;

        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_range_free(ptr);
    }
    /**
    * @returns {number | undefined}
    */
    get start() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.__wbg_get_range_start(retptr, this.__wbg_ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            return r0 === 0 ? undefined : r1 >>> 0;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * @param {number | undefined} [arg0]
    */
    set start(arg0) {
        wasm.__wbg_set_range_start(this.__wbg_ptr, !isLikeNone(arg0), isLikeNone(arg0) ? 0 : arg0);
    }
    /**
    * @returns {number | undefined}
    */
    get end() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.__wbg_get_range_end(retptr, this.__wbg_ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            return r0 === 0 ? undefined : r1 >>> 0;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * @param {number | undefined} [arg0]
    */
    set end(arg0) {
        wasm.__wbg_set_range_end(this.__wbg_ptr, !isLikeNone(arg0), isLikeNone(arg0) ? 0 : arg0);
    }
    /**
    * Creates a new formatting range from the given start and end point.
    * All content within these boundaries (inclusive) will be formatted.
    * @param {number | undefined} [start]
    * @param {number | undefined} [end]
    * @returns {Range}
    */
    static from_values(start, end) {
        const ret = wasm.range_from_values(!isLikeNone(start), isLikeNone(start) ? 0 : start, !isLikeNone(end), isLikeNone(end) ? 0 : end);
        return Range.__wrap(ret);
    }
}
/**
* Configuration for the Sort Requires codemod
*/
class SortRequiresConfig {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(SortRequiresConfig.prototype);
        obj.__wbg_ptr = ptr;

        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_sortrequiresconfig_free(ptr);
    }
    /**
    * Whether the sort requires codemod is enabled
    * @returns {boolean}
    */
    get enabled() {
        const ret = wasm.__wbg_get_sortrequiresconfig_enabled(this.__wbg_ptr);
        return ret !== 0;
    }
    /**
    * Whether the sort requires codemod is enabled
    * @param {boolean} arg0
    */
    set enabled(arg0) {
        wasm.__wbg_set_sortrequiresconfig_enabled(this.__wbg_ptr, arg0);
    }
    /**
    * @returns {SortRequiresConfig}
    */
    static new() {
        const ret = wasm.sortrequiresconfig_new();
        return SortRequiresConfig.__wrap(ret);
    }
    /**
    * @param {boolean} enabled
    * @returns {SortRequiresConfig}
    */
    set_enabled(enabled) {
        const ret = wasm.sortrequiresconfig_set_enabled(this.__wbg_ptr, enabled);
        return SortRequiresConfig.__wrap(ret);
    }
}

function __wbg_get_imports() {
    const imports = {};
    imports.wbg = {};
    imports.wbg.__wbindgen_string_new = function(arg0, arg1) {
        const ret = getStringFromWasm0(arg0, arg1);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_throw = function(arg0, arg1) {
        throw new Error(getStringFromWasm0(arg0, arg1));
    };

    return imports;
}

function __wbg_finalize_init(instance, module) {
    wasm = instance.exports;
    cachedInt32Memory0 = null;
    cachedUint8Memory0 = null;


    return wasm;
}

function initSync(module) {
    if (wasm !== undefined) return wasm;

    const imports = __wbg_get_imports();

    if (!(module instanceof WebAssembly.Module)) {
        module = new WebAssembly.Module(module);
    }

    const instance = new WebAssembly.Instance(module, imports);

    return __wbg_finalize_init(instance);
}

// Get current file location for relative path resolution
const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

// Load and initialize the WASM module
const wasmPath = join(__dirname, 'stylua.web', 'stylua_lib_bg.wasm');
const wasmBytes = readFileSync(wasmPath);
initSync(wasmBytes);

export { CallParenType, CollapseSimpleStatement, Config, IndentType, LineEndings, LuaVersion, OutputVerification, QuoteStyle, Range, SortRequiresConfig, SpaceAfterFunctionNames, __wbg_finalize_init as __finalizeInit, __wbg_get_imports as __getImports, formatCode, initSync };
