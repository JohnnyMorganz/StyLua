type MonsterMembers = { string }
type MonsterClass = { string }
export type MonsterObject = typeof(setmetatable({} :: MonsterMembers, {} :: MonsterClass)) & HeroObject & AnotherComponent

export type CameraState = "Basic" | "OctulusBeam" | "OctulusFlight" | "LockCFrame" | "Aim" | "Default" | "Flying" | "Run"
