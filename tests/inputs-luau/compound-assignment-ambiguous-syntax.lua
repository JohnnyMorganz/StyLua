-- https://github.com/JohnnyMorganz/StyLua/issues/885

local function foo()
    return { b = "foo" }
end

local a = foo();
(a :: any).b ..= "bar"