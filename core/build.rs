// client/build.rs

use std::io::Result;

fn main() -> Result<()> {
    // 配置 prost-build
    let mut config = prost_build::Config::new();

    // 可选：设置输出目录
    config.out_dir("src/proto");

    // 可选：为所有类型添加 derive
    config.type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]");
    config.type_attribute(".", "#[serde(rename_all = \"snake_case\")]");

    // 可选：重命名消息或包
    // config.rename("MyMessage", "RenamedMessage");

    // 编译 proto 文件
    config.compile_protos(
    &["proto/message.proto"],  // proto 文件列表
    &["proto"],  // proto 文件所在目录列表
    )?;
    Ok(())
}