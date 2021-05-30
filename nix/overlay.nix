final: prev:
{
  dominator = rec {
    nix = prev.callPackage ./nix { };
  };
}
