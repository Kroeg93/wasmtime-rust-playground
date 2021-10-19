export WASI_VERSION=12
export WASI_VERSION_FULL=${WASI_VERSION}.0
export WASI_SDK_PATH=/home/manji/AUR/wasi-sdk-${WASI_VERSION_FULL}

CC="${WASI_SDK_PATH}/bin/clang"

#  --sysroot=${WASI_SDK_PATH}/share/wasi-sysroot