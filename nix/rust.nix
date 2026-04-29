{
  inputs,
  flake-parts-lib,
  ...
}:
{
  options.perSystem = flake-parts-lib.mkPerSystemOption (
    {
      lib,
      pkgs,
      self',
      ...
    }:
    {
      options.rust = {
        craneLib = lib.mkOption { readOnly = true; };
        rustfmt = lib.mkOption { readOnly = true; };
      };

      config =
        let
          fenixToolchain = (import inputs.fenix { inherit pkgs; }).stable;

          rustToolchain = fenixToolchain.withComponents [
            "rustc"
            "cargo"
            "rustfmt"
            "clippy"
          ];

          craneLib = (inputs.crane.mkLib pkgs).overrideToolchain rustToolchain;

          src = craneLib.cleanCargoSource ../.;

          commonArgs = {
            inherit src;
            strictDeps = true;
          };

          cargoArtifacts = craneLib.buildDepsOnly commonArgs;

          nclgen = craneLib.buildPackage (
            commonArgs
            // {
              inherit cargoArtifacts;
            }
          );

          nclgen-docs = craneLib.cargoDoc (
            commonArgs
            // {
              inherit cargoArtifacts;
              env.RUSTDOCFLAGS = "--deny warnings";
            }
          );

          nclgen-clippy = craneLib.cargoClippy (
            commonArgs
            // {
              inherit cargoArtifacts;
              cargoClippyExtraArgs = "--all-targets -- --deny warnings";
            }
          );
        in
        {
          rust = {
            craneLib = craneLib;
            rustfmt = fenixToolchain.rustfmt;
          };

          packages = {
            inherit
              nclgen
              ;
          };

          checks = {
            inherit
              nclgen
              nclgen-docs
              nclgen-clippy
              ;
          };

          devShells.rust = craneLib.devShell {
            checks = self'.checks;
            packages = builtins.attrValues {
              inherit (pkgs)
                cargo-watch
                rust-analyzer
                ;
            };
          };
        };
    }
  );
}
