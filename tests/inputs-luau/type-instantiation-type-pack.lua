-- https://github.com/JohnnyMorganz/StyLua/issues/1089
-- Single-element type packs in explicit type instantiation should preserve parentheses

local function f(...: T...) end

-- Single type pack: parentheses must be preserved (otherwise it becomes a type error)
f<<(number)>>(10)

-- Multi-element type packs already work, but test for completeness
f<<(string, number)>>(10, "a")

-- Method call variant
local t = {}
t:method<<(number)>>(10)
