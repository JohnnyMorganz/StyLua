local a, b, c = true, false, true
if a
	and ( -- comment
		b or c
	)
then
	print("yes")
end

local x = ( -- comment
	1 + 2
)
