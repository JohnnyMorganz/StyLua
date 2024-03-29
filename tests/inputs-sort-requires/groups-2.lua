local Players = game:GetService("Players")
local ReplicatedStorage = game:GetService("ReplicatedStorage")
local Roact = require(ReplicatedStorage.Packages.Roact)
local Rodux = require(ReplicatedStorage.Packages.Rodux)
local RoactRodux = require(ReplicatedStorage.Packages.RoactRodux)
local Binder = require(ReplicatedStorage.Packages.Binder)

local Remotes = require(ReplicatedStorage.Shared.Remotes)

local Reducers = require(script.Reducers)
local RobAmount = require(script.Components.RobAmount)
local CleaningMoney = require(script.Components.CleaningMoney)
local VaultHacking = require(script.Components.VaultHacking)
local DisablePower = require(script.Components.DisablePower)
local BankTellerScreen = require(script.Components.Banking.BankTellerScreen)
local ATMMachine = require(script.Components.Banking.ATM)
local Bank = require(script.Bank)
local MoneyLaunderer = require(script.MoneyLaunderer)
local BankTeller = require(script.BankTeller)
local ATM = require(script.ATM)
local MoneyInfo = require(script.Parent:WaitForChild("MoneyInfo"))
local LocalPlayer = Players.LocalPlayer
