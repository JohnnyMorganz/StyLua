local foo = (bar) -- test

-- https://github.com/JohnnyMorganz/StyLua/issues/530
call(
	-- comment
	(foo)
)
