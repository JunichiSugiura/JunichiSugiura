#!/bin/sh 
 
set -e 

###########################################################
# Utils
###########################################################
log() {
    message=$1
    echo 📌 "$message"
}

is_file() {
    path="$1"
    [ -f "$path" ]
}

is_dir() {
    path="$1"
    [ -d "$path" ]
}

###########################################################
# Neovim
###########################################################
plug_path=~/.local/share/nvim/site/autoload/plug.vim
if ! is_file "$plug_path"; then
    log 'Setup vim-plug"'
    sh -c "curl -fLo $plug_path --create-dirs https://raw.githubusercontent.com/junegunn/vim-plug/master/plug.vim"
fi

plugged_path=~/.local/share/nvim/autoload/plugged
if ! is_dir "$plugged_path"; then
    log 'Install Neovim Plugins'
    nvim +PlugInstall +qall
fi

###########################################################
# GitHub CLI Login
###########################################################
log "Check GitHub auth status"
if ! gh auth status; then
    log 'Login with GitHub CLI'
    gh auth login
fi

###########################################################
# Git GPG Signing Key
###########################################################
if ! is_dir ~/.gnupg || [ -z "$(gpg --list-secret-keys --keyid-format LONG)" ]; then
    log 'Generate GPG key'
    gpg --default-new-key-algo rsa4096 --gen-key
    key_id=$(gpg --list-secret-keys --keyid-format LONG | ggrep -oP "rsa4096\/[0-9a-fA-F]{16}" | cut -d"/"  -f2)

    log 'Setup Git with GPG key'
    git config --global user.signingkey "$key_id"

    log 'Upload GPG public key to GitHub'
    gpg --armor --export "$key_id" | gh gpg-key add -
fi

###########################################################
# asdf
###########################################################
for plugin in $(awk '{print $1}' ~/.tool-versions); do
    if ! is_dir ~/.asdf/plugins/"$plugin"; then
        asdf plugin add "$plugin"
    fi
done

is_runtime_versions_changed () {
    plugin="$1"
    specified=$(grep "$plugin" ~/.tool-versions | awk '{$1=""; print $0}')
    installed=$(asdf list "$plugin" 2>&1)

    is_changed=
    for version in $specified; do
        match=$(echo "$installed" | grep "$version")
        [ -z "$match" ] && is_changed=1
    done

    [ "$is_changed" ]
}

for plugin in $(asdf plugin list); do
    if is_runtime_versions_changed "$plugin"; then
        log "Install runtime: $plugin"
        asdf install "$plugin"
    fi
done

###########################################################
# wasm-pack
###########################################################
if ! is_dir ~/.cargo/bin/wasm-pack; then
    log 'Setup wasm-pack'
    curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
fi

###########################################################
# starkli
###########################################################
if ! is_dir ~/.starkli; then
    log 'Setup starkli'
    curl https://get.starkli.sh | sh
    ~/.starkli/bin/starkliup
fi
