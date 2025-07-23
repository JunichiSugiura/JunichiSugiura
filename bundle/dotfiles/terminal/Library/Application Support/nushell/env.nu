$env.path ++= [
    '/Applications/Docker.app/Contents/Resources/bin',
    $'($env.HOME)/.cargo/bin',
    $'($env.HOME)/.dojo/bin',
    $'($env.HOME)/.dojo/dojoup',
    $'($env.HOME)/.local/bin',
    $'($env.HOME)/.slot/bin',
    $'($env.HOME)/.starkli/bin',
    $'($env.HOME)/Library/Application Support/dip/bundle/shims',
    '/opt/homebrew/bin',
    '/opt/homebrew/lib/ruby/gems/3.0.0/bin',
    '/opt/homebrew/opt/ruby/bin',
    # $'($env.HOME)/.asdf/shims',
]

$env.path ++= [
    $'(npm config get prefix)/bin',
]

##################################################
# GPG
##################################################

$env.GPG_TTY = (tty)

##################################################
# Docker
##################################################

if ('/Applications/Docker.app/Contents/Resources/bin' | path exists) == true {
    $env.path ++= [
        '/Applications/Docker.app/Contents/Resources/bin'
    ]
}

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
