{
  self,
  ...
}:
{
  perSystem =
    {
      config,
      pkgs,
      self',
      ...
    }:
    {
      checks."nclgen-check" =
        pkgs.runCommand "nclgen-check"
          {
            nativeBuildInputs = [
              self'.packages.nclgen
              pkgs.nickel
            ];
          }
          ''
            nclgen check -p "${self}/example"
            touch $out
          '';
    };
}
