local x = -(-foo)
local y = - -foo

local z1 = -(-foo) -- bar
local z2 = - -foo -- baz

-- Repeated unary minus that exceeds column width should not collapse into a comment (https://github.com/JohnnyMorganz/StyLua/issues/1075)
_ = (- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - 5)