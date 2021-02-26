#!/usr/bin/env bash
set -e

ANDROID_HOME=~/Library/Android/sdk
ANDROID_NDK_HOME=${ANDROID_HOME}/ndk-bundle
export ANDROID_NDK_SYSROOT=${ANDROID_NDK_HOME}/toolchains/llvm/prebuilt/darwin-x86_64/sysroot

export ORT_STRATEGY=system
export ORT_LIB_LOCATION=~/dev/onnxruntime/

min_ver=22

#cargo ndk --target aarch64-linux-android    --platform ${min_ver} -- build --package onnxruntime-sys --features generate-bindings
#cargo ndk --target armv7-linux-androideabi  --platform ${min_ver} -- build --package onnxruntime-sys --features generate-bindings
#cargo ndk --target i686-linux-android       --platform ${min_ver} -- build --package onnxruntime-sys --features generate-bindings
#cargo ndk --target x86_64-linux-android     --platform ${min_ver} -- build --package onnxruntime-sys --features generate-bindings

cargo build --features generate-bindings
