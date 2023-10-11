# mx-plug

This repository contains the framework for creating plugins for my fork of the MultiversX Blockchain VM. More information about it [here](https://github.com/laurci/mx-unit).

Each plugin is split in 2 parts:

- contract bridge: this is a library that is used by the smart contract and it wraps direct calls to the plugin
- plugin: this is a C dynamic library that gets loaded by the VM on startup and exposes functions that are called from the VM

## Plugin definition

You just need to use the `plugin!` macro and specify the name and the list of functions you want to expose to the contract:

```rust
mx_plug_core::plugin! {
    name = "demo",
    fns = [test_1, test_2, test_3]
}
```

Check the example in the `plugin-demo/plugin/` directory.
