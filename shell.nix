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
  export TOMODORO_PATH=$(pwd)
  '';
}
