##################################################
# Config
##################################################

let-env config = {
    show_banner: false
}

###################################################
## Zoxide
###################################################

source ~/.zoxide.nu

##################################################
# Starship
##################################################

source ~/.cache/starship/init.nu

##################################################
# Alias
##################################################

# Conditional alias does not seem to work currently
# https://github.com/nushell/nushell/issues/5068
alias cat = bat
# alias cd = z
alias vi = nvim
alias ll = ls -la
alias ps = procs
alias top = ytop
