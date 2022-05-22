# 2022

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