{ lib, buildInputs ? [] }:

lib.package.mkDerivation {
  pname = "tomodoro";
  version = "0.1.0";
  src = ./.;

  buildInputs = buildInputs ++ [
    # Add your build dependencies here
  ];

  meta = with lib; {
    description = "Your package description";
    license = licenses.mit;
  };
}

