# ROADMAP

## v0.1.0

- [x] Commandline Interface
  - [x] `adam run`
  - [x] `--yyc`
  - [x] `adam build`
  - [x] `adam clean`
  - [x] `--config=some_config`
  - [x] `-v`
- [x] Figure out the YYC
- [x] Clarify output status on gamestartup
- [ ] Mac Checkup
  - [ ] Basic compile
  - [ ] Yyc
- [x] Check Config is working
  - [x] visual_studio_path needs to be added (committed and reverted -- cannot be implemented now)
  - [x] rename `target` to `yyp`
  - [x] add `output_folder_name` option.
- [x] Clean up intermediary files
- [x] Build without Run
- [x] Run immediately
- [-] Fancy boy stuff (not necessary)
  - [x] better handles for errors
  - [x] parse fpaths in output to be cleaner
- [x] Show the Config on the compiling output
- [x] Make sure the yyc cache and the vm cache are coherent and separate.
- [-] Installation Guide in the Readme
  - [ ] Make Configuration Guide
  - [ ] Homebrew publish
  - [ ] Chocolatey publish
  - [ ] Scoop publish
