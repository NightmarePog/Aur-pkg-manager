use std::{
    fmt::{Display, Write as _},
    fs,
    path::{Path, PathBuf},
};

use alpm::{Alpm, LoadedPackage, SigLevel};

use crate::build::BuildError;

pub struct Database<'a>(&'a Path);


pub struct Artifact<'a>(&'a Path);

impl<'a> Database<'a> {

    pub const fn new(path: &'a Path) -> Self {
        Self(path)
    }


    pub fn push(&self, artifact: Artifact<'_>) -> Result<(), BuildError> {
        artifact.load(self, |package| self.register(package))
    }

    fn register(&self, package: &LoadedPackage<'_>) -> Result<(), BuildError> {
        let package_directory = self.package_directory(&package);

        fs::create_dir_all(&package_directory)?;
        PackageDescription::from(package).write_description_file(&package_directory)?;
        self.write_package_file_list(&package_directory, package)
    }

    fn package_directory(&self, package: &LoadedPackage<'_>) -> PathBuf {
        self.0
            .join("local")
            .join(format!("{}-{}", package.name(), package.version()))
    }

    fn write_package_file_list(
        &self,
        package_directory: &Path,
        package: &LoadedPackage<'_>,
    ) -> Result<(), BuildError> {
        let mut file_list = String::from("%FILES%\n");

        package.files().files().iter().for_each(|file| {
            let file_name = String::from_utf8_lossy(file.name());
            if is_package_file_entry(&file_name) {
                file_list.push_str(file_name.trim_start_matches("./"));
                file_list.push('\n');
            }
        });

        file_list.push('\n');
        fs::write(package_directory.join("files"), file_list)?;
        Ok(())
    }
}

impl<'a> Artifact<'a> {
    pub const fn new(path: &'a Path) -> Self {
        Self(path)
    }
}

impl Artifact<'_> {
    fn load<T>(
        &self,
        database: &Database<'_>,
        process: impl for<'b> FnOnce(&LoadedPackage<'b>) -> Result<T, BuildError>,
    ) -> Result<T, BuildError> {
        let alpm = Alpm::new("/".to_owned(), database.0.to_string_lossy().into_owned())?;
        let package = alpm.pkg_load(self.0.to_string_lossy().into_owned(), true, SigLevel::NONE)?;
        process(&package)
    }
}

fn is_package_file_entry(file_name: &str) -> bool {
    !file_name.ends_with('/')
        && (!file_name.starts_with('.') || file_name.starts_with("./"))
        && !matches!(file_name, ".BUILDINFO" | ".MTREE" | ".PKGINFO" | ".INSTALL")
}

struct PackageDescription {
    content: String,
}

impl PackageDescription {
    fn new() -> Self {
        Self {
            content: String::with_capacity(1024),
        }
    }

    fn write_field<T: Display>(mut self, name: &str, values: impl IntoIterator<Item = T>) -> Self {
        let _ = writeln!(self.content, "%{name}%");
        values.into_iter().for_each(|value| {
            let _ = writeln!(self.content, "{value}");
        });
        self.content.push('\n');
        self
    }

    fn write_description_file(self, package_directory: &Path) -> Result<(), BuildError> {
        fs::write(package_directory.join("desc"), self.content)?;
        Ok(())
    }
}

impl<'a> From<&LoadedPackage<'a>> for PackageDescription {
    fn from(package: &LoadedPackage<'a>) -> Self {
        Self::new()
            .write_field("NAME", [package.name()])
            .write_field("VERSION", [package.version()])
            .write_field("BASE", [package.base().unwrap_or(package.name())])
            .write_field("DESC", [package.desc().unwrap_or_default()])
            .write_field("URL", [package.url().unwrap_or_default()])
            .write_field("ARCH", [package.arch().unwrap_or_default()])
            .write_field("BUILDDATE", [package.build_date()])
            .write_field("INSTALLDATE", [package.build_date()])
            .write_field("PACKAGER", [package.packager().unwrap_or_default()])
            .write_field("SIZE", [package.size()])
            .write_field("REASON", [0])
            .write_field("LICENSE", package.licenses())
            .write_field("VALIDATION", ["none"])
            .write_field("DEPENDS", package.depends())
            .write_field("OPTDEPENDS", package.optdepends())
            .write_field("PROVIDES", package.provides())
    }
}
