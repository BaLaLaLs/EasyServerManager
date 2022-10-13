use std::collections::HashMap;
use std::io::Error;
use std::ffi::OsString;
use std::io;
use std::ops::Deref;
use std::io::Write;

use rbatis::snowflake::new_snowflake_id;
use serde::{Serialize, Serializer};
use sysinfo::{Component, Disk, Process, System, Cpu, SystemExt, Pid, LoadAvg, Networks, NetworksExt, NetworkExt};


/// `sysinfo` with extended features
#[derive(Debug)]
pub struct SysInfoExt<'a> {
    processes: Vec<&'a Process>,
    cpu_list: &'a [Cpu],
    hostname: String,
    up_time: u64,
    pub system_name: String,
    system_kernel_version: String,
    os_version: String,
    long_os_version: String,
    global_cpu_info: &'a Cpu,
    memory: Memory,
    disks: &'a [Disk],
    load_avg: LoadAvg,
    networks: Vec<Network<'a>>
}
#[derive(Debug, Serialize)]
struct Memory {
    total_memory: u64,
    used_memory: u64,
    total_swap: u64,
    used_swap: u64,
}
#[derive(Debug, Serialize)]
struct Network<'a> {
    interface_name: &'a str,
    received: u64,
    total_received: u64,
    transmitted: u64,
    total_transmitted: u64
}

impl<'a> SysInfoExt<'a> {
    pub fn new(system: &'a System) -> Self {
        SysInfoExt {
            processes: system.processes().iter().map(|i| i.1).collect(),
            cpu_list: system.cpus(),
            system_name: system.name().unwrap_or("Unknown".to_owned()),
            up_time: system.uptime(),
            hostname: system.host_name().unwrap_or("Unknown".to_owned()),
            system_kernel_version: system.kernel_version().unwrap_or("Unknown".to_owned()),
            os_version: system.os_version().unwrap_or("Unknown".to_owned()),
            long_os_version: system.long_os_version().unwrap_or("Unknown".to_owned()),
            global_cpu_info: system.global_cpu_info(),
            memory: Memory {
                total_memory: system.total_memory(),
                used_memory: system.used_memory(),
                total_swap: system.total_swap(),
                used_swap: system.used_swap()
            },
            disks: system.disks(),
            load_avg: system.load_average(),
            networks: system.networks().iter().map(|(interface_name, data)| {
                Network {
                    interface_name,
                    received: data.received(),
                    total_received: data.total_received(),
                    transmitted: data.transmitted(),
                    total_transmitted: data.total_transmitted()
                }
            }).collect()
        }
    }
}
use serde::ser::SerializeMap;
use crate::modules::system::sysinfo_serde::Ser;

impl<'a> Serialize for SysInfoExt<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("cpus", &self.cpu_list.iter().map(|p| Ser::new(p)).collect::<Vec<Ser<Cpu>>>()).unwrap();
        map.serialize_entry("hostname", &self.hostname).unwrap();
        map.serialize_entry("up_time", &self.up_time).unwrap();
        map.serialize_entry("system_name", &self.system_name).unwrap();
        map.serialize_entry("system_kernel_version", &self.system_kernel_version).unwrap();
        map.serialize_entry("long_os_version", &self.long_os_version).unwrap();
        map.serialize_entry("global_cpu_info", &Ser::new(self.global_cpu_info)).unwrap();
        map.serialize_entry("memory", &self.memory).unwrap();
        map.serialize_entry("disks", &self.disks.iter().map(|d| Ser::new(d)).collect::<Vec<Ser<Disk>>>()).unwrap();
        map.serialize_entry("load_avg", &Ser::new(&self.load_avg)).unwrap();
        map.serialize_entry("networks", &self.networks).unwrap();
        map.serialize_entry("processes", &self.processes.iter().map(|p| Ser::new(p.deref())).collect::<Vec<Ser<Process>>>()).unwrap();
        map.end()
    }
}
