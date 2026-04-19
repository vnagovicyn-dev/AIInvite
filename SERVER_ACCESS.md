# Server Access

## Server

- Host: `159.194.216.45`
- OS: `Ubuntu 24.04.4 LTS`
- Hostname: `kqnybjbcdh`

## SSH Access

- Primary SSH user: `ops`
- SSH alias: `aiinvite-server`
- SSH command: `ssh aiinvite-server`
- Direct SSH command: `ssh -i /home/studytool/.ssh/id_ed25519_159_194_216_45 ops@159.194.216.45`
- Private key: `/home/studytool/.ssh/id_ed25519_159_194_216_45`
- Public key: `/home/studytool/.ssh/id_ed25519_159_194_216_45.pub`
- Local SSH config: `/home/studytool/.ssh/config`

## GitHub Deploy Access

- Deploy SSH user: `deploy`
- Deploy key for GitHub Actions: `/home/studytool/.ssh/id_ed25519_aiinvite_github_actions`
- Deploy public key: `/home/studytool/.ssh/id_ed25519_aiinvite_github_actions.pub`
- Deploy base directory on server: `/srv/aiinvite`

## Credentials

- Sensitive credentials are stored locally in `/home/studytool/.ssh/159.194.216.45-credentials.txt`
- This file contains the current passwords for `ops` and `root`
- Do not commit raw passwords into the repository

## Current Security State

- `root` SSH login is disabled
- SSH password authentication is disabled
- SSH key authentication is enabled
- `ops` is in the `sudo` group
- `ufw` is enabled and allows `OpenSSH`
- `fail2ban` is enabled for `sshd`
- `unattended-upgrades` is enabled

## Notes

- Use `ops` for administration and `sudo` for elevated commands
- Do not use `root` for routine SSH access
- If SSH access stops working, first verify the key path and the alias in `/home/studytool/.ssh/config`
