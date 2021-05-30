{ pkgs }:

with pkgs;

# Configure your development environment.
#
# Documentation: https://github.com/numtide/devshell
devshell.mkShell {
  name = "Dominator";
  motd = ''
    Welcome to Dominator unofficial website development environment
  '';
  commands = [
  ];

  bash = {
    extra = ''
      export LD_INCLUDE_PATH="$DEVSHELL_DIR/include"
      export LD_LIB_PATH="$DEVSHELL_DIR/lib"
    '';
    interactive = '''';
  };

  env = [
    { name = "OPENSSL_DIR"; value = "${openssl.bin}/bin"; }
    { name = "OPENSSL_LIB_DIR"; value = "${openssl.out}/lib"; }
    { name = "OPENSSL_INCLUDE_DIR"; value = "${openssl.out.dev}/include"; }
  ];

  packages = [
    # build tools
    ## Rust
    dominator.nix.rust-overlay
    dominator.nix.yarn-package

    ### Others
    binutils
    pkgconfig
    openssl
    openssl.dev
    gcc
    glibc
    gmp.dev
    nixpkgs-fmt

    # Javascript related frontend
    # It is also used for Rust's frontend development
    wasm-pack
    nodejs-14_x
    nodePackages.node2nix
    yarn
    yarn2nix
  ];
}
