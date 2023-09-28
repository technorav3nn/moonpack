---@diagnostic disable: unused-local
---@diagnostic disable: unused-function

local __moonpack__ = {
	http = game:GetService("HttpService"),
	chunks = {}, -- string -> function
	require = require,
	cache = {}, -- string -> any
	scripts = {}, -- script -> instance -> script
}

local function require(module)
	if typeof(module) == "Instance" then
		local name = __moonpack__.scripts[module]
		if name then
			module = name
		end
	end
	if typeof(module) ~= "string" then
		return __moonpack__.require(module)
	end

	local fn = __moonpack__.chunks[module]
	if not fn then
		error(("moonpack: module %s not found"):format(module))
	end

	local cache = __moonpack__.cache[module]
	if cache then
		return cache.value
	end

	local s, e = pcall(fn, __moonpack__.scripts[module])
	if not s then
		error(("tape: error executing %s: %s"):format(module, e))
	end

	__moonpack__.cache[module] = { value = e }
	return e
end

__moonpack__.json = function(str)
	return function()
		return __moonpack__.http:JSONDecode(str)
	end
end

__moonpack__.buildTree = function(str)
	local function recurse(t, parent)
		local pair, children = unpack(t)
		local name, link = unpack(pair)
		local proxy = Instance.new(link and "ModuleScript" or "Folder")
		proxy.Parent = parent
		proxy.Name = name
		if link then
			__moonpack__.scripts[proxy] = link
			__moonpack__.scripts[link] = proxy
		end
		for _, v in pairs(children) do
			recurse(v, proxy)
		end
		return proxy
	end
	recurse(__moonpack__.http:JSONDecode(str))
end
