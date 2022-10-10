use serde::{Serialize, Serializer};
use serde::ser::SerializeMap;
use sysinfo::{Cpu, CpuExt, Disk, DiskExt, DiskType, LoadAvg, Networks, NetworksExt};


pub fn serialize<T, S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer,
          for<'a> Ser<'a, T>: Serialize
{
    Ser::new(value).serialize(serializer)
}

pub struct Ser<'a, T: 'a>(&'a T);

impl<'a, T> Ser<'a, T>
    where Ser<'a, T>: Serialize
{
    #[inline(always)]
    pub fn new(value: &'a T) -> Self {
        Ser(value)
    }
}

impl<'a> Serialize for Ser<'a, Cpu> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("name", self.0.name())?;
        let i = *&self.0.cpu_usage() as i32;
        map.serialize_entry("cpu_usage", &i).unwrap();
        map.serialize_entry("brand", self.0.brand()).unwrap();
        map.end()
    }
}

impl<'a> Serialize for Ser<'a, Disk> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("type",
                            &match self.0.type_() {
                                DiskType::HDD => "HDD".to_owned(),
                                DiskType::SSD => "SSD".to_owned(),
                                DiskType::Unknown(size) => format!("Unknown({})", size),
                            })?;
        map.serialize_entry("name", self.0.name().to_str().unwrap())?;
        map.serialize_entry("file_system", ::std::str::from_utf8(self.0.file_system()).unwrap())?;
        map.serialize_entry("mount_point", self.0.mount_point())?;
        map.serialize_entry("total_space", &self.0.total_space())?;
        map.serialize_entry("available_space", &self.0.available_space())?;
        map.end()
    }
}

impl<'a> Serialize for Ser<'a, LoadAvg> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("one", &self.0.one).unwrap();
        map.serialize_entry("fifteen", &self.0.fifteen).unwrap();
        map.serialize_entry("five", &self.0.five).unwrap();
        map.end()
    }
}
impl<'a> Serialize for Ser<'a, Networks> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        let mut map = serializer.serialize_map(None)?;

        map.end()
    }
}