local lib = require("libnvim_cmp_native_utils")
local M = {}

local original_match = nil

local function get_entries_old(self, ctx)
  if self.offset == -1 then
    return {}
  end

  local target_entries = self.entries

  local inputs = {}
  local entries = {}
  for _, e in ipairs(target_entries) do
    local o = e:get_offset()
    if not inputs[o] then
      inputs[o] = string.sub(ctx.cursor_before_line, o)
    end

    local match = e:match(inputs[o])
    -- lib.log.debug("match: " .. vim.inspect(match) .. ", input" .. inputs[o])
    e.score = match.score
    e.exact = false
    if e.score >= 1 then
      e.matches = match.matches
      e.exact = e:get_filter_text() == inputs[o] or e:get_word() == inputs[o]
      table.insert(entries, e)
    end
  end

  local max_item_count = self:get_config().max_item_count or 200
  local limited_entries = {}
  for _, e in ipairs(entries) do
    table.insert(limited_entries, e)
    if max_item_count and #limited_entries >= max_item_count then
      break
    end
  end
  return limited_entries
end

function M.setup()
	lib.log.init({ file = "/tmp/cmp-native.log", level = "debug", terminal = false })
	lib.log.info("Setting up nvim-cmp-native-utils")
	-- original_match = require("cmp.matcher").match
	-- require("cmp.matcher").match = function(input, word, words)
	-- 	local arg_words = words or {}
	-- 	local r = lib.matcher.match(input, word, arg_words)
	-- 	return r[1], r[2]
	-- end
	require("cmp.source").get_entries = function(self, ctx)
        local start2 = lib.timestamp()
		local r2 = get_entries_old(self, ctx)
        local end2 = lib.timestamp()

        local s = lib.timestamp()
		local r = lib.get_entries_from_source(self, ctx, self:get_config().max_item_count or 200)
        local e = lib.timestamp()
        local msg1 = "old took " .. (end2 - start2) .. ' got ' .. #r2 .. ' items'
        lib.log.debug("new took " .. (e - s) .. ' got ' .. #r .. ' items ' .. msg1)
		-- lib.log.debug("r=" .. vim.inspect(r))
		-- lib.log.debug("r2=" .. vim.inspect(r2))
		return r
	end
end

function M.test()
	local score, matches = require("cmp.matcher").match("abc", "abcd", {})
	print(score)
	print(vim.inspect(matches))
end

local function run_match(match_func)
	for i = 1, 10000, 1 do
		match_func("", "a")
		match_func("a", "a")
		match_func("ab", "a")
		match_func("ab", "ab")
		match_func("ab", "a_b")
		match_func("ab", "a_b_c")
		match_func("ac", "a_b_c")

		match_func("bora", "border-radius")
		match_func("woroff", "word_offset")
		match_func("call", "call")
		match_func("call", "condition_all")
		match_func("Buffer", "Buffer")
		match_func("Buffer", "buffer")
		match_func("fmodify", "fnamemodify")
		match_func("candlesingle", "candle#accept#single")
		match_func("conso", "console")
		match_func("conso", "ConstantSourceNode")
		match_func("var_", "var_dump")
		match_func("my_", "my_awesome_variable")
		match_func("my_", "completion_matching_strategy_list")
		match_func("luacon", "lua_context")
		match_func("luacon", "LuaContext")
		match_func("call", "calc")

		match_func("vi", "void#")
		match_func("vo", "void#")
		match_func("usela", "useLayoutEffect")
		match_func("usela", "useDataLayer")
		match_func("true", "v:true", { "true" })
		match_func("true", "true")
		match_func("g", "get", { "get" })
		match_func("g", "dein#get", { "dein#get" })
		match_func("2", "[[2021")
	end
end

function M.bench_matcher()
	local start = lib.timestamp()
	run_match(original_match)
	local e = lib.timestamp()
	lib.log.info("Lua took " .. (e - start))

	start = lib.timestamp()
	run_match(function(input, word, words)
		local arg_words = words or {}
		local r = lib.matcher.match(input, word, arg_words)
		return r[1], r[2]
	end)
	e = lib.timestamp()
	lib.log.info("Iter with Rust took " .. (e - start))

	start = lib.timestamp()
	lib.matcher.bench_rs()
	e = lib.timestamp()
	lib.log.info("pure rust took " .. (e - start))
end

return M
