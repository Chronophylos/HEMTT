# build
# hemtt build

<pre><code>Build your project

Usage: hemtt.exe build [OPTIONS]

Options:
    <a href="commands.md#-t---threads">-t, --threads &lt;threads&gt;</a>
        Number of threads, defaults to # of CPUs

    <a href="commands.md#-v">-v...</a>
        Verbosity level

    -h, --help
        Print help information (use `-h` for a summary)
</code>
</pre>

`hemtt build` will build your mod into `.hemttout/build`. It will binarize all applicable files, and will not create folder links like `hemtt dev`.

It is intended to be used for testing your mod locally before release.

## Configuration

**.hemtt/project.toml**

```toml
[hemtt.build]
optional_mod_folders = false # Default: true
```

### optional_mod_folders

By default, `hemtt build` will create separate mods for each optional mod folder.
