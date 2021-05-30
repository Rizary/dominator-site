{ pkgs, dominator }:
let inherit (pkgs) stdenv lib;
in
stdenv.mkDerivation {
  name = "${package.name}-${package.version}";

  src = lib.cleanSourceWith {
    filter = name: type:
      !(lib.hasSuffix ".log" name) &&
      !(lib.hasSuffix ".nix" name) &&
      !(lib.hasSuffix "node_modules" name)
    ;
    src = ../../.;
  };

  buildInputs = [ pkgs.nodejs-14_x dominator.yarnPkg pkgs.yarn dominator.nix.rust-overlay pkgs.openssl pkgs.zlib pkgs.cacert ];

  patchPhase = ''
    ln -s ${dominator.yarnPkg}/libexec/${package.name}/node_modules .
  '';

  buildPhase = ''
    # Yarn writes cache directories etc to $HOME.
    export HOME=$PWD/yarn_home
    export PATH=$PWD/node_modules/.bin:$PATH
    node node_modules/wasm-pack/install.js
    ls -lahg src-css/
    yarn --enable-pnp --offline build
  '';

  installPhase = ''
    mkdir -p $out/js
    cp -r devhtml/. $out/
    cp -r dist/css/. $out/css
    cp -r dist/js/. $out/js
  '';

  shellHook = ''
    rm -rf node_modules
    ln -sv ${dominator.yarnPkg}/libexec/${package.name}/node_modules .
    export PATH=$PWD/node_modules/.bin:$PATH
  '';
}
