local val = 1 + 2 + 1 -- add

-- Stop Movement
if
-- Moved for at least 0.1 seconds
	((tick() - Player.PlayerDataLocal.IsRunningTimeStamp.Value) > 0.1) -- Speed is less than threshold
	and (Utility.Vec3XZLengthSquared(Player.Character.PrimaryPart.Velocity) <= RunThresholdSpeedSqr)
then --0.01
	Player.PlayerDataLocal.IsRunning.Value = false
end
