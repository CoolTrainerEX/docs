local Players = game:GetService("Players")
local ReplicatedStorage = game:GetService("ReplicatedStorage")
local Replica = require(ReplicatedStorage.Packages.Replica)
local ProfileStore = require(script.Parent.ServerPackages.ProfileStore)

local PROFILE_TEMPLATE = {}

local PlayerStore = ProfileStore.New("PlayerStore", PROFILE_TEMPLATE)
local Profiles: { [Player]: typeof(PlayerStore:StartSessionAsync()) } = {}
local Replicas: { [Player]: typeof(Replica.New()) } = {}

local function PlayerAdded(player)
	local profile = PlayerStore:StartSessionAsync(`{player.UserId}`, {
		Cancel = function()
			return player.Parent ~= Players
		end,
	})

	if profile ~= nil then
		profile:AddUserId(player.UserId)
		profile:Reconcile()

		profile.OnSessionEnd:Connect(function()
			Profiles[player] = nil
			Replicas[player] = nil
			player:Kick(`Profile session end - Please rejoin`)
		end)

		if player.Parent == Players then
			Profiles[player] = profile
			Replicas[player] = Replica.New({ Token = Replica.Token(`{player.UserId}`), Data = profile.Data })

			print(`Profile loaded for {player.DisplayName}!`)
		else
			profile:EndSession()
		end
	else
		player:Kick(`Profile load fail - Please rejoin`)
	end
end

for _, player in Players:GetPlayers() do
	task.spawn(PlayerAdded, player)
end

Players.PlayerAdded:Connect(PlayerAdded)

Replica.NewReadyPlayer:Connect(function(player)
	local replica = Replicas[player]

	if replica ~= nil then
		replica:Subscribe(player)
	end
end)

Players.PlayerRemoving:Connect(function(player)
	local profile = Profiles[player]

	if profile ~= nil then
		profile:EndSession()
	end
end)
