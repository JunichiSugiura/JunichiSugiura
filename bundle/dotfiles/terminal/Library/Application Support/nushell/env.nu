##################################################
# Homebrew
##################################################

let-env PATH = ($'/opt/homebrew/bin:($env.PATH)')

##################################################
# Cargo
##################################################

# List doesn't seem to work.
let-env PATH = ($'($env.HOME)/.cargo/bin:($env.PATH)')

##################################################
# Homebrew Ruby
##################################################

let-env PATH = ($'/opt/homebrew/opt/ruby/bin:/opt/homebrew/lib/ruby/gems/3.0.0/bin:($env.PATH)')

###################################################
# asdf
###################################################

# let-env PATH = ($'($env.HOME)/.asdf/shims/:($env.PATH)')

##################################################
# dip
##################################################

let-env PATH = ($'($env.HOME)/Library/Application Support/dip/bundle/shims:($env.PATH)')

##################################################
# GPG
##################################################

let-env GPG_TTY = (echo (tty))

##################################################
# pnpm
##################################################

let-env PNPM_HOME = ($'($env.HOME)/Library/pnpm') 
let-env PATH = ($'($env.PNPM_HOME):($env.PATH)')

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

let-env FZF_DEFAULT_COMMAND = "rg --files --hidden -l -g '!.git/*' -g '!node_modules/*'"
let-env FZF_DEFAULT_OPTS = "-m --height 100% --border --preview 'cat {}'"
