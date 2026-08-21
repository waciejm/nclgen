# `nclgen` - resource generation with Nickel

## How it works

`nclgen` generates resources in directories relative to the project root,
which is a directory that contains a `ncl.gen` subdir.

`nclgen generate` generates outputs based on `targets` definitions inside the `ncl.gen/config.toml` file.

`nclgen check` checks that generated outputs are up-to-date.

`nclgen imports` prints the import paths that `nclgen` adds to the evaluation.

## Example

See `example` directory for a quick look at how a `nclgen` project works.

## Output format

`nclgen` requires that the Nickel evaluation to produce a record of strings:

- The fields of the record must be file paths relative to project root
  and can't contain any `/.` or `/..` components.

- The string values become the content of the generated files.

By default `nclgen` expects that the whole evaluation result is the outputs record.
Use the `targets.<target>.outputs_field` config option to set which field contains the outputs record.

## Config schema

See `src/project/config.rs` for the full `ncl.gen/config.toml` schema.

## Nickel environment

### Nickel binary

By default, `nclgen` will use the `nickel` binary from the environment.
You can override the binary used using the `NCLGEN_NICKEL_PATH` env var.

### Nickel Language Server

In order for Nickel Language Server to resolve imports that will be available during `nclgen`
evaluation, you can add the output of `nclgen imports` to the `NICKEL_IMPORT_PATH` env var.

```bash
readonly NCLGEN_IMPORTS="$(nclgen imports)"
if [[ -n "$NCLGEN_IMPORTS" ]]; then
  if [[ -z "$NICKEL_IMPORT_PATH" ]]; then
    export NICKEL_IMPORT_PATH="$NCLGEN_IMPORTS"
  else
    export NICKEL_IMPORT_PATH="$NCLGEN_IMPORTS:$NICKEL_IMPORT_PATH"
  fi
fi
```
