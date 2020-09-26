#!/usr/bin/bash

# Cleans up build files.

cd ./cli/fshuf/;
rm -rf ./target/;
cd ../../;

cd ./cli/jsgrep/;
rm -rf ./node_modules/;
rm ./index.js ./index.bundle.js ./index.min.js ./jsgrep;
cd ../../;

cd ./cli/round-time-up/;
rm -rf ./node_modules/;
rm ./round-time-up;
cd ../../;

cd ./cli/openweatherbar/;
rm -rf ./target/;
cd ../../;

