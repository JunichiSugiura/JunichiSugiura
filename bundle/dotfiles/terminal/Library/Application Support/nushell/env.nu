##################################################
# Homebrew
##################################################

let-env PATH = ($env.PATH | prepend '/opt/homebrew/bin')

##################################################
# Zoxide
##################################################

zoxide init nushell | save ~/.zoxide.nu

##################################################
# Starship
##################################################

mkdir ~/.cache/starship
starship init nu | sed "s/size -c/size/" | save ~/.cache/starship/init.nu
