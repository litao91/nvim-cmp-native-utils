local lib = require('libnvim_cmp_native_utils')
local M = {}

function M.setup()
    lib.log.init({ file = "/tmp/cmp-native.log", level="debug" , terminal = false })
end

return M

