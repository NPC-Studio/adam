# adam

<p align="center">
  <img width="400" height="400" src=assets/adam-200x200.png>
</p>

> I am thy creature: I ought to be thy Adam, but I am rather the fallen angel, whom thou drivest from joy for no misdeed.
> -- Frankenstein's Monster in *Frankenstein* by Mary Shelly

adam is a command-line utility for compiling GameMaker projects on Windows and macOS. Invoking adam is trivial:

```sh
adam run
```

This will compile your project, run it, and give you stdout (`"show_debug_message"`) with colorization and links. `adam` supports compiling with the VM (default) and the YYC (by passing in `--yyc`). `adam` also supports faster recompilation than GameMaker does, so if users recompile a game without making changes, their game will instantly load, without invoking the compiler at all. This is especially useful, since `adam` easily allows you to run multiple instances of your game at the same time on your machine.

`adam` will place all its generated artifacts within a folder relative to the working directory -- by default, it will use `"target"` as its output output. **It is highly advised that you add your output directory to your .gitignore.**

## COMMANDS

`run`: Builds and runs the project.

`build`: Builds a project without running it (but will report compile errors)

`release`: Builds a zip of the project (only available for users with an Enterprise license)

`clean`: Cleans the output directory

`test`: Runs the game after setting user-defined environment variables. See the [config file guide](docs/CONFIG_FILE_GUIDE.md) for more information.

You can also run `adam help` to see a more detailed version of the above.

## INSTALLATION

The best way to install adam is through cargo:

```sh
cargo install adam
```

You can get cargo by install Rust.

## CHANGELOG AND ROADMAP

Please see the [CHANGELOG](CHANGELOG.md) for release history, and the [ROADMAP](ROADMAP.md) for planned features.

## CUSTOMIZATION

You will likely need to customize an `adam` command for most usages. To see CLI options, simply run `adam run --help`:

Of special note, please see `--yyc`, which will allow users to compile using the YYC, and `-c`, which allows users to pass in a configuration.

However, passing in numerous values every compile can become tiresome. To support this, users can create a config file in either `JSON` or `TOML`, where these options can be specified. To create an adam configuration file, please follow [this guide](docs/CONFIG_FILE_GUIDE.md).

## CHECK

Running `adam check` will invoke scripts *if you set up them up in your configuration file.*
These scripts will also have the following environment variables set:

| Name              | Value   |
| ------------------|---------|
| `ADAM_CHECK`        | 1       |
| `ADAM_YYC`          | 0 or 1  |
| `ADAM_CONFIG`       | String  |
| `ADAM_VERBOSITY`    | Number  |
| `ADAM_OUTPUT_FOLDER`| String  |
| `ADAM_IGNORE_CACHE` | Number  |

## LICENSE

Dual-licensed under MIT or APACHE 2.0.
