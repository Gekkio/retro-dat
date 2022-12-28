// SPDX-FileCopyrightText: 2019-2022 Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::{ForceMerging, ForceNoDump, ForcePacking, RomMode, SampleMode, Status};

pub trait XmlAttr {
    fn set_from_str(&mut self, _: &str) -> bool {
        false
    }
}

impl XmlAttr for String {
    fn set_from_str(&mut self, value: &str) -> bool {
        self.clear();
        self.push_str(value);
        true
    }
}

impl XmlAttr for bool {
    fn set_from_str(&mut self, value: &str) -> bool {
        match value {
            "yes" => *self = true,
            "no" => *self = false,
            _ => return false,
        }
        true
    }
}

impl XmlAttr for ForceMerging {
    fn set_from_str(&mut self, value: &str) -> bool {
        match value {
            "none" => *self = ForceMerging::None,
            "split" => *self = ForceMerging::Split,
            "full" => *self = ForceMerging::Full,
            _ => return false,
        }
        true
    }
}

impl XmlAttr for ForceNoDump {
    fn set_from_str(&mut self, value: &str) -> bool {
        match value {
            "obsolete" => *self = ForceNoDump::Obsolete,
            "required" => *self = ForceNoDump::Required,
            "ignore" => *self = ForceNoDump::Ignore,
            _ => return false,
        }
        true
    }
}

impl XmlAttr for ForcePacking {
    fn set_from_str(&mut self, value: &str) -> bool {
        match value {
            "zip" => *self = ForcePacking::Zip,
            "unzip" => *self = ForcePacking::Unzip,
            _ => return false,
        }
        true
    }
}

impl XmlAttr for RomMode {
    fn set_from_str(&mut self, value: &str) -> bool {
        match value {
            "merged" => *self = RomMode::Merged,
            "split" => *self = RomMode::Split,
            "unmerged" => *self = RomMode::Unmerged,
            _ => return false,
        }
        true
    }
}

impl XmlAttr for SampleMode {
    fn set_from_str(&mut self, value: &str) -> bool {
        match value {
            "merged" => *self = SampleMode::Merged,
            "unmerged" => *self = SampleMode::Unmerged,
            _ => return false,
        }
        true
    }
}

impl XmlAttr for Status {
    fn set_from_str(&mut self, value: &str) -> bool {
        match value {
            "baddump" => *self = Status::BadDump,
            "nodump" => *self = Status::NoDump,
            "good" => *self = Status::Good,
            "verified" => *self = Status::Verified,
            _ => return false,
        }
        true
    }
}
