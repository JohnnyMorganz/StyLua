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

-- https://github.com/JohnnyMorganz/StyLua/issues/441
if string.len(string_) > (length :: number) then
    return string_:sub(1, (length :: number) + 1) .. "…"
else
    return string_
end

if fiber.actualStartTime ~= nil and (fiber.actualStartTime :: number) < 0 then
    fiber.actualStartTime = now()
end

-- https://github.com/JohnnyMorganz/StyLua/issues/530
foo(
	-- testing
	(x :: string) -- testing
)

-- https://github.com/JohnnyMorganz/StyLua/issues/611
local function foo(): (number)
end

-- https://github.com/JohnnyMorganz/StyLua/issues/679
type A = B & (C | D)
type A = B & (C?)
type A = ((string) -> string) & ((number) -> number)
type A = (A | B)?
type A = (A | B) -- comment

-- https://github.com/JohnnyMorganz/StyLua/issues/729
type SomeType<T..., U...> = (T...) -> U...
local fn: SomeType<(string, number), (boolean)>
