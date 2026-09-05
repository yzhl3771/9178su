#!/bin/sh
set -eu

GKI_ROOT=$(pwd)

display_usage() {
    echo "Usage: $0 [--cleanup | <commit-or-tag>]"
    echo "  --cleanup:              Cleans up previous modifications made by the script."
    echo "  <commit-or-tag>:        Sets up or updates the 9178su to specified tag or commit."
    echo "  -h, --help:             Displays this usage information."
    echo "  (no args):              Sets up or updates the 9178su environment to the latest tagged version."
}

initialize_variables() {
    if test -d "$GKI_ROOT/common/drivers"; then
         DRIVER_DIR="$GKI_ROOT/common/drivers"
    elif test -d "$GKI_ROOT/drivers"; then
         DRIVER_DIR="$GKI_ROOT/drivers"
    else
         echo '[ERROR] "drivers/" directory not found.'
         exit 127
    fi

    DRIVER_MAKEFILE=$DRIVER_DIR/Makefile
    DRIVER_KCONFIG=$DRIVER_DIR/Kconfig
}

# Reverts modifications made by this script
perform_cleanup() {
    echo "[+] Cleaning up..."
    [ -L "$DRIVER_DIR/9178su" ] && rm "$DRIVER_DIR/9178su" && echo "[-] Symlink removed."
    grep -q "9178su" "$DRIVER_MAKEFILE" && sed -i '/9178su/d' "$DRIVER_MAKEFILE" && echo "[-] Makefile reverted."
    grep -q "drivers/9178su/Kconfig" "$DRIVER_KCONFIG" && sed -i '/drivers\/9178su\/Kconfig/d' "$DRIVER_KCONFIG" && echo "[-] Kconfig reverted."
    if [ -d "$GKI_ROOT/9178su" ]; then
        rm -rf "$GKI_ROOT/9178su" && echo "[-] 9178su directory deleted."
    fi
}

# Sets up or update KernelSU environment
setup_kernelsu() {
    echo "[+] Setting up 9178su..."
    test -d "$GKI_ROOT/9178su" || git clone https://github.com/9178su/9178su.git 9178su && echo "[+] Repository cloned."
    cd "$GKI_ROOT/9178su"
    git stash && echo "[-] Stashed current changes."
    if [ "$(git status | grep -Po 'v\d+(\.\d+)*' | head -n1)" ]; then
        git checkout main && echo "[-] Switched to main branch."
    fi
    git pull && echo "[+] Repository updated."
    if [ -z "${1-}" ]; then
        git checkout "$(git describe --abbrev=0 --tags)" && echo "[-] Checked out latest tag."
    else
        git checkout "$1" && echo "[-] Checked out $1." || echo "[-] Checkout default branch"
    fi
    cd "$DRIVER_DIR"
    ln -sf "$(realpath --relative-to="$DRIVER_DIR" "$GKI_ROOT/9178su/kernel")" "9178su" && echo "[+] Symlink created."

    # Add entries in Makefile and Kconfig if not already existing
    grep -q "9178su" "$DRIVER_MAKEFILE" || printf "\nobj-\$(CONFIG_N9178SU) += 9178su/\n" >> "$DRIVER_MAKEFILE" && echo "[+] Modified Makefile."
    grep -q "source \"drivers/9178su/Kconfig\"" "$DRIVER_KCONFIG" || sed -i "/endmenu/i\source \"drivers/9178su/Kconfig\"" "$DRIVER_KCONFIG" && echo "[+] Modified Kconfig."
    echo '[+] Done.'
}

# Process command-line arguments
if [ "$#" -eq 0 ]; then
    initialize_variables
    setup_kernelsu
elif [ "$1" = "-h" ] || [ "$1" = "--help" ]; then
    display_usage
elif [ "$1" = "--cleanup" ]; then
    initialize_variables
    perform_cleanup
else
    initialize_variables
    setup_kernelsu "$@"
fi
