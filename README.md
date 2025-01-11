# neovimacs
The Most Powerful OS!

# What is Neovimacs?
Neovimacs is a Neovim distribution for development inside virtualized Alpine Linux
environments.

By default, Neovimacs assumes you want a neovim-centric environment where Neovim
is launched by default. Quitting Neovim will cause the process to finish. Use Neovim's
Terminal mode to get a shell, and use CTRL+\ + CTRL+n to get out of terminal mode.

# Neobuilder
Neobuilder is a configuration tool that builds an Alpine Linux environment. Neobuilder
produces a root filesystem which you can for example use in a container. It is
designed to build environments for Neovimacs, but is not limited to this usecase.

Neobuilder produces .tar.gz files containing the root filesystem.

Neobuilder creates a root filesystem from a Lua table, which is returned from a
Lua module. The table represents the root filesystem for the desired environment.

All files in the returned table must specify exactly how to build that specific
file, even if several files have the same steps. Neobuilder will automatically
join the steps together for you, they will not be executed separately for every
file. This also means that if a step produces files that are not found in the table
will be discarded.

# Neoman
Neoman is a management tool for Neovimacs environments. It allows you to quickly
enter the right environment and compress old environments for long-term storage.

It is highly reccomended to enter Neovimacs environments through Neoman, as it
allows you to quickly re-enter an environment when accidentally exiting.
