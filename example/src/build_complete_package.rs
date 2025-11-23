use std::fs::{self, File};
use std::io::{Write, BufReader, BufWriter};
use std::path::Path;
use std::process::Command;
use zip::write::FileOptions;
use zip::{ZipWriter, CompressionMethod};
use serde::{Serialize};

/// Plugin manifest structure
#[derive(Serialize)]
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
    println!("Building complete plugin package...");
    
    // Create a temporary directory for building
    let temp_dir = tempfile::TempDir::new()?;
    let temp_path = temp_dir.path();
    
    // Compile the plugin to a shared library
    println!("  Compiling plugin...");
    let output = Command::new("cargo")
        .args(&["build", "--release"])
        .current_dir("..")
        .output()?;
    
    if !output.status.success() {
        eprintln!("Failed to compile plugin");
        eprintln!("stdout: {}", String::from_utf8_lossy(&output.stdout));
        eprintln!("stderr: {}", String::from_utf8_lossy(&output.stderr));
        return Err("Failed to compile plugin".into());
    }
    
    // Determine the library file name based on the platform
    let library_name = if cfg!(target_os = "windows") {
        "example.dll"
    } else if cfg!(target_os = "macos") {
        "libexample.dylib"
    } else {
        "libexample.so"
    };
    
    // Find the compiled library
    let target_dir = Path::new("../target/release");
    let library_path = target_dir.join(library_name);
    
    if !library_path.exists() {
        return Err(format!("Compiled library not found: {:?}", library_path).into());
    }
    
    // Copy the library to the temp directory
    let plugin_library_path = temp_path.join(library_name);
    fs::copy(&library_path, &plugin_library_path)?;
    println!("  Copied library: {:?}", plugin_library_path);
    
    // Create the manifest
    let manifest = PluginManifest {
        name: "complete_example".to_string(),
        version: "1.0.0".to_string(),
        description: "A complete example plugin for PubChat".to_string(),
        main: library_name.to_string(),
        author: Some("PubChat Team".to_string()),
        repository: Some("https://github.com/pubchat/pubchat".to_string()),
        display_name: Some("Complete Example Plugin".to_string()),
        categories: Some(vec!["example".to_string(), "demo".to_string()]),
        keywords: Some(vec!["example".to_string(), "demo".to_string(), "complete".to_string()]),
        license: Some("MIT".to_string()),
        homepage: Some("https://pubchat.example.com".to_string()),
    };
    
    let manifest_path = temp_path.join("manifest.json");
    let manifest_file = File::create(&manifest_path)?;
    serde_json::to_writer_pretty(manifest_file, &manifest)?;
    println!("  Created manifest: {:?}", manifest_path);
    
    // Create the plugin package (.pubchat file)
    let package_path = Path::new("complete_example.pubchat");
    let package_file = File::create(&package_path)?;
    let mut zip = ZipWriter::new(BufWriter::new(package_file));
    
    // Add manifest.json to the package
    // zip.start_file("manifest.json", FileOptions::default().compression_method(CompressionMethod::Deflated))?;
    let manifest_content = std::fs::read(&manifest_path)?;
    zip.write_all(&manifest_content)?;
    println!("  Added manifest to package");
    
    // Add plugin library to the package
    // zip.start_file(library_name, FileOptions::default().compression_method(CompressionMethod::Deflated))?;
    let library_content = std::fs::read(&plugin_library_path)?;
    zip.write_all(&library_content)?;
    println!("  Added library to package");
    
    // Finish creating the package
    zip.finish()?;
    println!("  Package finalized");
    
    println!("\nPlugin package created successfully: {:?}", package_path);
    println!("\nTo use this package:");
    println!("1. Copy the .pubchat file to the client's plugins directory");
    println!("2. Start the PubChat client");
    println!("3. The plugin will be automatically loaded");
    
    Ok(())
}