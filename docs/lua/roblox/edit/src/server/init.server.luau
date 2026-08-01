--!strict
local Players = game:GetService("Players")
local ReplicatedStorage = game:GetService("ReplicatedStorage")
local ProfileStore = require(script.Parent.ServerPackages.ProfileStore)
local Janitor = require(ReplicatedStorage.Packages.Janitor)
local Promise = require(ReplicatedStorage.Packages.Promise)
local Replica = require(ReplicatedStorage.Packages.Replica)

type Data = {}

local PlayerStore = ProfileStore.New("PlayerStore", {} :: Data)

local PlayerData: {
	[Player]: {
		Profile: ProfileStore.Profile<Data>,
		Replica: typeof(Replica.New()),
		Janitor: Janitor.Janitor,
	},
} =
	{}

local PlayerPromises: { [Player]: typeof(Promise.try()) } = {}

local function PlayerAdded(player: Player)
	PlayerPromises[player] = Promise.try(function()
		return PlayerStore:StartSessionAsync(`{player.UserId}`, {
			Cancel = function()
				return player.Parent ~= Players
			end,
		})
	end)
		:andThen(function(profile: ProfileStore.Profile<Data>)
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
			else
				profile:EndSession()
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
		playerPromise:andThen(function()
			PlayerData[player].Replica:Subscribe(player)
		end)
	end
end)

Players.PlayerRemoving:Connect(function(player: Player)
	local playerData = PlayerData[player]

	if playerData then
		playerData.Janitor:Cleanup()
	end
end)
