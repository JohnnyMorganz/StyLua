local x = function()
	return (function()
		local z = 3 + 4
		return complexCall(z)
	end)
end

local x = function()
	return (function()
		local z = 3 + 4
		return complexCall(z)
	end)()
end

local x = function()
	return not (function()
		local z = 3 + 4
		return complexCall(z)
	end)()
end

local x = function()
	return { function() return true end }
end

local x = function()
	return { [(function() return false end)()] = true }
end
