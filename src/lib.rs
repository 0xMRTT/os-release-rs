use std::collections::BTreeMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::iter::FromIterator;
use std::path::Path;


macro_rules! map_keys {
    ($item:expr, { $($pat:expr => $field:expr),+ }) => {{
        $(
            if $item.starts_with($pat) {
                $field = parse_line($item, $pat.len()).into();
                continue;
            }
        )+
    }};
}

fn is_enclosed_with(line: &str, pattern: char) -> bool {
    line.starts_with(pattern) && line.ends_with(pattern)
}

fn parse_line(line: &str, skip: usize) -> &str {
    let line = line[skip..].trim();
    if is_enclosed_with(line, '"') || is_enclosed_with(line, '\'') {
        &line[1..line.len() - 1]
    } else {
        line
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct OsRelease {
    /// ANSI color code for the distribution.
    /// This is a six numbers.
    /// For example, on ArchLinux, this is "38;2;23;147;209.
    pub ansi_color:         String,
    /// If the distro is a rolling release, it will be "rolling".
    pub build_id:           String,
    /// Url of bug reporting system.
    /// This is the URL of the bug reporting system for the distribution.
    /// For example, on ArchLinux, this is "https://bugs.archlinux.org".
    pub bug_report_url:     String,
    /// Url of the documentation for the distribution.
    /// This is the URL of the documentation for the distribution.
    /// For example, on ArchLinux, this is "https://wiki.archlinux.org".
    /// The ArchWiki is the biggest documentation of every open source project.
    /// This is not the same as the URL of the distribution's website.
    pub documentation_url:  String,
    /// Extra keys will be stored in this map.
    pub extra:              BTreeMap<String, String>,
    /// Homepage of the distribution.
    /// This is the homepage of the distribution.
    /// For example, on ArchLinux, this is "https://www.archlinux.org/".
    pub home_url:           String,
    /// The name of the distribution in the form of a codename.
    /// For example, on ArchLinux, this is "archlinux".
    pub id:                 String,
    /// Related distribution id.
    /// If the distro is derived from another distro, it will be the id of the parent distro.
    /// For example, on Manjaro, this is "arch".
    pub id_like:            String,
    /// The name of the operating system.
    /// This is the name of the operating system as it appears to the user.
    /// For example, on ArchLinux, this is "Arch Linux".
    pub logo:               String,
    /// Logo of the distribution.
    /// This is the logo of the distribution.
    /// For example, on ArchLinux, this is "archlinux-logo".
    pub name:               String,
    /// The pretty name of the operating system.
    pub pretty_name:        String,
    /// Privacy policy url.
    /// This is the URL of the privacy policy of the distribution.
    /// For example, on ArchLinux, this is "https://www.archlinux.org/legal/privacy-policy/".
    pub privacy_policy_url: String,
    /// The version of the distribution.
    /// This is the version of the distribution.
    /// For example, on ArchLinux, this is "" because ArchLinux is rolling release so ArchLinux doesn't have version.
    pub version:            String,
    /// The version codename of the distribution.
    /// This is the version codename of the distribution.
    /// For example, on ArchLinux, this is "" because ArchLinux is rolling release so ArchLinux doesn't have version.
    pub version_codename:   String,
    /// The version id of the distribution.
    /// This is the version id of the distribution.
    /// For example, on ArchLinux, this is "" because ArchLinux is rolling release so ArchLinux doesn't have version.
    pub version_id:         String,
    /// The support url of the distribution.
    /// This is the support url of the distribution.
    /// For example, on ArchLinux, this is "https://bbs.archlinux.org/"
    pub support_url:        String,
}

impl OsRelease {
    /// Reads the `/etc/os-release` file and returns a `OsRelease` struct.
    /// If `/etc/os-release` does not exist, searches for `/usr/lib/os-release`
    pub fn new() -> io::Result<OsRelease> {
        let file = BufReader::new(open("/etc/os-release").unwrap_or(open("/usr/lib/os-release")?));
        Ok(OsRelease::from_iter(file.lines().flat_map(|line| line)))
    }

    /// Attempt to parse any `/etc/os-release`-like file.
    pub fn new_from<P: AsRef<Path>>(path: P) -> io::Result<OsRelease> {
        let file = BufReader::new(open(&path)?);
        Ok(OsRelease::from_iter(file.lines().flat_map(|line| line)))
    }
}

impl FromIterator<String> for OsRelease {
    fn from_iter<I: IntoIterator<Item = String>>(lines: I) -> Self {
        let mut os_release = Self::default();

        for line in lines {
            let line = line.trim();
            map_keys!(line, {
                "ANSI_COLOR=" => os_release.ansi_color,
                "BUILD_ID=" => os_release.build_id,
                "BUG_REPORT_URL=" => os_release.bug_report_url,
                "DOCUMENTATION_URL=" => os_release.documentation_url,
                "HOME_URL=" => os_release.home_url,
                "ID=" => os_release.id,
                "ID_LIKE=" => os_release.id_like,
                "LOGO=" => os_release.logo,
                "NAME=" => os_release.name,
                "PRETTY_NAME=" => os_release.pretty_name,
                "PRIVACY_POLICY_URL=" => os_release.privacy_policy_url,
                "SUPPORT_URL=" => os_release.support_url,
                "VERSION=" => os_release.version,
                "VERSION_ID=" => os_release.version_id,
                "VERSION_CODENAME=" => os_release.version_codename
            });

            if let Some(pos) = line.find('=') {
                if line.len() > pos+1 {
                    os_release.extra.insert(line[..pos].to_owned(), line[pos+1..].to_owned());
                }
            }
        }

        os_release
    }
}

fn open<P: AsRef<Path>>(path: P) -> io::Result<File> {
    File::open(&path).map_err(|why| io::Error::new(
        io::ErrorKind::Other,
        format!("unable to open file at {:?}: {}", path.as_ref(), why)
    ))
}
#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"NAME="Arch Linux"
PRETTY_NAME="Arch Linux"
ID=arch
BUILD_ID=rolling
ANSI_COLOR="38;2;23;147;209"
HOME_URL="https://archlinux.org/"
DOCUMENTATION_URL="https://wiki.archlinux.org/"
SUPPORT_URL="https://archlinux.org/"
BUG_REPORT_URL="https://bugs.archlinux.org/"
LOGO=archlinux-logo
EXTRA_KEY=thing"#;

    #[test]
    fn os_release() {
        let os_release = OsRelease::from_iter(EXAMPLE.lines().map(|x| x.into()));

        assert_eq!(
            os_release,
            OsRelease {
                name:               "Arch Linux".into(),
                pretty_name:        "Arch Linux".into(),
                version:            "".into(),
                id:                 "arch".into(),
                id_like:            "".into(),
                version_id:         "".into(),
                home_url:           "https://archlinux.org/".into(),
                support_url:        "https://archlinux.org/".into(),
                bug_report_url:     "https://bugs.archlinux.org/".into(),
                privacy_policy_url: "".into(),
                version_codename:   "".into(),
                logo:               "archlinux-logo".into(),
                build_id:           "rolling".into(),
                ansi_color:         "38;2;23;147;209".into(),
                documentation_url:   "https://wiki.archlinux.org/".into(),
                extra: {
                    let mut map = BTreeMap::new();
                    map.insert("EXTRA_KEY".to_owned(), "thing".to_owned());
                    map
                }
            }
        )
    }
}
