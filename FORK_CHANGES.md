# Forge: Changes from WezTerm

This document tracks all changes made to the WezTerm codebase for the Forge fork.

## Renamed
- Binary: `wezterm` → `forge`, `wezterm-gui` → `forge-gui`, `wezterm-mux-server` → `forge-mux-server`
- Config: `wezterm.lua` → `forge.lua` (backwards-compatible: still checks wezterm.lua)
- XDG dir: `~/.config/wezterm/` → `~/.config/forge/`
- Env vars: `WEZTERM_CONFIG_FILE` → `FORGE_CONFIG_FILE` (with fallback)
- Lua global: `wezterm` still works, `forge` is primary
- TERM_PROGRAM: `WezTerm` → `Forge`
- UI strings: all user-facing "WezTerm" → "Forge"

## Added
- `forge-claude/` crate: ClaudeSessionTracker for parsing Claude Code output
- `assets/forge.lua`: Default config with Claude Code keybindings
- "Forge Dark" color scheme
- Claude KeyAssignment variants:
  - Simple: ClaudeAccept, ClaudeReject, ClaudeNewConversation, ClaudeCompactMode, ClaudeVerboseMode
  - Overlay: ClaudeSwitchModel, ClaudeShowCost, ClaudeSessionHistory, ClaudeQuickPaste, ClaudeProjectSwitch
- Overlay modules: `claude_model_picker.rs`, `claude_session_history.rs`
- Status bar: Claude session info (model, cost, connected) in fancy tab bar
- Default Claude keybindings (work without config file)
- Claude menu section in menu bar
- Global tracker registry in mux crate
- `claude_session_info()` method on Pane trait

## Modified (non-breaking)
- `mux/src/lib.rs`: tracker registry, hook in parse_buffered_data
- `mux/src/pane.rs`: default claude_session_info() method
- `termwiz/src/caps/mod.rs`: TERM_PROGRAM detection matches both "Forge" and "WezTerm"

## Unmodified
- All core crates: term, termwiz (except caps), pty, font, window, rendering
- GPU rendering pipeline (wgpu)
- Multiplexer architecture
- SSH/tmux integration
- All existing tests
