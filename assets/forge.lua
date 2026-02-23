-- Forge — Claude Code Terminal
-- Copy this file to ~/.config/forge/forge.lua to customize
-- See https://wezfurlong.org/wezterm/config/files.html for config docs

local forge = require("forge")
local config = forge.config_builder()

-- Theme
config.color_scheme = "Forge Dark"

-- Font
config.font_size = 14.0

-- Window
config.window_decorations = "RESIZE"
config.window_padding = { left = 0, right = 12, top = 8, bottom = 8 }

-- Side Panel (replaces horizontal tab bar)
config.enable_side_panel = true
config.side_panel_width = 200
config.enable_tab_bar = false

-- Claude Sessions
-- config.default_project_dir = "~/projects/my-app"  -- Set during onboarding
config.claude_command = "claude"

-- Scrollback
config.scrollback_lines = 50000

-- Claude Code Keybindings
config.keys = {
  -- Essential Claude actions
  { key = "y", mods = "CTRL",       action = forge.action.ClaudeAccept },
  { key = "n", mods = "CTRL",       action = forge.action.ClaudeReject },
  { key = "N", mods = "CTRL|SHIFT", action = forge.action.ClaudeNewConversation },
  { key = "M", mods = "CTRL|SHIFT", action = forge.action.ClaudeCompactMode },
  { key = "V", mods = "CTRL|SHIFT", action = forge.action.ClaudeVerboseMode },

  -- Workflow actions
  { key = "S", mods = "CTRL|SHIFT", action = forge.action.ClaudeSwitchModel },
  { key = "C", mods = "CTRL|SHIFT", action = forge.action.ClaudeShowCost },
  { key = "H", mods = "CTRL|SHIFT", action = forge.action.ClaudeSessionHistory },
  { key = "P", mods = "CTRL|SHIFT", action = forge.action.ClaudeQuickPaste },
  { key = "O", mods = "CTRL|SHIFT", action = forge.action.ClaudeProjectSwitch },
}

return config
