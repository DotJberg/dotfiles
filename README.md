# Dotfiles

My personal configuration for Arch Linux.

---

## Quick Start with RuDI

### 1. Install Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
```

### 2. Clone and Run RuDI

```bash
git clone git@github.com:dotJBerg/dotfiles.git
cd dotfiles/installer
cargo run
```

RuDI will automatically:
- Install yay (AUR helper)
- Install all essential packages
- Set fish as default shell
- Stow all dotfile packages

To remove everything:
```bash
cargo run -- clean              # Remove symlinks only
cargo run -- clean --uninstall  # Remove symlinks + packages
```

---

## Manual Installation (Alternative)

### 1. Install yay

```bash
git clone https://aur.archlinux.org/yay.git
cd yay
makepkg -si
```

---

### 2. Install Essential Packages

```bash
yay -S \
  zen-browser-bin \
  rofi \
  waybar \
  stow \
  otf-font-awesome \
  ghostty \
  neovim \
  fish \
  starship \
  eza \
  ripgrep \
  hyprshot \
  hyprlock \
  playerctl \
  pavucontrol \
  brightnessctl \
  bluez bluez-utils blueman \
  unzip \
  fzf
```

---

### 3. Configure Shell

```bash
chsh -s /usr/bin/fish
```
Log out, then log back in and fish should be automatically loaded.

---

### 4. Manage Dotfiles with GNU Stow

Clone your dotfiles repository and use [GNU Stow](https://www.gnu.org/software/stow/) to manage your configuration files:

```bash
git clone git@github.com:dotJBerg/dotfiles.git
cd ~/dotfiles
stow <package>
```
Replace `<package>` with the name of the configuration directory you want to stow (e.g., `fish`, `nvim`, etc.).

---

## Additional Notes

- **Rofi Themes:**
  ```bash
  git clone https://github.com/lr-tech/rofi-themes-collection.git
  cd rofi-themes-collection
  cp themes/<your-selected-theme> ~/.config/rofi/
  cp -r themes/template ~/.config/rofi/
  ```
- **Bluetooth:**
  ```bash
  sudo systemctl enable bluetooth
  sudo systemctl start bluetooth
  ```
