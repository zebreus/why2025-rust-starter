#!/usr/bin/env bash
set -ex

CRATE_NAME=${CRATE_NAME:=$(cat Cargo.toml | grep 'name =' | grep -o '".*"' | sed 's/"//g')}
if test -z "$CRATE_NAME" ; then
    echo "Failed to find the crate name in Cargo.toml"
    exit 1
fi
PROJECT_NAME=${PROJECT_NAME:=$(cat metadata.json | grep '"unique_identifier": ' | grep -o ': ".*"' | sed 's/[" :]//g')}
if test -z "$PROJECT_NAME" ; then
    echo "Please set PROJECT_NAME to the app slug of your project"
    echo "If you don't yet have a project, create one at https://badge.why2025.org/page/create-project"
    exit 1
fi
if test -z "$BADGEHUB_API_TOKEN" ; then
    echo "Please set BADGEHUB_API_TOKEN to an API token for your project"
    echo "You can get one from https://badge.why2025.org/page/project/${PROJECT_NAME}/edit"
    exit 1
fi

cargo build --release

LOCAL_ELF_NAME=target/riscv32imafc-unknown-none-elf/release/${CRATE_NAME}
REMOTE_ELF_NAME=main.elf

curl -X POST -H "badgehub-api-token: ${BADGEHUB_API_TOKEN}" -F "file=@./${LOCAL_ELF_NAME}" https://badge.why2025.org/api/v3/projects/${PROJECT_NAME}/draft/files/${REMOTE_ELF_NAME} \
&& echo "Uploaded main.elf to a draft"

curl -X POST -H "badgehub-api-token: ${BADGEHUB_API_TOKEN}" -F "file=@./metadata.json" https://badge.why2025.org/api/v3/projects/${PROJECT_NAME}/draft/files/metadata.json \
&& echo "Uploaded metadata.json to a draft"

curl -X 'PATCH' -H "badgehub-api-token: ${BADGEHUB_API_TOKEN}" \
  "https://badge.why2025.org/api/v3/projects/${PROJECT_NAME}/publish" \
  -H 'accept: application/json' \
  -H 'Content-Type: application/json' \
&& echo "Published project ${PROJECT_NAME}"
