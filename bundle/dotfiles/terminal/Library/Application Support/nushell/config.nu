##################################################
# Config
##################################################

let-env config = {
    show_banner: false
}

##################################################
# Zoxide
##################################################

source ~/.zoxide.nu

##################################################
# Starship
##################################################

source ~/.cache/starship/init.nu

##################################################
# Alias
##################################################

# Conditional alias does not seem to work currently
alias cd = z
alias cat = bat
alias vi = /opt/homebrew/bin/nvim

alias ll = ls -la
alias ps = procs
alias top = ytop
alias du = dust
