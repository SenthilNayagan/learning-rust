# learning-rust
Learning Rust from basics to advance.

## Root Cargo.toml
**Why no [package]? on root `Cargo.toml`:** The root `Cargo.toml` is not a crate itself. It is just a workspace definition file that tells Cargo:

- which crates belong to this workspace
- which resolver to use

It has no code, nothing to compile, nothing to publish. Adding [package] would tell Cargo "this root is also a crate" which we don't want here.

**Why no [dependencies]:** 
Each crate manages its own dependencies in its own `Cargo.toml`. However, if we later find ourself adding the same dependency across multiple crates (e.g., serde in basics, intermediate, and advanced), Rust workspaces support a [workspace.dependencies] section to define versions once and share them:

```toml
[workspace]
members = [...]
resolver = "2"

# Optional — define shared dependency versions in one place
[workspace.dependencies]
serde = { version = "1", features = ["derive"] }
tokio = { version = "1", features = ["full"] }
```

Then in each individual crate's Cargo.toml you just reference it without a version:

```toml
[dependencies]
serde = { workspace = true }   # version inherited from root
```

### Resolver in the root Cargo.toml
The **resolver in the root Cargo.toml** sets the dependency resolution strategy for the entire workspace. Since it's in the root, it applies to all crates in the workspace uniformly. We only need to declare it once here rather than in every individual crate's `Cargo.toml`.

`resolver = "1"` — old behavior (pre-2021)
The problem it had was feature unification — if two crates in your workspace both depended on the same external crate but needed different features, Cargo would blindly merge all features together globally:

```text
basics      → depends on tokio with feature ["rt"]
advanced    → depends on tokio with feature ["full"]

Result with resolver "1": tokio gets ALL features everywhere
             even in basics where you only wanted ["rt"]
```

This caused subtle bugs and unexpected behavior especially with:

- `std` vs `no_std` crates
- Dev dependencies leaking into normal builds
- Build script dependencies polluting regular dependencies

`resolver = "2"` — new behavior (2021 edition default)
Resolves features **per context** independently:

- Dev dependencies ([dev-dependencies]) don't leak into normal builds
- Build script dependencies ([build-dependencies]) are kept separate
- Each crate gets only the features it actually asked for

```text
basics      → tokio gets ["rt"] only
advanced    → tokio gets ["full"] only
             each crate gets exactly what it declared, nothing more
```

### Why we must declare it explicitly in a workspace root
For a single crate using `edition = "2021"`, resolver "2" is automatically the default — we don't need to write it.

But a workspace doesn't automatically inherit it. Without explicitly declaring it in the root Cargo.toml, Cargo falls back to resolver "1" for the whole workspace regardless of what edition each individual crate uses. So we always declare it explicitly at the workspace level:

```toml
[workspace]
members = [...]
resolver = "2"    ← without this line, the whole workspace uses resolver "1"
```

## How run with `cargo run` command
`cargo run` command does two things in sequence:
1. **Compiles** the target (equivalent to cargo build)
2. **Executes** the compiled binary immediately after

We run our exercises/examples using `-p` or `--package` flag which tells Cargo which crate in the workspace to target.

Without `-p`, Cargo looks at our current directory. Since we're running from the workspace root (`learning-rust/`), Cargo sees multiple crates and gets confused:

```bash
cargo run --example variables
# ✗ Error: ambiguous, which package? basics? intermediate? advanced?
```

With `-p basics`, Cargo knows exactly where to look:
```
learning-rust/exercises/basics/   ← go here
```

It matches against the **name** field in that crate's `Cargo.toml`:

```toml
[package]
name = "basics"    ← this is what -p basics matches against
```

---

### `--example variables`

This tells Cargo **which file inside the `examples/` folder** to compile and run.

Without `--example`, Cargo looks for `src/main.rs` as the default entry point. But your crate has no `src/main.rs` — all your exercises live in `examples/`. So you must tell Cargo explicitly which example to run.

`--example variables` maps directly to the filename:
```text
exercises/basics/examples/variables.rs
                           ↑
                           --example variables
```

The name is just the filename without the `.rs` extension.

### What Cargo does end to end
```bash
# pattern
cargo run -p <crate-name> --example <filename-without-rs>

# Example
cargo run -p basics --example variables
```

Step by step internally:
```
1. Read root Cargo.toml          → finds all workspace members
2. -p basics                     → locate exercises/basics/Cargo.toml
3. --example variables           → locate exercises/basics/examples/variables.rs
4. Compile variables.rs          → produces binary at
                                   target/debug/examples/variables.exe
5. Execute variables.exe         → prints output to terminal
```

---

### Where the compiled binary goes

After running, Cargo stores the compiled binary here:

```
learning-rust/
└── target/
    └── debug/
        └── examples/
            └── variables.exe    ← compiled output
```

All crates in the workspace share the same **target/** folder at the root — this is one of the big benefits of a workspace, it avoids recompiling shared dependencies multiple times.

The only exception in our repo is the **playground** crate (for proc macros) which has a `src/main.rs` — that one runs simply as:

```bash
cargo run -p playground   # no --example needed
```

## Others
### Tell Git who we are before pushing our changes into remote repo
Run these two commands:

```bash
git config --global user.name "Your Name"
git config --global user.email "your-email@example.com"
```

Here, `--global` means this configuration applies to all Git repos on our machine, not just `learning-rust`. We only ever need to do this once per machine. Git stores it in `C:\Users\<your-username>\.gitconfig`, in case of Windows.

### Git needs a credential helper (Non personal access tokens)
Git needs a credential helper to store and retrieve your authentication details when pushing to a remote repository (for example on GitHub, GitLab, or Bitbucket).

In Visual Studio Code on Windows, Git asks us where it should store our credentials so we don't have to enter them every time.

Choose the recommended option, **manager** which refers to Git Credential Manager Core. 

It:
- securely stores credentials in **Windows Credential Manager**
- supports OAuth login
- works well with GitHub / Azure DevOps / GitLab
- integrates smoothly with VS Code

**What happens internally:** Git will add this setting:

```bash
git config --global credential.helper manager
```

We can verify later:

```bash
git config --global --list
```

We can view the **Windows Credential Manager** at:

```text
Control Panel → Credential Manager → Windows Credentials
```