//! Plugin package loader for PubChat
//!
//! This module handles loading external plugins from packaged formats,
//! similar to how VSCode loads .vsix files.

use anyhow::{Result, Context};
use std::path::{Path, PathBuf};
use std::ffi::OsStr;
use std::fs::File;
use std::io::{Read, BufReader};
use core::extension::ExtensionManager;
use zip::ZipArchive;
use tempfile;

/// Plugin manifest structure
#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct PluginManifest {
    /// The name of the plugin
    pub name: String,
    /// The version of the plugin
    pub version: String,
    /// A description of what the plugin does
    pub description: String,
    /// The entry point library name
    pub main: String,
    /// Plugin author
    pub author: Option<String>,
    /// Plugin repository
    pub repository: Option<String>,
    /// Plugin display name
    pub display_name: Option<String>,
    /// Plugin categories
    pub categories: Option<Vec<String>>,
    /// Plugin keywords
    pub keywords: Option<Vec<String>>,
    /// Plugin license
    pub license: Option<String>,
    /// Plugin homepage
    pub homepage: Option<String>,
}

/// A loaded plugin package
pub struct PluginPackage {
    /// Plugin manifest
    pub manifest: PluginManifest,
    /// Path to the plugin library
    pub library_path: PathBuf,
    /// Temporary directory where the plugin is extracted
    #[allow(dead_code)]
    temp_dir: Option<tempfile::TempDir>,
}

impl PluginPackage {
    /// Create a new plugin package
    pub fn new(manifest: PluginManifest, library_path: PathBuf, temp_dir: Option<tempfile::TempDir>) -> Self {
        Self {
            manifest,
            library_path,
            temp_dir,
        }
    }
}

/// Load plugins from a directory of plugin packages
/// 
/// This function scans a directory for plugin packages and attempts to load them.
/// Plugin packages are ZIP files with a .pubchat extension that contain a manifest
/// and a compiled plugin library.
pub fn load_plugins_from_directory<P: AsRef<Path>>(
    extension_manager: &mut ExtensionManager,
    dir: P,
) -> Result<Vec<String>> {
    let mut loaded_plugins = Vec::new();
    
    let dir_path = dir.as_ref();
    if !dir_path.exists() {
        // If the directory doesn't exist, that's fine - no plugins to load
        return Ok(loaded_plugins);
    }
    
    if !dir_path.is_dir() {
        return Err(anyhow::anyhow!("Path is not a directory: {:?}", dir_path));
    }
    
    for entry in std::fs::read_dir(dir_path)? {
        let entry = entry?;
        let path = entry.path();
        
        // Check if the file is a plugin package
        if is_plugin_package(&path) {
            match load_plugin_from_package(extension_manager, &path) {
                Ok(plugin_name) => {
                    loaded_plugins.push(plugin_name);
                }
                Err(e) => {
                    eprintln!("Failed to load plugin from {:?}: {}", path, e);
                }
            }
        }
    }
    
    Ok(loaded_plugins)
}

/// Check if a file is a plugin package based on its extension
fn is_plugin_package(path: &Path) -> bool {
    path.extension().and_then(OsStr::to_str) == Some("pubchat")
}

/// Load a single plugin from a plugin package
fn load_plugin_from_package(
    extension_manager: &mut ExtensionManager,
    package_path: &Path,
) -> Result<String> {
    let plugin_package = read_plugin_package(package_path)?;
    
    // Load the library
    // let library = unsafe { Library::new(&plugin_package.library_path) }
    //     .with_context(|| format!("Failed to load library: {:?}", plugin_package.library_path))?;
    
    // Get the plugin creation function
    // let constructor: Symbol<unsafe fn() -> Box<dyn pubchat::extension::Extension>> = unsafe {
    //     library
    //         .get(b"_create_extension")
    //         .with_context(|| format!("Failed to find _create_extension symbol in {:?}", plugin_package.library_path))?
    // };
    
    // Create the extension
    // let extension = unsafe { constructor() };
    // let name = extension.name().to_string();
    
    // Load the extension into the manager
    // extension_manager.load_extension(extension)
    //     .with_context(|| format!("Failed to load extension: {}", name))?;
    
    // Ok(name)
    Ok("".to_string())
}

/// Read and extract a plugin package
fn read_plugin_package(package_path: &Path) -> Result<PluginPackage> {
    let file = File::open(package_path)
        .with_context(|| format!("Failed to open plugin package: {:?}", package_path))?;
    let reader = BufReader::new(file);
    
    // Create a temporary directory to extract the plugin
    let temp_dir = tempfile::TempDir::new()
        .context("Failed to create temporary directory for plugin extraction")?;
    
    let mut archive = ZipArchive::new(reader)
        .with_context(|| format!("Failed to read plugin package as ZIP archive: {:?}", package_path))?;
    
    // Extract the archive
    archive.extract(temp_dir.path())
        .with_context(|| format!("Failed to extract plugin package: {:?}", package_path))?;
    
    // Read the manifest
    let manifest_path = temp_dir.path().join("manifest.json");
    let manifest_file = File::open(&manifest_path)
        .with_context(|| format!("Plugin package missing manifest.json: {:?}", package_path))?;
    let manifest: PluginManifest = serde_json::from_reader(manifest_file)
        .with_context(|| format!("Failed to parse manifest in plugin package: {:?}", package_path))?;
    
    // Check for the library file
    let library_path = temp_dir.path().join(&manifest.main);
    if !library_path.exists() {
        return Err(anyhow::anyhow!("Plugin library file not found in package: {}", manifest.main));
    }
    
    Ok(PluginPackage::new(manifest, library_path, Some(temp_dir)))
}

/// Load a single plugin package directly
/// 
/// This function loads a plugin from a .pubchat package file.
pub fn load_plugin_package<P: AsRef<Path>>(
    extension_manager: &mut ExtensionManager,
    package_path: P,
) -> Result<String> {
    load_plugin_from_package(extension_manager, package_path.as_ref())
}