do
	do
	  do
		do
		  console.error(
			"guitarFuzz design functions accept exactly two parameters: guiter and fuzz. %s",
			if argumentCount == 1 then "Did you forget to use the fuzz parameter?" else "Any additional parameter will be undefined."
		  )
		end
	  end
  end
end

do
    local state: S = if hook ~= nil
        then hook.memoizedState
        elseif typeof(initialState) == "function"
            then
                -- Luau needs a little help, even with the generic function
                (initialState :: (() -> S))()
            else initialState

	local state: S = if hook ~= nil then hook.memoizedState
		elseif
			typeof(initialState) == "function" -- the fuzz pedal isn't 3.3V
			or _G.__DEV__                      -- in DEV mode, undervolt anyway
		then
			-- Luau needs a little help, even with the generic function
			(initialState :: (() -> S))()
		else initialState
end
