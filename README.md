# neovimacs
The Most Powerful OS!

# What is Neovimacs?
Neovimacs is a Neovim distribution for development inside virtualized Alpine Linux
environments.

By default, Neovimacs assumes you want a neovim-centric environment where Neovim
is launched by default. Quitting Neovim will cause the process to finish. Use Neovim's
Terminal mode to get a shell, and use CTRL+\ + CTRL+n to get out of terminal mode.

# Neoconfig
Neoconfig is the configuration tool for Neovimacs. It sets up an Alpine linux environment
with the tools and packages you desire.

Neoconfig uses a file called `neoconfig.lua` to create the environment. The file
returns a table, which Neoconfig builds according to. The structure of the table
looks like this:
```
config
|-apps
|-system
| |-packages
|-neovimacs
| |-plugins
| |-settings
|-root
```

# Neoman
Neoman is a management tool for Neovimacs environments. It allows you to quickly
enter the right environment and compress old environments for long-term storage.

It is highly reccomended to enter Neovimacs environments through Neoman, as it
allows you to quickly re-enter an environment when accidentally exiting.
