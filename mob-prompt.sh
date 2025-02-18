#!/bin/bash

#UID=$(getuid)
GID=$(id -g)
USERNAME=$(id -un)
GROUPNAME=$(id -gn)

set -ex

docker run --rm --env=ENTRYPOINT_VERBOSE=1 --volume=$(pwd):/tmp/mobilenode --workdir=/tmp/mobilenode --env=EXTERNAL_UID=$UID --env=EXTERNAL_GID=$GID --env=EXTERNAL_USER=$USERNAME --env=EXTERNAL_GROUP=$GROUPNAME --env=CARGO_TARGET_DIR=/tmp/mobilenode/target/docker --env=MC_CHAIN_ID=local --env=TEST_DATABASE_URL=postgres://localhost --env=RUST_BACKTRACE=1 --env=CARGO_BUILD_JOBS=4 --env=SGX_MODE=HW --env=IAS_MODE=PROD --cap-add=SYS_PTRACE -it --env=SSH_AUTH_SOCK --volume=/run/user/$UID/keyring/ssh:/run/user/$UID/keyring/ssh --volume=$HOME/.ssh:/var/tmp/user/.ssh mobilecoin/builder-install:v0.0.36 /bin/bash
