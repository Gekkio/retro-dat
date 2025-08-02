// SPDX-FileCopyrightText: 2019-2022 Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::{
    xml_attr::XmlAttr, Archive, BiosSet, ClrMamePro, DataFile, Disk, Game, Header, Release, Rom,
    RomCenter, Sample, XmlCursor,
};

fn cursor<'a, T: XmlElement>(tag: &'static str, element: &'a mut T) -> Option<XmlCursor<'a>> {
    Some(XmlCursor { tag, element })
}

pub(crate) trait XmlElement {
    fn attr(&mut self, _: &str) -> Option<&mut dyn XmlAttr> {
        None
    }
    fn child(&mut self, _: &str) -> Option<XmlCursor> {
        None
    }
    fn content(&mut self) -> Option<&mut String> {
        None
    }
}

impl XmlElement for String {
    fn content(&mut self) -> Option<&mut String> {
        Some(self)
    }
}

impl XmlElement for DataFile {
    fn attr(&mut self, key: &str) -> Option<&mut dyn XmlAttr> {
        match key {
            "build" => Some(&mut self.build),
            "debug" => Some(&mut self.debug),
            _ => None,
        }
    }
    fn child(&mut self, tag: &str) -> Option<XmlCursor> {
        match tag {
            "header" => cursor("header", self.header.get_or_insert_with(Header::default)),
            "game" => {
                self.games.push(Game::default());
                cursor("game", self.games.last_mut().unwrap())
            }
            _ => None,
        }
    }
}
impl XmlElement for Header {
    fn child(&mut self, tag: &str) -> Option<XmlCursor> {
        match tag {
            "name" => cursor("name", &mut self.name),
            "description" => cursor("description", &mut self.description),
            "category" => cursor("category", &mut self.category),
            "version" => cursor("version", &mut self.version),
            "date" => cursor("date", &mut self.date),
            "author" => cursor("author", &mut self.author),
            "email" => cursor("email", &mut self.email),
            "homepage" => cursor("homepage", &mut self.homepage),
            "url" => cursor("url", &mut self.url),
            "comment" => cursor("comment", &mut self.comment),
            "clrmamepro" => cursor(
                "clrmamepro",
                self.clr_mame_pro.get_or_insert_with(Default::default),
            ),
            "romcenter" => cursor(
                "romcenter",
                self.rom_center.get_or_insert_with(Default::default),
            ),
            _ => None,
        }
    }
}

impl XmlElement for ClrMamePro {
    fn attr(&mut self, key: &str) -> Option<&mut dyn XmlAttr> {
        match key {
            "header" => Some(&mut self.header),
            "forcemerging" => Some(&mut self.force_merging),
            "forcenodump" => Some(&mut self.force_no_dump),
            "forcepacking" => Some(&mut self.force_packing),
            _ => None,
        }
    }
}

impl XmlElement for RomCenter {
    fn attr(&mut self, key: &str) -> Option<&mut dyn XmlAttr> {
        match key {
            "plugin" => Some(&mut self.plugin),
            "rommode" => Some(&mut self.rom_mode),
            "biosmode" => Some(&mut self.bios_mode),
            "samplemode" => Some(&mut self.sample_mode),
            "lockrommode" => Some(&mut self.lock_rom_mode),
            "lockbiosmode" => Some(&mut self.lock_bios_mode),
            "locksamplemode" => Some(&mut self.lock_sample_mode),
            _ => None,
        }
    }
}

impl XmlElement for Game {
    fn attr(&mut self, key: &str) -> Option<&mut dyn XmlAttr> {
        match key {
            "id" => Some(&mut self.id),
            "name" => Some(&mut self.name),
            "sourcefile" => Some(&mut self.source_file),
            "isbios" => Some(&mut self.is_bios),
            "cloneof" => Some(&mut self.clone_of),
            "cloneofid" => Some(&mut self.clone_of_id),
            "romof" => Some(&mut self.rom_of),
            "sampleof" => Some(&mut self.sample_of),
            "board" => Some(&mut self.board),
            "rebuildto" => Some(&mut self.rebuild_to),
            _ => None,
        }
    }
    fn child(&mut self, tag: &str) -> Option<XmlCursor> {
        match tag {
            "name" => cursor("name", &mut self.name),
            "description" => cursor("description", &mut self.description),
            "comment" => {
                self.comments.push(String::new());
                cursor("comment", self.comments.last_mut().unwrap())
            }
            "year" => cursor("year", &mut self.year),
            "manufacturer" => cursor("manufacturer", &mut self.manufacturer),
            "release" => {
                self.releases.push(Release::default());
                cursor("release", self.releases.last_mut().unwrap())
            }
            "biosset" => {
                self.bios_sets.push(BiosSet::default());
                cursor("biosset", self.bios_sets.last_mut().unwrap())
            }
            "rom" => {
                self.roms.push(Rom::default());
                cursor("rom", self.roms.last_mut().unwrap())
            }
            "disk" => {
                self.disks.push(Disk::default());
                cursor("disk", self.disks.last_mut().unwrap())
            }
            "sample" => {
                self.samples.push(Sample::default());
                cursor("sample", self.samples.last_mut().unwrap())
            }
            "archive" => {
                self.archives.push(Archive::default());
                cursor("archive", self.archives.last_mut().unwrap())
            }
            _ => None,
        }
    }
}

impl XmlElement for Release {
    fn attr(&mut self, key: &str) -> Option<&mut dyn XmlAttr> {
        match key {
            "name" => Some(&mut self.name),
            "region" => Some(&mut self.region),
            "language" => Some(&mut self.language),
            "date" => Some(&mut self.date),
            "default" => Some(&mut self.default),
            _ => None,
        }
    }
}

impl XmlElement for BiosSet {
    fn attr(&mut self, key: &str) -> Option<&mut dyn XmlAttr> {
        match key {
            "name" => Some(&mut self.name),
            "description" => Some(&mut self.description),
            "default" => Some(&mut self.default),
            _ => None,
        }
    }
}

impl XmlElement for Rom {
    fn attr(&mut self, key: &str) -> Option<&mut dyn XmlAttr> {
        match key {
            "name" => Some(&mut self.name),
            "size" => Some(&mut self.size),
            "crc" => Some(&mut self.crc),
            "sha1" => Some(&mut self.sha1),
            "sha256" => Some(&mut self.sha256),
            "md5" => Some(&mut self.md5),
            "merge" => Some(&mut self.merge),
            "status" => Some(&mut self.status),
            "date" => Some(&mut self.date),
            "serial" => Some(&mut self.serial),
            _ => None,
        }
    }
}

impl XmlElement for Disk {
    fn attr(&mut self, key: &str) -> Option<&mut dyn XmlAttr> {
        match key {
            "name" => Some(&mut self.name),
            "sha1" => Some(&mut self.sha1),
            "md5" => Some(&mut self.md5),
            "merge" => Some(&mut self.merge),
            "status" => Some(&mut self.status),
            _ => None,
        }
    }
}

impl XmlElement for Sample {
    fn attr(&mut self, key: &str) -> Option<&mut dyn XmlAttr> {
        match key {
            "name" => Some(&mut self.name),
            _ => None,
        }
    }
}

impl XmlElement for Archive {
    fn attr(&mut self, key: &str) -> Option<&mut dyn XmlAttr> {
        match key {
            "name" => Some(&mut self.name),
            _ => None,
        }
    }
}
