#!/bin/sh 
 
set -e 

# GITHUB_PATH=~/projects/github.com
# GIT_CLONE_PATH="$GITHUB_PATH/JunichiSugiura"


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

ensure_dir() {
    path="$1"
    if ! is_dir "$path"; then
        mkdir -p "$path"
    fi
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
        if [ "$plugin" = nodejs ]; then log "Import release team keyring for Node.JS"
            bash -c '${ASDF_DATA_DIR:=$HOME/.asdf}/plugins/nodejs/bin/import-release-team-keyring'
        fi

        log "Install runtime: $plugin"
        asdf install "$plugin"
    fi
done

system_node_path=/usr/local/bin/node
if ! is_file "$system_node_path"; then
    log "Create symlink to $system_node_path so that XCode can reference"
    ln -s ~/.asdf/shims/node "$system_node_path"
fi

###########################################################
# Yarn global
###########################################################
if ! is_dir ~/.config/yarn/global/node_modules; then
    log 'Setup Yarn global'
    yarn global add
fi

if ! is_file ~/prysm/prysm.sh; then
    log 'Setup Ethereum 2.0 client'
    if ! is_dir ~/prysm; then
        mkdir ~/prysm
    fi

    (
        cd ~/prysm
        curl https://raw.githubusercontent.com/prysmaticlabs/prysm/master/prysm.sh --output prysm.sh
        chmod +x prysm.sh
    )
fi
