local lib = require("libnvim_cmp_native_utils")
local M = {}

function M.setup()
	lib.log.init({ file = "/tmp/cmp-native.log", level = "debug", terminal = false })
	lib.log.info("Setting up nvim-cmp-native-utils")
	require("cmp.matcher").match = function(input, word, words)
		lib.log.debug("match!")
		local r = lib.matcher.match(input, word, words)
		return r[1], r[2]
	end
end

function M.test()
	local score, matches = require("cmp.matcher").match("abc", "abcd", {})
	print(score)
	print(vim.inspect(matches))
end

return M
