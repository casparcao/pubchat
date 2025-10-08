use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Page{
    pub ps: u32,//页大小
    pub pn: u32,//页码
    pub fd: Option<String>,//过滤字段
    pub dir: Option<String>,//过滤方向
}

impl Page{
    pub fn offset(&self) -> u32 {
        if self.pn <= 0 {
            return 0;
        }
        (self.pn - 1) * self.ps
    }

    pub fn fd(&self) -> String {
        self.fd.as_ref().unwrap_or(&"".to_string()).to_string()
    }

    pub fn dir(&self) -> String {
        self.dir.as_ref().unwrap_or(&"ASC".to_string()).to_string()
    }
}

impl Default for Page {
    fn default() -> Self {
        Page { ps: 10u32, pn: 1u32, fd: None, dir: None }
    }
}