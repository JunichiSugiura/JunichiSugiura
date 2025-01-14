##################################################
# sh
##################################################

$env.PATH = ($'/bin:/usr/bin:($env.PATH)')

##################################################
# Homebrew
##################################################

$env.PATH = ($'/opt/homebrew/bin:($env.PATH)')

##################################################
# Cargo
##################################################

# List doesn't seem to work.
$env.PATH = ($'($env.HOME)/.cargo/bin:($env.PATH)')

##################################################
# Homebrew Ruby
##################################################

$env.PATH = ($'/opt/homebrew/opt/ruby/bin:/opt/homebrew/lib/ruby/gems/3.0.0/bin:($env.PATH)')

###################################################
# asdf
###################################################

# $env.PATH = ($'($env.HOME)/.asdf/shims/:($env.PATH)')

##################################################
# dip
##################################################

$env.PATH = ($'($env.HOME)/Library/Application Support/dip/bundle/shims:($env.PATH)')

##################################################
# GPG
##################################################

$env.GPG_TTY = (tty)

##################################################
# Docker
##################################################

if ('/Applications/Docker.app/Contents/Resources/bin' | path exists) == true {
    $env.PATH = ($'/Applications/Docker.app/Contents/Resources/bin:($env.PATH)')
}

##################################################
# corepack
##################################################

$env.PATH = ($'/Users/js/Library/Application Support/dip/bundle/installs/nodejs/18.17.0/lib/node_modules/corepack/shims:($env.PATH)')

##################################################
# Zoxide
##################################################

if (which zoxide | is-empty) == false {
    zoxide init nushell | save -f ~/.zoxide.nu
}

##################################################
# Starship
##################################################

if ('~/.cache/starship/init.nu' | path exists) == false {
    mkdir ~/.cache/starship
    starship init nu | sed "s/size -c/size/" | save ~/.cache/starship/init.nu
}

##################################################
# FZF
##################################################

$env.FZF_DEFAULT_COMMAND = "rg --files --hidden -l -g '!.git/*' -g '!node_modules/*'"
$env.FZF_DEFAULT_OPTS = "-m --height 100% --border --preview 'cat {}'"

##################################################
# Dojo
##################################################

$env.PATH = ($'($env.HOME)/.dojo/bin:($env.PATH)')

##################################################
# Slot
##################################################

$env.PATH = ($'($env.HOME)/.slot/bin:($env.PATH)')

##################################################
# Scarb
##################################################

$env.PATH = ($'($env.HOME)/.local/bin:($env.PATH)')

##################################################
# Starkli
##################################################

$env.PATH = ($'($env.HOME)/.starkli/bin:($env.PATH)')
