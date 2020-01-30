use quick_xml::events::attributes::Attributes;
use quick_xml::events::Event;
use std::borrow::Borrow;
use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use crate::xml_element::XmlElement;

mod xml_attr;
mod xml_element;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct DataFile {
    pub build: String,
    pub debug: bool,
    pub header: Option<Header>,
    pub games: Vec<Game>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Header {
    pub name: String,
    pub description: String,
    pub category: String,
    pub version: String,
    pub date: String,
    pub author: String,
    pub email: String,
    pub homepage: String,
    pub url: String,
    pub comment: String,
    pub clr_mame_pro: Option<ClrMamePro>,
    pub rom_center: Option<RomCenter>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ClrMamePro {
    pub header: String,
    pub force_merging: ForceMerging,
    pub force_no_dump: ForceNoDump,
    pub force_packing: ForcePacking,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ForceMerging {
    None,
    Split,
    Full,
}

impl Default for ForceMerging {
    fn default() -> ForceMerging {
        ForceMerging::Split
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ForceNoDump {
    Obsolete,
    Required,
    Ignore,
}

impl Default for ForceNoDump {
    fn default() -> ForceNoDump {
        ForceNoDump::Obsolete
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ForcePacking {
    Zip,
    Unzip,
}

impl Default for ForcePacking {
    fn default() -> ForcePacking {
        ForcePacking::Zip
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct RomCenter {
    pub plugin: String,
    pub rom_mode: RomMode,
    pub bios_mode: RomMode,
    pub sample_mode: SampleMode,
    pub lock_rom_mode: bool,
    pub lock_bios_mode: bool,
    pub lock_sample_mode: bool,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum RomMode {
    Merged,
    Split,
    Unmerged,
}

impl Default for RomMode {
    fn default() -> RomMode {
        RomMode::Split
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum SampleMode {
    Merged,
    Unmerged,
}

impl Default for SampleMode {
    fn default() -> SampleMode {
        SampleMode::Merged
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Game {
    pub name: String,
    pub description: String,
    pub is_bios: bool,
    pub source_file: String,
    pub clone_of: String,
    pub rom_of: String,
    pub sample_of: String,
    pub board: String,
    pub rebuild_to: String,
    pub comments: Vec<String>,
    pub year: String,
    pub manufacturer: String,
    pub releases: Vec<Release>,
    pub bios_sets: Vec<BiosSet>,
    pub roms: Vec<Rom>,
    pub disks: Vec<Disk>,
    pub samples: Vec<Sample>,
    pub archives: Vec<Archive>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Release {
    pub name: String,
    pub region: String,
    pub language: String,
    pub date: String,
    pub default: bool,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct BiosSet {
    pub name: String,
    pub description: String,
    pub default: bool,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Rom {
    pub name: String,
    pub size: String,
    pub crc: String,
    pub sha1: String,
    pub md5: String,
    pub merge: String,
    pub status: Status,
    pub date: String,
    pub serial: String, // No-Intro extension
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Status {
    BadDump,
    NoDump,
    Good,
    Verified,
}

impl Default for Status {
    fn default() -> Status {
        Status::Good
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Disk {
    pub name: String,
    pub sha1: String,
    pub md5: String,
    pub merge: String,
    pub status: Status,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Sample {
    pub name: String,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Archive {
    pub name: String,
}

pub struct DatReader<B: BufRead> {
    reader: quick_xml::Reader<B>,
    buf: Vec<u8>,
    strict: bool,
}

impl<'a> DatReader<&'a [u8]> {
    pub fn from_string(xml: &str) -> DatReader<&[u8]> {
        DatReader::from_xml_reader(quick_xml::Reader::from_str(xml))
    }
}

impl<B: BufRead> DatReader<B> {
    pub fn from_reader(reader: B) -> DatReader<B> {
        DatReader::from_xml_reader(quick_xml::Reader::from_reader(reader))
    }
}

impl DatReader<BufReader<File>> {
    pub fn from_file<P: AsRef<Path>>(
        path: P,
    ) -> Result<DatReader<BufReader<File>>, DatReaderError> {
        Ok(DatReader::from_xml_reader(quick_xml::Reader::from_file(
            path,
        )?))
    }
}

#[derive(Debug)]
pub enum DatReaderError {
    Xml(quick_xml::Error),
    UnexpectedAttribute(String),
    UnexpectedElement(String),
}

impl Error for DatReaderError {}

impl fmt::Display for DatReaderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use crate::DatReaderError::*;
        match self {
            Xml(err) => write!(f, "{}", err),
            UnexpectedAttribute(msg) | UnexpectedElement(msg) => write!(f, "{}", msg),
        }
    }
}

impl From<quick_xml::Error> for DatReaderError {
    fn from(e: quick_xml::Error) -> DatReaderError {
        DatReaderError::Xml(e)
    }
}

impl<B: BufRead> DatReader<B> {
    fn from_xml_reader(mut reader: quick_xml::Reader<B>) -> DatReader<B> {
        reader.trim_text(true);
        reader.expand_empty_elements(true);
        DatReader {
            reader,
            buf: Vec::new(),
            strict: true,
        }
    }
    pub fn set_strict(&mut self, strict: bool) {
        self.strict = strict;
    }
    pub fn read_all(mut self) -> Result<DataFile, DatReaderError> {
        let mut result: Option<DataFile> = None;
        loop {
            match self.reader.read_event(&mut self.buf)? {
                Event::Start(ref e) => {
                    let tag = self.reader.decode(e.name())?;
                    match tag.borrow() {
                        "datafile" => {
                            let mut cursor = XmlCursor {
                                tag: "datafile",
                                element: result.get_or_insert_with(Default::default),
                            };
                            cursor.apply_attrs(&self.reader, e.attributes(), self.strict)?;
                            self.read_content(cursor)?;
                        }
                        _ => {
                            if self.strict {
                                break Err(DatReaderError::UnexpectedElement(format!(
                                    "Unexpected top-level element \"{}\"",
                                    tag
                                )));
                            } else {
                                self.skip_content()?;
                            }
                        }
                    }
                }
                Event::Eof => {
                    break result.ok_or_else(|| {
                        DatReaderError::Xml(quick_xml::Error::UnexpectedEof(
                            "Unexpected EOF before a datafile element was seen".to_owned(),
                        ))
                    })
                }
                _ => (),
            }
        }
    }
    fn skip_content(&mut self) -> Result<(), DatReaderError> {
        let mut level = 1;
        loop {
            match self.reader.read_event(&mut self.buf)? {
                Event::Start(_) => {
                    level += 1;
                }
                Event::End(_) => {
                    level -= 1;
                    if level == 0 {
                        break Ok(());
                    }
                }
                Event::Eof => break Ok(()),
                _ => (),
            }
        }
    }
    fn read_content(&mut self, cursor: XmlCursor) -> Result<(), DatReaderError> {
        loop {
            match self.reader.read_event(&mut self.buf)? {
                Event::Start(e) => {
                    let tag = self.reader.decode(e.name())?;
                    if let Some(mut child) = cursor.element.child(&tag) {
                        child.apply_attrs(&self.reader, e.attributes(), self.strict)?;
                        self.read_content(child)?;
                    } else if self.strict {
                        break Err(DatReaderError::UnexpectedElement(format!(
                            "Unexpected child element \"{}\" in element \"{}\"",
                            tag, cursor.tag,
                        )));
                    } else {
                        self.skip_content()?;
                    }
                }
                Event::Text(e) | Event::CData(e) => {
                    if let Some(content) = cursor.element.content() {
                        content.push_str(&self.reader.decode(&e.unescaped()?)?);
                    }
                }
                Event::End(_) => break Ok(()),
                Event::Eof => {
                    break Err(DatReaderError::Xml(quick_xml::Error::UnexpectedEof(
                        format!("Unexpected EOF while reading element \"{}\"", cursor.tag),
                    )));
                }
                _ => (),
            };
        }
    }
}

pub(crate) struct XmlCursor<'a> {
    tag: &'static str,
    element: &'a mut dyn XmlElement,
}

impl<'a> XmlCursor<'a> {
    fn apply_attrs<B: BufRead>(
        &mut self,
        reader: &quick_xml::Reader<B>,
        attrs: Attributes,
        strict: bool,
    ) -> Result<(), DatReaderError> {
        for attr in attrs {
            let attr = attr?;
            let key = reader.decode(attr.key)?;
            let value = attr.unescape_and_decode_value(reader)?;
            if let Some(target) = self.element.attr(&key) {
                if target.set_from_str(&value) {
                    continue;
                }
            }
            if strict {
                return Err(DatReaderError::UnexpectedAttribute(format!(
                    "Unexpected attribute \"{}\"=\"{}\" in element \"{}\"",
                    key, value, self.tag
                )));
            }
        }
        Ok(())
    }
}

#[test]
fn test_full_parse() {
    let input = r#"
<?xml version="1.0"?>
<!DOCTYPE datafile PUBLIC "-//Logiqx//DTD ROM Management Datafile//EN" "http://www.logiqx.com/Dats/datafile.dtd">
<datafile build="Build" debug="yes">
    <header>
        <name>Name</name>
        <description>Description</description>
        <category>Category</category>
        <version>Version</version>
        <date>Date</date>
        <author>Author</author>
        <email>Email</email>
        <homepage>Homepage</homepage>
        <url>Url</url>
        <comment>Comment</comment>
        <clrmamepro header="Header" forcemerging="full" forcenodump="ignore" forcepacking="unzip" />
        <romcenter plugin="Plugin" rommode="unmerged" biosmode="unmerged" samplemode="unmerged" lockrommode="yes" lockbiosmode="yes" locksamplemode="yes" />
    </header>
    <game name="Name" sourcefile="Sourcefile" isbios="yes" cloneof="Cloneof" romof="Romof" sampleof="Sampleof" board="Board" rebuildto="Rebuildto">
        <comment>Comment1</comment>
        <comment>Comment2</comment>
        <description>Description</description>
        <year>Year</year>
        <manufacturer>Manufacturer</manufacturer>
        <release name="Name1" region="Region1" language="Language1" date="Date1" default="yes" />
        <release name="Name2" region="Region2" language="Language2" date="Date2" default="no" />
        <biosset name="Name1" description="Description1" default="yes" />
        <biosset name="Name2" description="Description2" default="yes" />
        <rom name="Name1" size="Size1" crc="Crc1" sha1="Sha1" md5="Md1" merge="Merge1" status="baddump" date="Date1" serial="Serial1" />
        <rom name="Name2" size="Size2" crc="Crc2" sha1="Sha2" md5="Md2" merge="Merge2" status="verified" date="Date2" serial="Serial2" />
        <disk name="Name1" sha1="Sha1" md5="Md1" merge="Merge1" status="baddump" />
        <disk name="Name2" sha1="Sha2" md5="Md2" merge="Merge2" status="verified" />
        <sample name="Name1" />
        <sample name="Name2" />
        <archive name="Name1" />
        <archive name="Name2" />
    </game>
    <game name="Name2">
        <description>Description2</description>
    </game>
</datafile>"#;
    let reader = DatReader::from_string(&input);
    let data_file = reader.read_all().unwrap();
    assert_eq!(
        data_file,
        DataFile {
            build: "Build".to_owned(),
            debug: true,
            header: Some(Header {
                name: "Name".to_owned(),
                description: "Description".to_owned(),
                category: "Category".to_owned(),
                version: "Version".to_owned(),
                date: "Date".to_owned(),
                author: "Author".to_owned(),
                email: "Email".to_owned(),
                homepage: "Homepage".to_owned(),
                url: "Url".to_owned(),
                comment: "Comment".to_owned(),
                clr_mame_pro: Some(ClrMamePro {
                    header: "Header".to_owned(),
                    force_merging: ForceMerging::Full,
                    force_no_dump: ForceNoDump::Ignore,
                    force_packing: ForcePacking::Unzip,
                }),
                rom_center: Some(RomCenter {
                    plugin: "Plugin".to_owned(),
                    rom_mode: RomMode::Unmerged,
                    bios_mode: RomMode::Unmerged,
                    sample_mode: SampleMode::Unmerged,
                    lock_rom_mode: true,
                    lock_bios_mode: true,
                    lock_sample_mode: true,
                })
            }),
            games: vec![
                Game {
                    name: "Name".to_owned(),
                    description: "Description".to_owned(),
                    source_file: "Sourcefile".to_owned(),
                    is_bios: true,
                    clone_of: "Cloneof".to_owned(),
                    rom_of: "Romof".to_owned(),
                    sample_of: "Sampleof".to_owned(),
                    board: "Board".to_owned(),
                    rebuild_to: "Rebuildto".to_owned(),
                    comments: vec!["Comment1".to_owned(), "Comment2".to_owned()],
                    year: "Year".to_owned(),
                    manufacturer: "Manufacturer".to_owned(),
                    releases: vec![
                        Release {
                            name: "Name1".to_owned(),
                            region: "Region1".to_owned(),
                            language: "Language1".to_owned(),
                            date: "Date1".to_owned(),
                            default: true,
                        },
                        Release {
                            name: "Name2".to_owned(),
                            region: "Region2".to_owned(),
                            language: "Language2".to_owned(),
                            date: "Date2".to_owned(),
                            default: false,
                        }
                    ],
                    bios_sets: vec![
                        BiosSet {
                            name: "Name1".to_owned(),
                            description: "Description1".to_owned(),
                            default: true,
                        },
                        BiosSet {
                            name: "Name2".to_owned(),
                            description: "Description2".to_owned(),
                            default: true,
                        }
                    ],
                    roms: vec![
                        Rom {
                            name: "Name1".to_owned(),
                            size: "Size1".to_owned(),
                            crc: "Crc1".to_owned(),
                            sha1: "Sha1".to_owned(),
                            md5: "Md1".to_owned(),
                            merge: "Merge1".to_owned(),
                            status: Status::BadDump,
                            date: "Date1".to_owned(),
                            serial: "Serial1".to_owned()
                        },
                        Rom {
                            name: "Name2".to_owned(),
                            size: "Size2".to_owned(),
                            crc: "Crc2".to_owned(),
                            sha1: "Sha2".to_owned(),
                            md5: "Md2".to_owned(),
                            merge: "Merge2".to_owned(),
                            status: Status::Verified,
                            date: "Date2".to_owned(),
                            serial: "Serial2".to_owned()
                        }
                    ],
                    disks: vec![
                        Disk {
                            name: "Name1".to_owned(),
                            sha1: "Sha1".to_owned(),
                            md5: "Md1".to_owned(),
                            merge: "Merge1".to_owned(),
                            status: Status::BadDump,
                        },
                        Disk {
                            name: "Name2".to_owned(),
                            sha1: "Sha2".to_owned(),
                            md5: "Md2".to_owned(),
                            merge: "Merge2".to_owned(),
                            status: Status::Verified,
                        },
                    ],
                    samples: vec![
                        Sample {
                            name: "Name1".to_owned(),
                        },
                        Sample {
                            name: "Name2".to_owned(),
                        }
                    ],
                    archives: vec![
                        Archive {
                            name: "Name1".to_owned(),
                        },
                        Archive {
                            name: "Name2".to_owned(),
                        }
                    ],
                },
                Game {
                    name: "Name2".to_owned(),
                    description: "Description2".to_owned(),
                    source_file: "".to_owned(),
                    is_bios: false,
                    clone_of: "".to_owned(),
                    rom_of: "".to_owned(),
                    sample_of: "".to_owned(),
                    board: "".to_owned(),
                    rebuild_to: "".to_owned(),
                    comments: vec![],
                    year: "".to_owned(),
                    manufacturer: "".to_owned(),
                    releases: vec![],
                    roms: vec![],
                    bios_sets: vec![],
                    disks: vec![],
                    samples: vec![],
                    archives: vec![],
                }
            ],
        }
    );
}
