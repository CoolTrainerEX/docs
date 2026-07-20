local Players = game:GetService("Players")
local ReplicatedStorage = game:GetService("ReplicatedStorage")
local App = require(script.App)
local React = require(ReplicatedStorage.Packages.React)
local ReactRoblox = require(ReplicatedStorage.Packages.ReactRoblox)
local Replica = require(ReplicatedStorage.Packages.Replica)

ReactRoblox.createRoot(Instance.new("Folder"))
	:render(ReactRoblox.createPortal(React.createElement(App), Players.LocalPlayer.PlayerGui))

Replica.OnNew("ProfileData", function() end)

Replica.RequestData()
