pub fn human_readable(mut value: f64) -> String {
    let suffixes = [
        "", "K", "M", "B", "T", "qd", "Qn", "sx", "Sp", "Oc", "No", "De", "Ud", "DD", "tdD", "qdD",
        "QnD", "sxD", "SpD", "OcD", "NvD", "Vgn", "UVg", "DVg", "TVg",
    ];
    let mut i = 0;

    while value >= 1000.0 && i < suffixes.len() - 1 {
        value /= 1000.0;
        i += 1;
    }

    format!("{:.1}{}", value, suffixes[i])
}
