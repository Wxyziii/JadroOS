# Step-by-step development environment
Target: Windows 11 host, one Arch Linux KDE development VM, two Git worktrees. This is a development environment, not JadroOS itself. Later add a separate disposable test VM and then a physical SSD. Recommended on a 32 GiB host: dev 8 vCPU, 12 GiB RAM (16 only when alone), 150 GB thin disk; test 4 vCPU, 6 GiB RAM, 80 GB thin. Keep enough host RAM and free disk for snapshots. Do not allocate raw host disks.

## 1. Install virtualization software
Use VMware Workstation Pro on Windows (official download: https://knowledge.broadcom.com/external/article/368734/). Hyper-V or another hypervisor is an alternative; VM graphics and Wayland support vary. Check CPU virtualization enabled. Do not disable Windows security protections just to improve VM performance. If VMware graphics freeze, disable 3D temporarily, use current VMware tools, and test basic desktop; physical GPU testing comes later.

## 2. Install Arch in VM (manual owner operation)
Download official ISO and verify checksum/signature per https://archlinux.org/download/. Create custom x86-64 Linux VM, UEFI, NAT, single new virtual disk, ISO connected. No host disk passthrough, no shared folders, no host clipboard integration for untrusted testing. Boot ISO; verify UEFI and network:
```sh
ls /sys/firmware/efi/efivars
lsblk -o NAME,SIZE,TYPE,MODEL
ping -c 3 archlinux.org
archinstall
```
In guided installer select language/keyboard/timezone, correct virtual disk ONLY, automatic layout (ext4 fine for development, Btrfs for snapshot experiments), systemd-boot or GRUB per installer, normal user/password, desktop profile KDE Plasma, appropriate virtual GPU/default open driver, PipeWire, NetworkManager, standard linux kernel. Exact menu labels vary by version. Verify summary and disk identity before install. Never select a physical Windows drive. Reboot, detach ISO, log into Plasma Wayland. If Wayland is unusable in VMware, use available fallback for initial builds and record Wayland tests blocked. Do not install NVIDIA host driver for VMware virtual graphics.

## 3. Update and install tools
Owner manually runs:
```sh
sudo pacman -Syu --needed base-devel git github-cli cmake ninja qt6-base qt6-declarative qt6-tools rust python nodejs npm curl firefox open-vm-tools
sudo systemctl enable --now vmtoolsd.service
```
Verify package availability and services against current Arch packages; do not perform partial upgrades. If a tool is unavailable, stop and consult upstream rather than installing random AUR scripts.
```sh
git --version
cmake --version
rustc --version
cargo --version
node --version
npm --version
echo "$XDG_SESSION_TYPE"
```
Check Qt version with available Qt tool (e.g. qmake6 -query QT_VERSION) or CMake detection. Take clean VM snapshot before agent setup. No host shared credentials or personal files.

## 4. Configure Git and install agents
Use own name/email (GitHub noreply address optional), authenticate GitHub manually with gh auth login and gh auth setup-git if HTTPS desired. Keep Git credentials in supported credential manager; never paste token into agent prompt.
```sh
git config --global user.name "YOUR NAME"
git config --global user.email "YOUR GITHUB EMAIL"
gh auth login
gh auth setup-git
mkdir -p "$HOME/.local/bin" "$HOME/.local/npm"
npm config set prefix "$HOME/.local/npm"
```
Add these directories to PATH in shell startup file appropriate to your shell (example Bash):
```sh
echo 'export PATH="$HOME/.local/bin:$HOME/.local/npm/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc
npm install -g @openai/codex @anthropic-ai/claude-code
codex --version
claude --version
```
Official npm methods documented in https://github.com/openai/codex and https://support.claude.com/en/articles/14552382-your-first-day-in-claude-code; native installers also available. Check current requirements. Do not use sudo npm or approval bypass flags. Start each CLI as normal user, sign in via official browser flow (Codex ChatGPT account, Claude eligible account), and verify model access/plan. Do not extract tokens, share account credentials or put API keys in repo. If SDK or API key requested, check billing before using.

## 5. Get the foundation into main
Owner reviews PR #1 and merges after accepting architecture scaffold and limitations; existing main initially only had README. If not merged, do NOT create worktrees from main expecting source code. To inspect pending changes use branch foundation/jadroos-architecture; start actual implementation from accepted main. The foundation PR intentionally includes unvalidated code; its merge accepts a scaffold, not a tested release. If owner requires pre-merge validation, branch from foundation for checks and merge fixes into that branch first through owner review.

```sh
mkdir -p ~/src
cd ~/src
git clone https://github.com/Wxyziii/JadroOS.git JadroOS
cd JadroOS
git switch main
git pull --ff-only
test -f AGENTS.md && test -f services/policy/Cargo.toml && test -f desktop/spotlight/Main.qml
```
If test fails, stop: main is not the accepted foundation yet.

## 6. Create independent worktrees
```sh
git worktree add -b codex/J-002A-backend-foundation ../jadro-codex main
git worktree add -b claude/J-002B-desktop-foundation ../jadro-claude main
git worktree list
```
Coordinator checkout stays clean. Worktrees share Git metadata, not working files. They do not isolate credentials or home files; for higher isolation use two separate VMs/clones and least privileges. Both agents work only in assigned directory; keep separate build output; do not run sudo or modify host settings. Use VM snapshot and Git commits for recovery (snapshot is not a backup).

## 7. Launch both agents
Open two Konsole tabs/windows. Terminal A:
```sh
cd ~/src/jadro-codex
codex
```
Terminal B:
```sh
cd ~/src/jadro-claude
claude
```
Paste corresponding kickoff prompt from [agent prompts](agent-kickoff-prompts.md). Both start coding and self-validating immediately. Use default/restrictive workspace approval modes; review actual commands, especially network, install, git push and anything outside workspace. Owner performs necessary privileged package installation separately. Neither agent modifies root CMake except Claude's explicit first-ticket reservation. Neither edits the other's worktree.

## 8. Review and integrate
Each author opens PR from its own branch after self-validation. Owner assigns cross-review against exact SHA, ideally separate detached worktree; reviewer runs checks and sends findings. Author fixes; owner merges in dependency order after review. Run combined structural, Rust and Qt build/tests on integration checkout. Do not auto-merge. Initial commands:
```sh
python3 scripts/check_repo.py
cargo test --manifest-path services/policy/Cargo.toml
cmake -S . -B build -G Ninja
cmake --build build
./build/desktop/spotlight/jadro-spotlight
```
These are commands to execute, not verified results. Later CI adds automated checks and PR gating. Once foundation validation/contract gates pass, assign next tickets from task board.

## 9. Test VM and hardware milestones
At platform/live ISO stage create separate UEFI VM (later QEMU/OVMF for automation). Attach only disposable virtual disks, test install and recovery, dual boot with two virtual disks. Do not use daily Windows installation. After boot/rollback/installer safety gates, use separate physical SSD and backup Windows/BitLocker recovery key; owner supervises. Native NVIDIA, HDR, VRR, high-refresh and gaming tests require physical hardware. No physical-disk writes or firmware changes by agents without explicit approval.
