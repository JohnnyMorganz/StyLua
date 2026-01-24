-- https://github.com/JohnnyMorganz/StyLua/issues/996

local function f()
  local a = "ab"
  a = a:gsub("a".. -- "
    'b', function() return "c" end)
  print(a)
end

f()
