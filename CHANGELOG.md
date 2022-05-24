# 2022

## 0.2.0

* Happy that 0.1.0 published to crates.io
* Sad that docs.rs only works with libaries, not binaries
* Debated making into a library as well
* ... Realized this chagelog is supposted to be about the code
* Figured out nested subcommands and wrote a post: https://dev.to/drazisil/nested-subcommands-in-rest-with-clap-4n5m
* Bumped to vestion 0.2.0 since changing the commands from `nodepm inspect` to `nodepm package inspect` feels like a breaking change

## 0.1.0

* Initial release
* Some docs
* Zero coverage
* Publish to crates.io
* Fingers crossed this actually publishes
### Binary
* Added commands

    * `init` - calls `create_project_manifest`

    * `inspect` - calls `query_package_reqistry`

### Functions
* `create_project_manifest` - Create a _very_ empty "package.json" file with a `name` key set

* `query_package_reqistry` - Query the NPM registry for a package and load the metadata into an `NPMPackageInfo` struct    

### Structures

* `NPMPackageInfo`