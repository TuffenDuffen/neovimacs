# neovimacs
The Most Powerful OS!

# What is Neovimacs?
Neovimacs is actually a configuration tool that sets up an Alpine Linux environment
for development with Neovim. It can correctly configure the system and packages
in a single step, reducing the need to run lots of commands in for example a Dockerfile.

The configuration of the system is done using Lua, and works similarly to Wezterm.
You create a lua module which returns a configuration of the system, and Neovimacs
sets up the Alpine system accordingly. Neovimacs is designed to work in virtualized
environments and containers.

By default, Neovimacs assumes you want a neovim-centric environment where Neovim
is launched by default. Quitting Neovim will cause the process to finish. Use Neovim's
Terminal mode to get a shell, and use CTRL+\ + CTRL+n to get out of terminal mode.
