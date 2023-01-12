-- https://github.com/JohnnyMorganz/StyLua/issues/627
t = t or function()
	print("Hello, World") -- comment
end

t = t or function()
	print("Hello, World")
end
