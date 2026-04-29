{ inputs, ... }:
{
  imports = [
    inputs.treefmt-nix.flakeModule
  ];
  perSystem =
    {
      config,
      pkgs,
      self',
      ...
    }:
    {
      treefmt = {
        projectRootFile = "flake.nix";
        settings.on-unmatched = "info";

        programs.keep-sorted.enable = true;

        programs.mdformat.enable = true;

        programs.nickel.enable = true;

        programs.nixfmt.enable = true;

        programs.rustfmt = {
          enable = true;
          package = config.rust.rustfmt;
        };

        programs.taplo.enable = true;
      };
    };
}
