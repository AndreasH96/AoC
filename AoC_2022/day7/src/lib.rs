use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct FolderFile {
    pub name: String,
    pub size: usize,
}
#[derive(Debug, Clone)]
pub struct Folder {
    pub name: String,
    pub sub_folders_names: Vec<String>,
    pub files: Vec<FolderFile>,
}

impl FolderFile {
    pub fn new(raw_string: String) -> FolderFile {
        let split: Vec<&str> = raw_string.split(" ").collect();
        let size: usize = split[0].parse().unwrap();
        let name = split[1];
        return FolderFile {
            name: name.to_owned(),
            size: size.to_owned(),
        };
    }
}
impl Folder {
    pub fn new(name: String, sub_folders: Vec<String>, files: Vec<FolderFile>) -> Folder {
        Folder {
            name: name,
            sub_folders_names: sub_folders,
            files: files,
        }
    }
    pub fn get_size(&self) -> usize {
        return self.files.iter().map(|f| f.size).sum();
    }
    pub fn get_size_recur(&self, files_map: &HashMap<String, Folder>) -> usize {
        let mut size = self.get_size(); 
        
        let sub_folders = self.sub_folders_names.clone(); 
        for sub in sub_folders {
            let s = files_map.get(&sub).unwrap();
     
            size += s.get_size_recur(files_map); 
        }
        return size;
    }
}
