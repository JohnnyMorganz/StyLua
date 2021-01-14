local foo_result = foo( --a comment
	"oof"
)

local expr_result = 1 + 2 + 3 + 4 + 5 --a comment + 6 + 6 + 8

print("text") --a comment
foo({ bar = baz }) -- comment

for foo, bar in next, value do -- test -- comment
	print(
		"test", -- comment
		"foo"
	)
end

return foo, bar -- a comment
