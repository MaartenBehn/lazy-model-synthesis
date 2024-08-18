#!/bin/sh

cd ./shaders || cd ../shaders || true
find ./ -name *.spv -type f -exec rm {} \;
find ./ -not -name *.spv -type f -exec glslangValidator -l --target-env spirv1.4 -V -o {}.spv {} \;
