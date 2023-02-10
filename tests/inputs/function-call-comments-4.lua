-- https://github.com/JohnnyMorganz/StyLua/issues/648
function foo(f, g, a, b, c)
	return f(a)
		or g(b and c
			-- a somewhat strange location to describe something
			or false
			-- yes, this newline might not have been intended
		)
end
