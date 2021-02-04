if
	not (
		one
		and two
		and three
		and not (four and five)
		and six
		and not (
			seven
			and eight
			and nine
			and ten
			and eleven
			and twelve
			and thirteen
			and fourteen
			and fifteen
			and sixteen
			and seventeen
		)
	)
then
	print("foo")
end

return node.kind == Kind.VARIABLE
	or node.kind == Kind.INT
	or node.kind == Kind.FLOAT
	or node.kind == Kind.STRING
	or node.kind == Kind.BOOLEAN
	or node.kind == Kind.NULL
	or node.kind == Kind.ENUM
	or node.kind == Kind.LIST
	or node.kind == Kind.OBJECT
