#!/bin/bash

export BUNDLE="user-color-editor.flatpak"
export APP_ID="com.system76.UserColorEditor"
export MANIFEST_PATH="build-aux/${APP_ID}.Devel.json"
export FLATPAK_MODULE="user-color-editor"
export RUNTIME_REPO="https://nightly.gnome.org/gnome-nightly.flatpakrepo"

sudo flatpak-builder --keep-build-dirs --user --disable-rofiles-fuse flatpak_app --repo=repo ${BRANCH:+--default-branch=$BRANCH} ${MANIFEST_PATH} --force-clean --install --delete-build-dirs

sudo rm -rf .flatpak-builder/
sudo rm -rf repo/
sudo rm -rf flatpak_app/

