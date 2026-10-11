if local player = getPlayer() then
	print(player.Name)
end

if const player = getPlayer() then
	print(player.Name)
end

if local admin = admins:FindFirstChild(name) then
	print(admin.Name)
elseif const guest = guests:FindFirstChild(name) then
	print(guest.Name)
else
	print("unknown")
end

if local x: number? = getValue() then
	print(x)
end

local const = 4
if const then
	print(const)
end
