
pub const NAME: &'static str = "Oxide";
// NOTE: When const-expr are better supported we can parse option_env!("CARGO_PKG_AUTHORS")
pub const AUTHORS: &'static str = "Jeffrey Meyer <itotallyrock>";

pub fn oxide_info() -> String {
    // How long the info string should be
    const INFO_LENGTH: usize = 40;

    let mut info = String::with_capacity(INFO_LENGTH);
    // Print the name
    info.push_str(NAME);
    // Get the version
    let version: Option<&'static str> = option_env!("CARGO_PKG_VERSION");
    // If the version exists print out along with name
    if let Some(version) = version {
        info.push(' ');
        info.push('v');
        info.push_str(version);
    }

    // Print out enabled features
    // Openings enabled
    #[cfg(any(feature = "openings", feature = "default"))]
        info.push_str(" + Openings");
    // Logging enabled
    #[cfg(any(feature = "logging", feature = "default"))]
        info.push_str(" + Logging");

    #[cfg(feature = "tuning")]
        info.push_str(" + Tuning");

    #[cfg(feature = "low_memory")]
        info.push_str(" + Low Memory");

    // Print out the license
    let license_result = option_env!("CARGO_PKG_LICENSE");
    if let Some(license) = license_result {
        info.push_str(" (");
        info.push_str(license);
        info.push(')');
    }

    info
}
