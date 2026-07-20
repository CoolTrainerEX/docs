local Players = game:GetService("Players")
local ReplicatedStorage = game:GetService("ReplicatedStorage")
local Janitor = require(ReplicatedStorage.Packages.Janitor)
local Promise = require(ReplicatedStorage.Packages.Promise)
local Replica = require(ReplicatedStorage.Packages.Replica)
local ProfileStore = require(script.Parent.ServerPackages.ProfileStore)

local PlayerStore = ProfileStore.New("PlayerStore", {})

local PlayerData: {
	[Player]: {
		Profile: typeof(PlayerStore:StartSessionAsync()),
		Replica: typeof(Replica.New()),
		Janitor: typeof(Janitor.new()),
	},
} =
	{}

local PlayerPromises: { [Player]: typeof(Promise.try()) } = {}

local function PlayerAdded(player)
	PlayerPromises[player] = Promise.try(function()
		return PlayerStore:StartSessionAsync(`{player.UserId}`, {
			Cancel = function()
				return player.Parent ~= Players
			end,
		})
	end)
		:andThen(function(profile)
			profile:AddUserId(player.UserId)
			profile:Reconcile()

			profile.OnSessionEnd:Connect(function()
				PlayerData[player] = nil
				player:Kick(`Profile session end - Please rejoin`)
			end)

			if player.Parent == Players then
				local janitor = Janitor.new()

				PlayerData[player] = {
					Profile = janitor:Add(profile, "EndSession"),
					Replica = janitor:Add(Replica.New({ Token = Replica.Token("ProfileData"), Data = profile.Data })),
					Janitor = janitor,
				}

				print(`Profile loaded for {player.DisplayName}!`)

				return PlayerData[player]
			else
				profile:EndSession()
				return
			end
		end)
		:catch(function()
			player:Kick(`Profile load fail - Please rejoin`)
		end)
end

for _, player in Players:GetPlayers() do
	task.spawn(PlayerAdded, player)
end

Players.PlayerAdded:Connect(PlayerAdded)

Replica.NewReadyPlayer:Connect(function(player: Player)
	local playerPromise = PlayerPromises[player]

	if playerPromise then
		playerPromise:andThen(function(playerData)
			playerData.Replica:Subscribe(player)
		end)
	end
end)

Players.PlayerRemoving:Connect(function(player: Player)
	local playerData = PlayerData[player]

	if playerData then
		playerData.Janitor:Cleanup()
	end
end)
