
use std::{
    env, fs, io::{Read, Write}, path::Path
};


pub fn makeall()
{
    let spath = SourceFiles::exe_dir();
    makedir(&format!("{}c3dlc", &spath));
    makedir(&format!("{}c3dlc/pics", &spath));
    makedir(&format!("{}c3dlc/source", &spath));

}


fn makedir(name: &str) 
{
    let filepath = Path::new(name);
    if !filepath.is_dir() {
        let _ = fs::create_dir(filepath);
    }
}


pub struct SourceFiles 
{
    pub current_path:   String,
    path_vector:        Vec<String>,
}


impl SourceFiles 
{
    pub fn new() -> Self 
    {
        let mut path_vector: Vec<String> = Vec::new();
        let source_path: String;

        source_path = format!("{}c3dlc/source/", &Self::exe_dir());

        let entries = fs::read_dir(&source_path).unwrap();
        for entry in entries {
            path_vector.push(format!("{}{}",
                source_path,
                entry.unwrap()
                    .file_name()
                    .into_string()
                    .unwrap()
                )
            );
        }

        
        path_vector.sort();
        Self {
            path_vector: path_vector.clone(),
            current_path: {
                match path_vector.first() {
                    Some(first_element) => String::from(first_element),
                    None => String::from("None"),
                }
            }
        }
    }


    pub fn update(&mut self)
    {
        let source_path = format!("{}c3dlc/source/", &Self::exe_dir()); 
        self.path_vector = Vec::new();
        let entries = fs::read_dir(&source_path).unwrap();
        for entry in entries {
            self.path_vector.push(format!("{}{}", 
                source_path, 
                entry.unwrap()
                   .file_name()
                   .into_string()
                   .unwrap()
                )
            );
        }
        self.path_vector.sort();
        if self.current_path == String::from("None") {
            self.current_path = match self.path_vector.first() {
                Some(path) => path.to_string(),
                None => return,
            }
        }
    }


    pub fn get_content(&self) -> String
    {
        let mut openresult = match fs::File::open(&self.current_path) {
            Ok(file) => file,
            Err(e) => {
                eprintln!(
                    "Error with opening colors.txt file. Path is {}. {}",
                    &self.current_path, e
                );
                return String::new();
            }
        };
        let mut content = String::new();
        if let Err(e) = openresult.read_to_string(&mut content) {
            eprintln!("Error with exporting data from file. {}", e);
            return String::from("");
        }
        return content;
    }


    pub fn savef(&self, content: &String)
    {
        let mut file = match fs::File::create(&self.current_path) {
            Ok(file) => file,
            Err(e) => {
                eprintln!("Error with opening file to write. {}", e);
                return;
            }
        };
        if let Err(e) = file.write_all(content.as_bytes()) {
            eprintln!("error with writing data to file. {}", e);
            return;
        }
    }


    pub fn nextf(&mut self) 
    {
        Self::update(self);
        for i in 0..self.path_vector.len() {
            if self.current_path == self.path_vector[i] {
                if i != (self.path_vector.len() - 1) {
                    self.current_path = self.path_vector[(i + 1) as usize].clone();
                    return;
                } else {
                    self.current_path = self.path_vector[0].clone();
                    return;
                }
            }
        }
    }


    pub fn prevf(&mut self)
    {
        Self::update(self);
        for i in 0..self.path_vector.len() {
            if self.current_path == self.path_vector[i] {
                if i != 0 {
                    self.current_path = self.path_vector[(i - 1) as usize].clone();
                    return;
                } else {
                    self.current_path =
                        self.path_vector[(self.path_vector.len() - 1) as usize].clone();
                    return;
                }
            }
        }
    }


    pub fn relativepath(&self) -> String
    {
        if let Ok(exe_dir) = env::current_exe() {
            if let Some(path) = exe_dir.to_str() {
                let source_path: String;
                source_path = format!("{}c3dlc/", path.replace(&Self::app_name(), ""));
                format!("{}", self.current_path.replace(&source_path, ""))
            } else {
                "Some errors with relative path module".to_string()
            }
        } else {
            "Some errors with relative path module".to_string()
        }
    }


    pub fn app_name() -> String 
    {
        env::current_exe()
            .unwrap()
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string()
    }


    pub fn exe_dir() -> String
    {
        format!("{}", env::current_exe()
            .unwrap()
            .to_str()
            .unwrap()
            .replace(&Self::app_name(), ""))
    }
}


