# nclgen - resource generation with Nickel

## How it works

`nclgen` generates resources in directories relative to the `project root`,
which is a directory that contains a `ncl.gen` subdir.

`nclgen generate` generates outputs based on `targets` definitions inside the `ncl.gen/config.toml` file.

`nclgen check` checks that generated outputs are up-to-date.

`nclgen imports` prints the import paths that `nclgen` adds to the evaluation.

## Example

See `example` directory for a quick look at how a `nclgen` project works.

## Config schema

See `src/project/config.rs` for the full `ncl.gen/config.toml` schema.

## Nickel environment

### Nickel binary

By default, `nclgen` will use the `nickel` binary from the environment.
You can override the binary used using the `NCLGEN_NICKEL_PATH` env var.

### Nickel Language Server

In order for Nickel Language Server to resolve imports that will be available during `nclgen`
evaluation, you can set the `NICKEL_IMPORT_PATH` env var to the output of `nclgen imports`.

```bash
export NICKEL_IMPORT_PATH="$(nclgen imports)"
```

If you are already using other import dirs via `NICKEL_IMPORT_PATH`, extend it
by joining the output of `nclgen imports` and `NICKEL_IMPORT_PATH` with `:`.

```bash
export NICKEL_IMPORT_PATH="$(nclgen imports):$NICKEL_IMPORT_PATH"
```
