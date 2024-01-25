{ pkgs ? import <nixpkgs> { } }:
pkgs.mkShell
{
  nativeBuildInputs = with pkgs; [
    pkg-config
    alsa-lib.dev
  ];
  shellHook = ''
  echo "Hello, wellcome to your custom development shell"
  export PKG_CONFIG_PATH=${pkgs.alsa-lib.dev}/lib/pkgconfig/
  export TOMODORO_PATH=$(pwd)
  '';
}
