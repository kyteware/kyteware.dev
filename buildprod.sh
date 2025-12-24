rm -rf build/*;
mkdir -p build;

cd gumballs;
./build-for-debug.sh
cd ..;

cd kyteware-home;
npm run build;
cd ..;

cp -r kyteware-home/dist build;
gzip -9 build/dist/assets/*.wasm
