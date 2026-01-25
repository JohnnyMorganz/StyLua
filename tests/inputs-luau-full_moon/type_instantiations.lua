--[[

Taken from https://github.com/luau-lang/luau/blob/535f92589bdc304c8a2012d6cfad5e7b9faff2f7/tests/conformance/explicit_type_instantiations.luau

MIT License

Copyright (c) 2019-2025 Roblox Corporation
Copyright (c) 1994–2019 Lua.org, PUC-Rio.

Permission is hereby granted, free of charge, to any person obtaining a copy of
this software and associated documentation files (the "Software"), to deal in
the Software without restriction, including without limitation the rights to
use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies
of the Software, and to permit persons to whom the Software is furnished to do
so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.

]]

-- Tests to ensure explicit type instantiations don't change runtime behavior
local function identity<T>(x: T): T
    return x
end

assert(identity<<number>>(1) == 1)

local function multipleReturns<T>(x: T): (T, T)
    return x, x
end

local a, b = multipleReturns<<number>>(1)
assert(a == 1 and b == 1)

local function typePacks<T...>(...: T...): T...
    return ...
end

local a, b = typePacks<<(string, number)>>(1, "a")
assert(a == 1 and b == "a")

local t = {}
function t:method<T>(x: T): T
    assert(self == t)
    return x
end

assert(t:method(1) == 1)

function t:methodTypePacks<T...>(...: T...): T...
    assert(self == t)
    return ...
end

local a, b = t:methodTypePacks<<(string, number)>>(1, "a")
assert(a == 1 and b == "a")

-- full-moon tests
local complicatedExpr = expr<<A<B<C>>>>()
local complicatedMethod = a:method<<A<B<C>>>>()

return "OK"