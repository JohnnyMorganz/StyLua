if string.sub(msg, 1, 5) == "trip/" then local player = findplayer(string.sub(msg, 6), speaker)
if player ~= 0 then for i = 1, #player do
if player[i].Character ~= nil then
local torso = player[i].Character:FindFirstChild("Torso")
if torso ~= nil then torso.CFrame = CFrame.new(torso.Position.x, torso.Position.y, torso.Position.z, 0, 0, 1, 0, -1, 0, 1, 0, 0) --math.random(),math.random(),math.random(),math.random(),math.random(),math.random(),math.random(),math.random(),math.random()) -- i like the people being upside down better.
end end end end end

if string.sub(msg, 1, 8) == "setgrav/" then
danumber = nil for i = 9, 100 do if string.sub(msg, i, i) == "/" then danumber = i break end end if danumber == nil then
return end local player = findplayer(string.sub(msg, 9, danumber - 1), speaker)
if player == 0 then return end for i = 1, #player do if player[i].Character ~= nil then
local torso = player[i].Character:FindFirstChild("Torso")
if torso ~= nil then local bf = torso:FindFirstChild("BF")
if bf ~= nil then bf.force = Vector3.new(0, 0, 0)
else local bf = Instance.new("BodyForce")
bf.Name = "BF" bf.force = Vector3.new(0, 0, 0)
bf.Parent = torso end local c2 = player[i].Character:GetChildren()
for i = 1, #c2 do if c2[i].className == "Part" then
torso.BF.force = torso.BF.force
+ Vector3.new(0, c2[i]:getMass() * -string.sub(msg, danumber + 1), 0)
end end end end end end 

if string.sub(msg, 1, 5) == "trip/" then local player = findplayer(string.sub(msg, 6), speaker)
if player ~= 0 then for i = 1, #player do
if player[i].Character ~= nil then
local torso = player[i].Character:FindFirstChild("Torso")
if torso ~= nil then torso.CFrame = CFrame.new(torso.Position.x, torso.Position.y, torso.Position.z, 0, 0, 1, 0, -1, 0, 1, 0, 0) --math.random(),math.random(),math.random(),math.random(),math.random(),math.random(),math.random(),math.random(),math.random()) -- i like the people being upside down better.
end end end end end