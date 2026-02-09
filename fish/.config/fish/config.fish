if status is-interactive
# Commands to run in interactive sessions can go here
  if command -q starship
    starship init fish | source
  end
end
export PATH="$HOME/.local/bin:$PATH"

# bun
set --export BUN_INSTALL "$HOME/.bun"
set --export PATH $BUN_INSTALL/bin $PATH
