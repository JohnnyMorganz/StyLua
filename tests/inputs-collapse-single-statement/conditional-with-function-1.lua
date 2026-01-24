-- https://github.com/JohnnyMorganz/StyLua/issues/898

if bar then
	return function()
		foo()
	end
end

if bar then
	return Array.filter({}, function()
		return true
	end)
end
