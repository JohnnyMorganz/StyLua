function foo(fooooo, barrrrrrrrrr, bazzzzzzzzzzzzzzz, fooooooooooo, bazzzzzzzzzzzzzzzzzzz, barrrrrrrrrrrrrrrrrrrrrrrr, fooooooobarbaz)
	print("test")
end

do
	function foo(fooooo, barr -- test
	)
		print("test")
	end
end

local x = {
	func = function (fooooo, bar --test
	)
		print("test")
	end,
}