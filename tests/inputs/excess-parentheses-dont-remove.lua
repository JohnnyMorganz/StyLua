-- https://github.com/JohnnyMorganz/StyLua/issues/609
-- Indicate precedence
local _ = (not true) == true
local _ = (not true) and false

-- https://github.com/JohnnyMorganz/StyLua/issues/623
-- Changes meaning
local y = (-X) ^ Y
