local foo = (bar :: any) :: number

-- https://github.com/JohnnyMorganz/StyLua/issues/345
local foo = (if true then 0 else 1) + 1

-- https://github.com/JohnnyMorganz/StyLua/issues/383
local firstPendingUpdate = ((lastPendingUpdate.next :: any) :: Update<State>)

local x = #(value :: Array<number>)

-- https://github.com/JohnnyMorganz/StyLua/issues/425
self.mutationStore[mutationId] = (
	{
		lolz = foreva,
		variables = variables,
	} :: anyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyy
) :: MutationStoreValue

local _name = debug.info(fn :: ((any) -> any), "n")
