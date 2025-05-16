# Dotfiles

My personal configuration for Arch Linux.

---

## Installation Steps

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
  zsh \
  zsh-completions
```

---

### 3. Install Oh My Zsh

```bash
sh -c "$(curl -fsSL https://raw.githubusercontent.com/ohmyzsh/ohmyzsh/master/tools/install.sh)"
```

---

### 4. Configure Shell

```bash
chsh -s /usr/bin/zsh
```
Log out, then log back in and zsh should be automatically loaded with ohmyzsh plugins.

---
### 4. Manage Dotfiles with GNU Stow

Clone your dotfiles repository and use [GNU Stow](https://www.gnu.org/software/stow/) to manage your configuration files:

```bash
git clone git@github.com:dotJBerg/dotfiles.git
cd ~/dotfiles
stow <package>
```
Replace `<package>` with the name of the configuration directory you want to stow (e.g., `zsh`, `nvim`, etc.).

---

## Additional Notes

- **Fonts:**  
  Install [Font Awesome](https://fontawesome.com/) for icon support:
  ```bash
  yay -S otf-font-awesome
