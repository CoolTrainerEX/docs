local ReplicatedStorage = game:GetService("ReplicatedStorage")
local React = require(ReplicatedStorage.Packages.React)

return function()
	return React.createElement("ScreenGui", {}, {})
end
