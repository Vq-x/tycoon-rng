use num::{Num, NumCast, ToPrimitive};

pub fn human_readable<T>(mut value: T) -> String
where
    T: Num + NumCast + ToPrimitive + Copy,
{
    let suffixes = [
        "", "K", "M", "B", "T", "qd", "Qn", "sx", "Sp", "Oc", "No", "De", "Ud", "DD", "tdD", "qdD",
        "QnD", "sxD", "SpD", "OcD", "NvD", "Vgn", "UVg", "DVg", "TVg",
    ];
    let mut i = 0;

    while value.to_f64().unwrap() >= 1000.0 && i < suffixes.len() - 1 {
        value = NumCast::from(value.to_f64().unwrap() / 1000.0).unwrap();
        i += 1;
    }

    format!("{:.1}{}", value.to_f64().unwrap(), suffixes[i])
}
