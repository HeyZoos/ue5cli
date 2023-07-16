use crate::my_format::MyFormat;
use derive_builder::Builder;

#[derive(Builder)]
pub struct ThirdPartyLibraryDetails {
    prefix_dirs: Vec<String>,
    include_dirs: Vec<String>,
    link_dirs: Vec<String>,
    libs: Vec<String>,
    system_libs: Vec<String>,
    definitions: Vec<String>,
    cxx_flags: Vec<String>,
    ld_flags: Vec<String>,
    cmake_flags: Vec<String>,
    definition_prefix: String,
    include_dir_prefix: String,
    linker_dir_prefix: String,
    system_lib_prefix: String,
}

impl ThirdPartyLibraryDetails {
    pub fn merge(&mut self, other: ThirdPartyLibraryDetails) {
        self.prefix_dirs.append(&mut other.prefix_dirs.clone());
        self.include_dirs.append(&mut other.include_dirs.clone());
        self.link_dirs.append(&mut other.link_dirs.clone());
        self.libs.append(&mut other.libs.clone());
        self.system_libs.append(&mut other.system_libs.clone());
        self.definitions.append(&mut other.definitions.clone());
        self.cxx_flags.append(&mut other.cxx_flags.clone());
        self.ld_flags.append(&mut other.ld_flags.clone());
        self.cmake_flags.append(&mut other.cmake_flags.clone());
    }

    pub fn get_compiler_flags(&self, engine_root: &str, fmt: &MyFormat) -> String {
        let mut all_strings = Vec::new();
        all_strings.extend(self.prefixed_strings(
            &self.definition_prefix,
            &self.definitions,
            engine_root,
        ));
        all_strings.extend(self.prefixed_strings(
            &self.include_dir_prefix,
            &self.include_dirs,
            engine_root,
        ));
        all_strings.extend(self.resolve_root(&self.cxx_flags, engine_root));
        all_strings.join(&fmt.delim)
    }

    pub fn get_linker_flags(
        &self,
        engine_root: &str,
        fmt: &MyFormat,
        include_libs: bool,
    ) -> String {
        let mut components = self.resolve_root(&self.ld_flags, engine_root);
        if include_libs {
            components.extend(self.prefixed_strings(
                &self.linker_dir_prefix,
                &self.link_dirs,
                engine_root,
            ));
            components.extend(self.resolve_root(&self.libs, engine_root));
            components.extend(self.prefixed_strings(
                &self.system_lib_prefix,
                &self.system_libs,
                engine_root,
            ));
        }
        components.join(&fmt.delim)
    }

    fn resolve_root(&self, paths: &Vec<String>, engine_root: &str) -> Vec<String> {
        paths
            .iter()
            .map(|p| p.replace("%UE4_ROOT%", engine_root))
            .collect()
    }

    fn prefixed_strings(
        &self,
        prefix: &str,
        paths: &Vec<String>,
        engine_root: &str,
    ) -> Vec<String> {
        let resolved = self.resolve_root(paths, engine_root);
        resolved
            .iter()
            .map(|s| format!("{}{}", prefix, s))
            .collect()
    }
}
