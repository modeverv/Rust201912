#! /bin/bash
rm -frv RustVolume.vst
cargo build --release
./osx_vst_bundler.sh RustVolume target/release/librust_volume.dylib
cp -Rfv RustVolume.vst /Volumes/MySD/DAW/VST/