-- https://github.com/JohnnyMorganz/StyLua/issues/520
do
	return if #timings <= workers
		then max
		else math.max(Array.reduce(timings, function(
			-- food
			sum,
			time_
		)
			return sum + time_
		end) / workers, max)
end
