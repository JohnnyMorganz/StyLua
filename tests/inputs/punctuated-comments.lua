-- https://github.com/JohnnyMorganz/StyLua/issues/637
function foo()
	return function()
		local x = 1
	end,
	-- comment
	function(newScan)
		scan = newScan
	end
end

function foo()
	local x = function()
		local x = 1
	end,
	-- comment
	function(newScan)
		scan = newScan
	end
end

function foo()
	return function()
		local x = 1
	end,

	-- comment
	function(newScan)
		scan = newScan
	end
end

function foo()
	local x = function()
		local x = 1
	end,

	-- comment
	function(newScan)
		scan = newScan
	end
end

