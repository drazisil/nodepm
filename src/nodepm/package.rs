use anyhow::{ anyhow, Result};

/// Query the `registry_host` for `package_name`, and return information for `version`
pub fn inspect_package(registry_host: &str, package_name: &str, version: &str) -> Result<()> {
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

    // Lets get the version blob
    let requested_version = match version {
        "latest" => response_json["dist-tags"]["latest"].as_str().unwrap_or(""),
        _ => version,
    };

    println!(
        "SHA1: {}",
        response_json["versions"][requested_version]["dist"]["shasum"]
    );
    println!(
        "SHA512: {}",
        response_json["versions"][requested_version]["dist"]["integrity"]
    );
    println!(
        "download_url: {}",
        response_json["versions"][requested_version]["dist"]["tarball"]
    );
    Ok(())
}
