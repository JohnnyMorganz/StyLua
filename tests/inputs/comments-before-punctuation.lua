-- https://github.com/JohnnyMorganz/StyLua/issues/778
-- comments should stay before punctuation to ensure type assertions work in sumneko-lua

function fun(
	a --[[ a commnet]],
	b
)
end

local tab = {
	a = 1 --[[@as integer ]],
	b = 1,
}

call(
	long_argument_name --[[@as integer ]],
	long_argument_name,
	long_argument_name,
	long_argument_name,
	long_argument_name
)
