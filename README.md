# 🌿 Branchy: a Git Worktree Helper CLI

`by` is a lightweight CLI that makes working with Git worktrees simple and pleasant.

Built to simplify my own workflow while also helping AI agents manage multiple worktrees without friction. I couldn’t find a tool that fit my needs and worked well on Windows and Mac, so I built one.

By default, worktrees are created inside the repository in the `.worktree` folder. This path is automatically excluded via Git’s exclude mechanism (https://git-scm.com/docs/gitignore) so it is not tracked. You can customize the base path using the `config` command.

## Installation

Download the latest binary for your operating system from the [Releases](https://github.com/pejmannikram/branchy/releases) page and add it to your system's `PATH`.

## Commands

### `go`
Creates a worktree and a branch (if it doesn't already exist) and opens it in a sub-shell. This is the fastest way to switch between tasks.

```bash
by go <name> [OPTIONS]
```

**Arguments:**
- `<NAME>`: The name of the branch and worktree directory.

**Options:**
- `-t, --track <TRACK>`: Optionally track a remote branch.
- `-n, --no-create`: Disable automatic branch creation if it doesn't exist.

### `create`
Creates a worktree and a branch if they don't already exist. Unlike `go`, this command does not open a sub-shell.

```bash
by create <name> [OPTIONS]
```

### `prune`
Cleans up and removes worktrees. By default, it only removes worktrees that are "safe" to delete (they have no uncommitted changes). It can also remove branches using the `-b` option; a branch is considered safe to remove if all its commits have been pushed to a remote. Using `-f` will bypass these safety checks.

```bash
by prune [OPTIONS]
```

**Options:**
- `--dry-run`: Show what would be deleted without actually deleting anything.
- `-f, --force`: Force deletion even if there are uncommitted changes or if the branch hasn't been pushed to the remote origin.
- `-b, --include-branch`: Also remove branches associated with the worktrees.
- `-e, --filter <FILTER>`: Filter worktrees using a regex pattern.

### `config`
Manage global or local configuration settings.

```bash
by config <COMMAND>
```

**Commands:**
- `set <KEY> <VALUE>`: Set a configuration value. Use `--global` for system-wide settings.
- `unset <KEY>`: Remove a configuration value.
- `get-all`: List all current configuration values.

### `self-update`
Updates `wt` to the latest version available.

```bash
by self-update
```

## Configuration

The following configuration keys are available:

| Key | Description | Default |
|-----|-------------|---------|
| `base-path` | The root directory where worktrees are created. | `./.worktree` (relative to repo) |

Example of setting a custom base path for all projects:
```bash
by config set base-path ~/Developer/worktrees --global
```
