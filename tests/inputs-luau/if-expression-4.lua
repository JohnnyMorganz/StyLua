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
