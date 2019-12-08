use std::{
    env,
    fs,
    io::Write,
    path::{Path, PathBuf},
};

use chrono::prelude::*;
use slug::slugify;

use errors::Result;
use front_matter::PageFrontMatter;
use site::Site;

mod config {
    use clap::ArgMatches;
    pub struct NewPageConfig {
        pub name: String,
        pub section: Option<String>,
    }

    impl<'a> std::convert::TryFrom<&'a ArgMatches<'_>> for NewPageConfig {
        type Error = String;

        fn try_from(matches: &ArgMatches) -> Result<Self, Self::Error> {
            Ok(NewPageConfig {
                name: matches
                    .value_of("name")
                    .map(Into::into)
                    .ok_or_else(|| String::from("Name is missing, please pass --name"))?,
                section: matches.value_of("section").map(Into::into),
            })
        }
    }
}

pub use self::config::NewPageConfig;

pub fn new_page(config_file: &str, config: NewPageConfig) -> Result<()> {
    let site = Site::new(env::current_dir().unwrap(), config_file)?;

    let NewPageConfig { name, section } = config;

    let now = Local::now();
    let title = name.clone();
    let folder_name = slugify(
        now.naive_local().date().to_string()
        + " "
        + &name);
    let section = section.map(slugify);

    let content_path: PathBuf = match section {
        Some(section) => site.get_content_path().join(section),
        None => site.get_content_path().to_owned(),
    };

    let front_matter = dbg!(PageFrontMatter {
        title: Some(title.clone()),
        datetime: Some(now.naive_local()),
        ..PageFrontMatter::default()
    });

    let target_folder = dbg!(content_path.join(folder_name));

    let content = dbg!(format!("+++\n{header}\n+++\n\n# {title}\n\n...", 
        header = front_matter.to_toml(),
        title = title,
    ));

    let target_file = create_page(&target_folder)?;

    println!("writing {} to {:?}", content, target_file);
    write_page(&target_file, &content);

    Ok(())
}

fn create_page(target_folder: &Path) -> std::result::Result<PathBuf, std::io::Error>{
    if dbg!(target_folder.exists()) {
        Err(std::io::Error::from(std::io::ErrorKind::AlreadyExists))
    } else {
        fs::create_dir(target_folder)?;
        let target_file = dbg!(target_folder.join("index.md"));
        Ok(dbg!(target_file))
    }
}

fn write_page(target_file: &Path, content: &str) {
    if dbg!(target_file.exists()) {
        println!("{:?} already exists, can't overwrite", target_file);
    } else if let Ok(mut file) = fs::File::create(target_file) {
        for line in content.lines() {
            file.write_fmt(format_args!("{}\n", line))
                .expect("cannot write this line to the config file");
        }
    }
}
