{ pkgs }:
let inherit (pkgs) lib; package = lib.importJSON ../../package.json;
in
pkgs.mkYarnPackage rec {
  pname = package.name;
  version = package.version;
  src = null;
  dontUnpack = true;
  packageJSON = ../../package.json;
  yarnLock = ../../yarn.lock;

  preConfigure = ''
    mkdir ${package.name}
    cd ${package.name}
    ln -s ${packageJSON} ./package.json
    ln -s ${yarnLock} ./yarn.lock
  '';

  yarnPreBuild = ''
    mkdir -p $HOME/.node-gyp/${pkgs.nodejs.version}
    echo 9 > $HOME/.node-gyp/${pkgs.nodejs.version}/installVersion
    ln -sfv ${pkgs.nodejs}/include $HOME/.node-gyp/${pkgs.nodejs.version}
  '';

  pkgConfig = { };

  publishBinsFor = [
    "rollup"
    "postcss"
    "postcss-cli"
    "purgecss"
    "@aws-amplify/cli"
    "amplify-util-mock"

  ];
}
