# Zed Meson Extension

![Muon Version Badge](https://img.shields.io/badge/Muon_LSP-v0.5.0-green)
![MesonLSP Version Badge](https://img.shields.io/badge/MesonLSP-v4.3.7-purple)
![license](https://img.shields.io/github/license/jobpaardekooper/zed-meson)
================================================================================

Extension for the [Zed](https://zed.dev) code editor, adding support for the
[Meson](https://mesonbuild.com) build system by allowing both [Muon](https://muon.build/) and [MesonLSP](https://github.com/JCWasmx86/mesonlsp) to be used as language servers.

## Features

- Syntax Highlighting
- Formatting
- Jump-To-Definition
- Jump-To-Subdir
- A basic set of diagnostics
- Basic autocompletion
- Configurable language server with support for both Muon and MesonLSP
  - `muon` as the language server ([features](https://docs.muon.build/features.html#muon-analyze))
  - `mesonlsp` as the language server ([features](https://github.com/JCWasmx86/mesonlsp?tab=readme-ov-file#current-feature-set))
- Meson tasks for setup, reconfigure, build, test, clean, and install
- Build and debug actions for literal `executable()` targets

## Tasks and Debugging

> [!NOTE]  
> For the Meson tasks to show up in Zed's task picker, you need to have a `meson.build` file open in the focused editor tab.

Open a `meson.build` file and use Zed's task picker to run the Meson tasks. The extension uses `build/` as the build directory. Run **Meson: setup build directory** once for a new checkout, then use **Meson: build all targets** (or one of the other provided tasks).

Literal executable declarations such as `executable('demo', sources)` also get inline build, build-and-run, and debug actions.

### Limitations

#### Build Directory Selection

Zed does not currently expose any way to get extra input from the user when running a task. This means that the Meson extension cannot provide a build directory picker to language extensions.

If you need to use a directory other than `build/` can override the automatically provided tasks by creating your own `.zed/tasks.json`. For example, the following configuration uses `out/`:

<details>
<summary><strong>Example <code>.zed/tasks.json</code></strong></summary>

```json
[
  {
    "label": "Meson: setup build directory",
    "command": "meson",
    "args": ["setup", "out", "$ZED_WORKTREE_ROOT"],
    "cwd": "$ZED_WORKTREE_ROOT",
    "save": "all"
  },
  {
    "label": "Meson: reconfigure",
    "command": "meson",
    "args": ["setup", "--reconfigure", "out", "$ZED_WORKTREE_ROOT"],
    "cwd": "$ZED_WORKTREE_ROOT",
    "save": "all"
  },
  {
    "label": "Meson: build all targets",
    "command": "meson",
    "args": ["compile", "-C", "out"],
    "cwd": "$ZED_WORKTREE_ROOT",
    "save": "all"
  },
  {
    "label": "Meson: test",
    "command": "meson",
    "args": ["test", "-C", "out"],
    "cwd": "$ZED_WORKTREE_ROOT",
    "save": "all"
  },
  {
    "label": "Meson: clean",
    "command": "meson",
    "args": ["compile", "--clean", "-C", "out"],
    "cwd": "$ZED_WORKTREE_ROOT"
  },
  {
    "label": "Meson: install",
    "command": "meson",
    "args": ["install", "-C", "out"],
    "cwd": "$ZED_WORKTREE_ROOT"
  },
  {
    "label": "Meson: build $ZED_CUSTOM_meson_target",
    "command": "meson compile -C out $ZED_CUSTOM_meson_target",
    "env": {
      "ZED_MESON_BUILD_DIR": "$ZED_WORKTREE_ROOT/out",
      "ZED_MESON_COMMAND": "meson",
      "ZED_MESON_DEFINED_IN": "$ZED_FILE",
      "ZED_MESON_TARGET": "$ZED_CUSTOM_meson_target"
    },
    "cwd": "$ZED_WORKTREE_ROOT",
    "save": "all",
    "tags": ["meson-executable"]
  },
  {
    "label": "Meson: build and run $ZED_CUSTOM_meson_target",
    "command": "meson compile -C out $ZED_CUSTOM_meson_target && meson devenv -C out \"./$ZED_RELATIVE_DIR/\"$ZED_CUSTOM_meson_target",
    "cwd": "$ZED_WORKTREE_ROOT",
    "save": "all",
    "tags": ["meson-executable"]
  }
]
```

</details>

#### Automatic Executable Discovery

To get a specific executable to show up in the task picker, first run its build/run/debug action from the inline runnable indicator. Zed then keeps the resolved target task in its recent-task history, making it available in the task picker for the rest of the current project session.

You need to do this each time after reopening the project.

## Language Server

Both Muon's LSP mode and MesonLSP are supported by this extension as separate language servers (`muon` and `mesonlsp`). Both support automatic installation. The extension first checks if the server binary is available in your `PATH`. If not, it downloads the matching version for your architecture and operating system.

> [!NOTE]  
> Muon has a [tracking issue regarding its LSP feature set](https://github.com/muon-build/muon/issues/159). Since MesonLSP has been archived and is no longer being developed, it is important that Muon's LSP functionality gets improved. Please contribute to Muon if you find it useful and would like a future-proof, actively developed and fully featured LSP for Meson (not just in Zed)!

You can choose which Meson language servers run through Zed's built-in language settings.

## Configuration

To choose which Meson language server(s) are used, configure Zed's `languages` settings in your `settings.json`:

```json
{
  "languages": {
    "Meson": {
      "language_servers": ["mesonlsp", "!muon"]
    }
  }
}
```

In `language_servers`, use `!` to disable a server. The current recommended configuration is to use only `mesonlsp` as shown above, since Muon's LSP mode is still in early development and lacks many features. However, you can also choose to use both servers at the same time.

## Formatting with Muon

Muon supports formatting code in LSP mode, but currently it is not possible to configure the format when running in LSP mode.

If you want to change how code is formatted, you need to manually change the [Zed formatter settings](https://zed.dev/docs/configuring-zed#formatter) for Meson to a formatter that you prefer.

If the custom formatter configuration changes the tab size, you should also specify this using the [Zed tab size setting](https://zed.dev/docs/configuring-zed#tab-size).

## Acknowledgments

This extension was created by taking parts of the now unmaintained [zed-meson](https://github.com/trshbn/zed-meson) extension.
