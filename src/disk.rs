#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Disk {
    label: String,
    path: String,

    smart: SmartData,

    size: u128, // Not u64 bc the largest drives in existence nowadays are near-ish to the 10^19 max value limit of a u64
    used: u128,
    format: String,

    partitions: Vec<Partition>,
}

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Partition {
    label: String,
    uuid: String,
    path: String,  // path relative to drive location
    mount: String, // path relative to where it is mounted for the user to see

    size: u128,
    used: u128,

    fs: String, // file system
}

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct SmartData {
    status: String,
    wear_lvl: u8,
    temp: u8,

    run_hours: u32,
    reads: u128,
    writes: u128,

    serial_number: String,
    model: String, // usually the large string of model number, revision, etc.
    model_family: Option<String>, // most of the time the actual human readable model
    interface: String,
    disk_type: String,
}
