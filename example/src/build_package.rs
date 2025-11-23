//! Example script to build a plugin package
//!
//! This script demonstrates how to package a plugin into the .pubchat format.

use std::fs::{self, File};
use std::io::{Write, BufReader, BufWriter};
use std::path::Path;
use zip::{ZipWriter, CompressionMethod};

/// Plugin manifest structure matching the one in the loader
#[derive(serde::Serialize)]
struct PluginManifest {
    name: String,
    version: String,
    description: String,
    main: String,
    author: Option<String>,
    repository: Option<String>,
    display_name: Option<String>,
    categories: Option<Vec<String>>,
    keywords: Option<Vec<String>>,
    license: Option<String>,
    homepage: Option<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create the manifest for our example plugin
    let manifest = PluginManifest {
        name: "example_plugin".to_string(),
        version: "1.0.0".to_string(),
        description: "An example plugin for PubChat".to_string(),
        main: "plugin.dll".to_string(), // This would be the compiled plugin library
        author: Some("PubChat Developers".to_string()),
        repository: Some("https://github.com/pubchat/pubchat".to_string()),
        display_name: Some("Example Plugin".to_string()),
        categories: Some(vec!["example".to_string(), "demo".to_string()]),
        keywords: Some(vec!["example".to_string(), "demo".to_string()]),
        license: Some("MIT".to_string()),
        homepage: Some("https://pubchat.example.com".to_string()),
    };

    // Create a temporary directory for building the package
    let temp_dir = tempfile::TempDir::new()?;
    let temp_path = temp_dir.path();
    
    // Write the manifest to a file
    let manifest_path = temp_path.join("manifest.json");
    let manifest_file = File::create(&manifest_path)?;
    serde_json::to_writer_pretty(manifest_file, &manifest)?;
    
    // Create a dummy plugin library file
    let library_path = temp_path.join("plugin.dll");
    let mut library_file = File::create(&library_path)?;
    library_file.write_all(b"This is a dummy plugin library file")?;
    
    // Create the plugin package (.pubchat file)
    let package_path = Path::new("example_plugin.pubchat");
    let package_file = File::create(&package_path)?;
    let mut zip = ZipWriter::new(BufWriter::new(package_file));
    
    // Add files to the ZIP archive
    zip.start_file("manifest.json", CompressionMethod::Deflated)?;
    let manifest_content = std::fs::read(&manifest_path)?;
    zip.write_all(&manifest_content)?;
    
    zip.start_file("plugin.dll", CompressionMethod::Deflated)?;
    let library_content = std::fs::read(&library_path)?;
    zip.write_all(&library_content)?;
    
    // Finish creating the ZIP archive
    zip.finish()?;
    
    println!("Plugin package created: {:?}", package_path);
    
    Ok(())
}