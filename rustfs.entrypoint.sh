#!/bin/sh
set -Eeu

function random_string() {
    local length="${1:-64}"
    openssl rand ${length} | base64 -w0 | tr '+/' '-_' | tr -d '='
}

USER_NAME="hcwnd"
EVENT_IMAGE_BUCKET="event-image"
USER_IMAGE_BUCKET="user-image"
ARTIST_IMAGE_BUCKET="artist-image"
ACCESS_KEY=$(cat /run/secrets/rustfs_access_key)
SECRET_KEY=$(cat /run/secrets/rustfs_secret_key)

rc alias set local http://rustfs:9000 "${ACCESS_KEY}" "${SECRET_KEY}"
rc admin user list local/
rc ls local/

rc mb "local/${EVENT_IMAGE_BUCKET}"
rc ilm rule import "local/${EVENT_IMAGE_BUCKET}" /storage/event-image/lifecycle.json
rc ilm rule list "local/${EVENT_IMAGE_BUCKET}"

rc mb "local/${USER_IMAGE_BUCKET}"
rc ilm rule import "local/${USER_IMAGE_BUCKET}" /storage/user-image/lifecycle.json
rc ilm rule list "local/${USER_IMAGE_BUCKET}"

rc mb "local/${ARTIST_IMAGE_BUCKET}"
rc ilm rule import "local/${ARTIST_IMAGE_BUCKET}" /storage/artist-image/lifecycle.json
rc ilm rule list "local/${ARTIST_IMAGE_BUCKET}"

PASSWORD=$(random_string 64)
rc admin user add local/ "${USER_NAME}" "${PASSWORD}"
rc admin policy attach local/ readwrite --user "${USER_NAME}"

# Remove all service accounts
echo "Removing old service accounts..."
rc admin service-account list local/ --user "${USER_NAME}" --json | jq -r '.accounts[].accessKey' | while read -r key; do
    rc admin service-account rm local/ "${key}"
done

IMAGE_PROXY_ACCESS_KEY="imgproxy-$(random_string 30)"
IMAGE_PROXY_SECRET=$(random_string 30)
rc admin service-account create local/ "${IMAGE_PROXY_ACCESS_KEY}" "${IMAGE_PROXY_SECRET}" --name image-proxy --user "${USER_NAME}" --quiet
mkdir -p /secrets/image-proxy/.aws
cat<<EOF >/secrets/image-proxy/.aws/credentials
[default]
aws_access_key_id = "${IMAGE_PROXY_ACCESS_KEY}"
aws_secret_access_key = "${IMAGE_PROXY_SECRET}"
EOF

APP_ACCESS_KEY="app-$(random_string 30)"
APP_SECRET=$(random_string 30)
rc admin service-account create local/ "${APP_ACCESS_KEY}" "${APP_SECRET}" --name "app" --user "${USER_NAME}" --quiet
cat<<EOF >/secrets/app/credentials.toml
[s3]
access_key = "${APP_ACCESS_KEY}"
secret_key = "${APP_SECRET}"
EOF

rc admin service-account list local/ --user "${USER_NAME}"
