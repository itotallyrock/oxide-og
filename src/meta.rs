
pub const NAME: &str = "Oxide";
// NOTE: When const-expr are better supported we can parse option_env!("CARGO_PKG_AUTHORS")
pub const AUTHORS: &str = "Jeffrey Meyer <itotallyrock>";

pub fn oxide_info() -> String {
    // How long the info string should be
    const INFO_LENGTH: usize = 40;

    let mut info = String::with_capacity(INFO_LENGTH);
    // Print the name
    info.push_str(NAME);

    #[cfg(debug_assertions)]
        info.push_str(" (Debug)");

    // Get the version
    let version: Option<&'static str> = option_env!("CARGO_PKG_VERSION");
    // If the version exists print out along with name
    if let Some(version) = version {
        info.push(' ');
        info.push('v');
        info.push_str(version);
    }

    // Print out enabled features
    // Tuning enabled
    #[cfg(feature = "tuning")]
        info.push_str(" + Tuning");
    // Low memory enabled
    #[cfg(feature = "low_memory")]
        info.push_str(" + Low Memory");

    // The following features aren't printed with default because they are assumed to be built in (still print when just manually added feature)
    // Logging enabled
    #[cfg(feature = "logging")]
        info.push_str(" + Logging");
    // Openings enabled
    #[cfg(feature = "openings")]
        info.push_str(" + Openings");
    // Quiescence search
    #[cfg(feature = "qsearch")]
        info.push_str(" + Q-Search");
    // Killer heuristic
    #[cfg(feature = "killer_heuristic")]
        info.push_str(" + Killers");
    // History heuristic
    #[cfg(feature = "history_heuristic")]
        info.push_str(" + History");
    // Delta pruning
    #[cfg(feature = "delta_pruning")]
        info.push_str(" + Delta");
    // Internal iterative deepening
    #[cfg(feature = "move_ordering")]
        info.push_str(" + IID");


    // Print out the license
    let license_result = option_env!("CARGO_PKG_LICENSE");
    if let Some(license) = license_result {
        info.push_str(" (");
        info.push_str(license);
        info.push(')');
    }

    info
}
