-- https://github.com/JohnnyMorganz/StyLua/issues/963
local value = nil

(Foo):Call()

local tbl = {}

(Foo):Call()

local x = 1

(Foo):Call()

local x = "value"

(Foo):Call()

local x = function() end

(Foo):Call()
