//use std::process::Command;
use clap::Parser;
use std::error::Error;
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Parser)]
#[command(name = "windiff")]
#[command(author = "Francois Malassenet <francois@concurrents.com>")]
#[command(version = "0.0")]
#[command(about="Launches WinMerge to compare files between two directories", long_about=None)]
struct Cli {
    file: String,

    #[arg(short, long)]
    left_root_dir: Option<String>,

    #[arg(short, long)]
    right_root_dir: Option<String>,

    #[arg(short, long)]
    verbose: bool,
}

fn truncate_path(absolute: &PathBuf, root: &PathBuf) -> Result<PathBuf, Box<dyn Error>> {
    let relative = absolute.strip_prefix(&root)?;
    Ok(relative.to_path_buf())
}

// Get rid of the \\\\?\\D: when using D:
fn clean_windows(path: &Path) -> String {
    let path_str = path.to_string_lossy();
    path_str
        .strip_prefix(r"\\?\")
        .unwrap_or(&path_str)
        .to_string()
}

// fn is_normal(component: &Component) -> bool {
//     matches!(component, Component::Normal(_))
// }

// // Ignore  should probably integrate with a ProjectConfig
// fn relative_path<P>(from: &Path, to: &Path, predicate: P) -> Option<PathBuf>
// where
//     P: Fn(&Component) -> bool + Copy,
// {
//     // skip the non Normal components, e.g. root etc.
//
//     let mut from_iter = from.components().filter(predicate);
//     let mut to_iter = to.components().filter(predicate);
//
//     let mut common_prefix_len = 0;
//     while let (Some(a), Some(b)) = (from_iter.next(), to_iter.next()) {
//         println!("a: {:?}, b: {:?}", a, b);
//         if a == b {
//             common_prefix_len += 1;
//         } else {
//             break;
//         }
//     }
//
//     let from_depth = from.components().filter(predicate).count();
//     println!("{}", from_depth);
//
//     let mut relative = PathBuf::new();
//     for _ in common_prefix_len..from_depth {
//         relative.push("..");
//     }
//     println!("{:?}", relative.to_str().unwrap());
//
//     relative.extend(to.components().filter(predicate).skip(common_prefix_len));
//
//     // Convert empty to '.'
//     if relative.as_os_str().is_empty() {
//         Some(".".into())
//     } else {
//         Some(relative)
//     }
// }

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    let winmerge_path = "C:\\Program Files\\WinMerge\\WinMergeU.exe";

    let default_left_root_dir = "D:\\UE5WorkTree\\UE5.2GPEGNanite";
    let default_right_root_dir = "D:\\UE5WorkTree\\UE5.3Nanite";

    // use the default values for the root directories
    let left_root_dir = PathBuf::from(
        cli.left_root_dir
            .as_deref()
            .unwrap_or(default_left_root_dir),
    );
    let right_root_dir = PathBuf::from(
        cli.right_root_dir
            .as_deref()
            .unwrap_or(default_right_root_dir),
    );

    // if can truncate the files keep as is otherwise
    let mut left_file = PathBuf::from(&cli.file);
    let can_truncate = truncate_path(&left_file, &left_root_dir.to_path_buf()).is_err();
    if can_truncate {
        left_file = left_root_dir.join(&cli.file);
    }
    let right_file = right_root_dir.join(&cli.file);

    if cli.verbose {
        eprintln!("left_root_dir: {}", clean_windows(&left_root_dir));
        eprintln!("left_file: {}", clean_windows(&left_file));
        eprintln!("right_root_dir: {}", clean_windows(&right_root_dir));
        eprintln!("right_file: {}", clean_windows(&right_file));
    }

    let output = Command::new(&winmerge_path)
        .arg(&left_file)
        .arg(&right_file)
        .output()
        .expect("Failed to execute winmerge");

    if cli.verbose {
        eprintln!("output: {:?}", output);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;
    #[test]
    fn verify_cli() {
        Cli::command().debug_assert();
    }
    #[test]
    fn verify_truncate() {
        {
            // Good truncation -> Ok(relative)
            let root = PathBuf::from("D:\\More");
            let absolute = PathBuf::from("D:\\More\\Morer\\junk.md");
            let relative = truncate_path(&absolute, &root).unwrap();

            let expected = PathBuf::from("Morer\\junk.md");
            assert_eq!(&relative, &expected);
        }

        {
            // Bad truncation -> Error
            let root = PathBuf::from("D:\\More");
            let absolute = PathBuf::from("D:\\Less\\Morer\\junk.md");
            let relative = truncate_path(&absolute, &root);
            assert!(relative.is_err());
        }
    }

    #[test]
    fn verify_root() {
        let root = PathBuf::from("D:\\More");
        let absolute = PathBuf::from("D:\\More\\Morer\\junk.md");
        let project_path = ProjectPathBuf::from_absolute(root, absolute).unwrap();

        let absolute_copy = PathBuf::from("D:\\More\\Morer\\junk.md");
        assert_eq!(&project_path.absolute(), &absolute_copy);
    }
}
