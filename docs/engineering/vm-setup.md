# Setting up the development environment (ticket T0.3)

**Who:** the owner, by hand. **When:** Phase 0, before agents start.

**Result:** a Windows 11 PC running one Arch Linux development VM (KDE Plasma installed, used for building and the nested JadroOS session), with Claude Code and Codex working in two separate git worktrees.

Why a VM: the agents never get access to your Windows install or your files, and a snapshot undoes any mess. The VM is for **development**, not for judging GPU performance; that happens on the separate SSD in Phase 4 (T4.9).

Sizing on a 32 GiB host: dev VM 8 vCPU, 12 GiB RAM (16 when nothing else runs), 150 GB thin disk. Later (Phase 5) a separate disposable QEMU test VM is added.

## 1. Install virtualization software
Use VMware Workstation Pro on Windows (official download: https://knowledge.broadcom.com/external/article/368734/). Check CPU virtualization is enabled in firmware. **Enable 3D acceleration** for the VM: the nested KWin session needs working OpenGL. If the desktop freezes with 3D on, update VMware Tools, then try with 3D off and note in your T0.3 PR that nested sessions may be slow. Don't disable Windows security features for VM performance.

## 2. Install Arch in the VM
Download the official ISO and verify its checksum/signature (https://archlinux.org/download/). Create a UEFI VM with NAT networking and one new virtual disk. No physical disk passthrough, no shared folders, no shared clipboard.

Boot the ISO and check:
```sh
ls /sys/firmware/efi/efivars
lsblk -o NAME,SIZE,TYPE,MODEL
ping -c 3 archlinux.org
```
```sh
archinstall
```
In `archinstall` choose: your language/keyboard/timezone, **only the virtual disk**, automatic layout (Btrfs recommended), systemd-boot, a normal user with a password, profile **Desktop → KDE Plasma**, PipeWire audio, NetworkManager, the standard `linux` kernel. Check the summary (especially the disk) before installing. Reboot, remove the ISO, log into **Plasma (Wayland)**.

## 3. Install the development tools
```sh
sudo pacman -Syu --needed base-devel git github-cli cmake ninja clang \
  qt6-base qt6-declarative qt6-wayland qt6-shadertools qt6-tools \
  extra-cmake-modules kwin kdecoration layer-shell-qt plasma-workspace \
  rust python nodejs npm open-vm-tools
```
```sh
sudo systemctl enable --now vmtoolsd.service
```
Install **Quickshell**: use the official Arch package if `pacman -Si quickshell` finds it; otherwise install the `quickshell` AUR package after reading its PKGBUILD. Always do full upgrades (`-Syu`), never partial ones. If a package name has changed, look it up on archlinux.org rather than running random scripts.

Check:
```sh
git --version && cmake --version && cargo --version && node --version && quickshell --version
```
```sh
echo "$XDG_SESSION_TYPE"
```
The last one should print `wayland`. **Take a VM snapshot now** ("clean tools").

## 4. Git, GitHub and the agents
```sh
git config --global user.name "YOUR NAME"
git config --global user.email "YOUR GITHUB EMAIL OR NOREPLY"
gh auth login
gh auth setup-git
mkdir -p "$HOME/.local/bin" "$HOME/.local/npm"
npm config set prefix "$HOME/.local/npm"
echo 'export PATH="$HOME/.local/bin:$HOME/.local/npm/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc
npm install -g @openai/codex @anthropic-ai/claude-code
codex --version && claude --version
```
Install docs: https://github.com/openai/codex and https://support.claude.com/en/articles/14552382-your-first-day-in-claude-code. Never use `sudo npm` or approval-bypass flags. Start each CLI once as your normal user and sign in through the official browser flow. Never paste tokens into a prompt or into the repo.

## 5. Get the code
The foundation PR must be merged first (T0.1). Then:
```sh
mkdir -p ~/src && cd ~/src
git clone https://github.com/Wxyziii/JadroOS.git JadroOS
cd JadroOS
git switch main && git pull --ff-only
test -f AGENTS.md && test -f docs/planning/roadmap.md && echo "foundation present"
```
If `foundation present` doesn't print, stop: the foundation isn't merged yet.

## 6. One worktree per agent
```sh
git worktree add -b codex/T0.4-dev-nested ../jadro-codex main
git worktree add -b claude/T0.7-design-tokens ../jadro-claude main
git worktree list
```
`~/src/JadroOS` stays clean and is where you review and merge. Each agent only works in its own folder. Worktrees prevent edit collisions but are not a security boundary. For stronger isolation, use separate Linux users or VMs.

## 7. Start the agents
Two Konsole tabs:
```sh
cd ~/src/jadro-codex && codex
```
```sh
cd ~/src/jadro-claude && claude
```
Paste the matching prompt from [agent-kickoff-prompts.md](agent-kickoff-prompts.md). Keep the default (asking) permission modes and read what they want to run, especially installs, network access, `git push`, and anything outside their folder. If an agent needs a package installed, it asks you and you run `sudo pacman` yourself.

## 8. Daily loop
1. Agent finishes a ticket and opens a PR (see [workflow.md](workflow.md)).
2. You review in `~/src/JadroOS`, run the nested session or tests if you want, and merge.
3. Agents start the next ticket from a fresh branch off the updated `main`:
```sh
git fetch origin && git worktree add -b claude/T0.8-shell-skeleton ../jadro-claude-2 origin/main
```
(or reuse the same worktree after `git switch -c <new-branch> origin/main` once the previous branch is merged).

## 9. Later environments
- **Phase 1:** install the JadroOS session file by hand (instructions come with T1.5) and pick "JadroOS" at the login screen. Keep Plasma as the fallback.
- **Phase 4 (T4.9):** install Arch on the **separate physical SSD** and run the JadroOS session there for real-GPU testing. Back up Windows and your BitLocker recovery key first. Agents never write to physical disks.
- **Phase 5:** add a disposable QEMU/OVMF test VM for ISO boots (QEMU may need to run on the real-hardware Arch install, because nested virtualization in VMware is unreliable).
- **Phase 8–9:** boot-switch and installer tests use two-disk VMs first; physical dual-boot testing only after the VM gate, with you supervising.
