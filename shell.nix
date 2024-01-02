{ pkgs ? import <nixpkgs> { } }:
pkgs.mkShell
{
  nativBuildInputs = with pkgs; [
    hello
    pkg-config
    alsa-lib.dev
  ];
  shellHook = ''
  echo "Hello, wellcome to your custom development shell"
  export PKG_CONFIG_PATH=${pkgs.alsa-lib.dev}/lib/pkgconfig/
  export PATH=${pkgs.pkg-config}/bin:$PATH
  export TOMODORO_PATH=$(pwd)
  '';
}
