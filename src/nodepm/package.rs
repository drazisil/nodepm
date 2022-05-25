use anyhow::{ anyhow, Result};
use json::JsonValue;

#[derive(Debug)]
pub struct NPMPackageInfo {
    /// Name of the package
    pub name: String,

    /// Version of the package
    pub version: String,

    // SHA1 of the package
    pub shasum: String,

    /// SHA512 of the package
    pub integrity: String,

    /// Tarball download URL
    pub tarball: String
}

impl From<& mut json::JsonValue> for NPMPackageInfo {
    fn from(json: & mut json::JsonValue) -> Self {
        let package_version = &json["version"].to_string();
        Self { 
            name: json["name"].to_string(),
            version: package_version.to_string(),
            shasum: json["versions"][package_version]["dist"]["shasum"].to_string(),
            integrity: json["versions"][package_version]["dist"]["integrity"].to_string(),
            tarball: json["versions"][package_version]["dist"]["tarball"].to_string()
        }
    }

}

impl NPMPackageInfo {
    fn from_json(json: & mut json::JsonValue, name: &str, version: &str) -> Self {
        json["name"] =  JsonValue::from(name);
        json["version"] = JsonValue::from(version);
        NPMPackageInfo::from(json)
    }

}

/// Query the `registry_host` for `package_name`, and return information for `version`
/// 
/// Example
/// ```rust
/// query_package_registry("https://registry.npmjs.com", "nodepm", "latest");
/// ```
/// 
/// # Errors
/// 
/// Will return `Err` in the following cases:
/// * Server error when querying the npm registry
/// * IO/Transport error when querying the npm registry
/// * Error parsing the response body into a string
/// * Error parsing the response body string into JSON
/// 
/// # TODO: Reduce error cases by bubbling or handling errors <https://github.com/drazisil/nodepm/issues/10>
pub fn query_package_reqistry(registry_host: &str, package_name: &str, version: &str) -> Result<()> {
    println!(
        "Inspecting the package named {:?} at version {:?}",
        package_name, version
    );

    let user_agent = format!(
        "{}: {} - {}",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION"),
        env!("CARGO_PKG_AUTHORS")
    );
    let query = format!("{}/{}", registry_host, package_name);

    // Requires the `json` feature enabled.
    let result = ureq::get(&query)
        .set("User-Agent", &user_agent)
        .set("Accept", "application/json")
        .call();
        

    let response_body_result = match result {
        Err(ureq::Error::Status(code, response)) => {
            /* the server returned an unexpected status
            code (such as 400, 500 etc) */
            return Err(anyhow!("Unable to locate package named {}: {} - {}", package_name, code, response.status_text()));
        }
        Err(error) => {
            /* some kind of io/transport error */
            return Err(anyhow!(
                "Unknown error querying registry '{}'. Please file an issue: {}/issues/new",
                error,
                env!("CARGO_PKG_REPOSITORY")
            ));
        }
        Ok(response) => response.into_string(),
    };

    let response_body = match response_body_result {
        Err(error) => { return Err(anyhow!("Unknown error reading response body: {}", error))},
        Ok(response) => response,
    };

    let result = json::parse(&response_body);

    let response_json = match result {
        Err(error) => {
            return Err(anyhow!("Unknown error transforming response into JSON '{}'. Please file an issue: {}/issues/new", error, env!("CARGO_PKG_REPOSITORY")))
        }
        Ok(response) => response,
    };


    let mut mut_json = JsonValue::clone(&response_json);

    // Lets get the version blob
    let requested_version = match version {
        "latest" => {response_json["dist-tags"]["latest"].as_str().unwrap_or("") },
        _ => version,
    };

    let package_info = NPMPackageInfo::from_json(& mut mut_json, package_name, requested_version);

    println!("{:#?}", package_info);
    Ok(())
}
