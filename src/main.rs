use std::path::PathBuf;

use clap::Parser;

use stdirgen::{cli::EntityType, create_dir, create_file, create_subfiles, Cli};

fn main() {
    let cli = Cli::parse();
    match &cli.entity_type {
        EntityType::Directory(args) => {
            let entity_type = EntityType::Directory(args.clone());
            let dir_name: &str = entity_type.get_name();
            let path: PathBuf = entity_type.get_path();
            let dir_iter: &i32 = entity_type.get_iter();
            let dir_start: &i32 = entity_type.get_start();
            let subfile_commands: Option<stdirgen::cli::SubfileCommands<'_>> =
                entity_type.get_subfile_commands();

            let enumer_dirs: Vec<String> =
                create_dir::create_dir(dir_name, &path, dir_iter, dir_start).unwrap();

            if let Some(command) = subfile_commands {
                let files: &Vec<String> = command.files;
                let files_iter: &Vec<i32> = command.files_iter;
                let files_start: &Vec<i32> = command.files_start;

                for directory in enumer_dirs {
                    let mut dir_path = path.clone();
                    dir_path.push(directory);

                    create_subfiles::create_subfiles(files, files_iter, files_start, &dir_path)
                        .unwrap();
                }
            }

            println!("The values of directory command is {:?}", dir_name)
        }

        EntityType::File(args) => {
            let entity_type: EntityType = EntityType::File(args.clone());
            let file_name: &str = entity_type.get_name();
            let path: PathBuf = entity_type.get_path();
            let file_iter: &i32 = entity_type.get_iter();
            let file_start: &i32 = entity_type.get_start();

            create_file::create_file(file_name, &path, file_iter, file_start).unwrap();
        }
    }
}
