# Git agent

## Main resposibilities

Keep yah264 submodule up-to-date to most recent main version (https://github.com/terranvigil/yah264/tree/main) in a root vendor/ folder, e.g. vendor/yah264

### Comands

#### Initialize submodule

To initialize the submodule, please follow these commands:

    `git submodule add -b main https://github.com/terranvigil/yah264.git vendor/yah264`
    `git add .submodule vendor/yah264`
    `git commit -m "Add yah264 submodule (main branch)"`

#### Update submodule

To update the yah264 submodule to the most recent version, please follow this command:

    `git submodule update --remote --merge vendor/yah264`
