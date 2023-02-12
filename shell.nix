{ pkgs ? import <nixpkgs> { } }:
with pkgs;
mkShell{
    buildInputs = [
        clang_12
        swagger-codegen3
    ];
}
