-- https://github.com/JohnnyMorganz/StyLua/issues/1089

local function f(...: T...) end

f<<(number)>>(10)
f<<(string, number)>>(10, "a")

local t = {}
t:method<<(number)>>(10)
