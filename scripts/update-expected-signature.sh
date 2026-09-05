#!/bin/sh
# Update kernel/Kbuild with the certificate hash/size of your own Manager keystore.
#
# Usage:
#   keytool -exportcert -alias <alias> -keystore <keystore> -storepass <pass> -file cert.der
#   scripts/update-expected-signature.sh cert.der
set -eu

DER=${1:-cert.der}
test -f "$DER" || { echo "usage: $0 <certificate.der>" >&2; exit 2; }

SIZE_HEX=$(printf '0x%04x' "$(stat -c%s "$DER")")
HASH=$(sha256sum "$DER" | awk '{print $1}')

KBUILD=$(dirname "$0")/../kernel/Kbuild

sed -i "s/^N9178SU_EXPECTED_SIZE := .*/N9178SU_EXPECTED_SIZE := $SIZE_HEX/" "$KBUILD"
sed -i "s/^N9178SU_EXPECTED_HASH := .*/N9178SU_EXPECTED_HASH := $HASH/" "$KBUILD"

echo "Updated $KBUILD"
echo "  N9178SU_EXPECTED_SIZE=$SIZE_HEX"
echo "  N9178SU_EXPECTED_HASH=$HASH"
