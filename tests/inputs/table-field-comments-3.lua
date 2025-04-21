-- https://github.com/JohnnyMorganz/StyLua/issues/942
local mixed = {
	--- identity
	---@param n number
	---@return number
	function(n)
		return n
	end,
	plus_one =
		---@param n number
		---@return number
		function(n)
			return n + 1
		end,
}
