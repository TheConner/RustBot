# Writing Templates

Templates are files that RustBot reads to provide various canned messages to users. Templates are designed to be user-modifiable, such that any user can edit canned responses to tailor their deployment **without** re-compiling RustBot. A key aspect of templates is they support variable injection, since parts of RustBot may be modified through environment variables (e.g., BOT_PREFIX).

An example template below:

```md
You can run the run command by entering {{BOT_PREFIX}}run
```
When rendered is (assuming no BOT_PREFIX is set, resulting in the default of !)
```
You can run the run command by entering !run
```