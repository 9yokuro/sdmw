#[cfg(test)]
mod tests {
    use crate::{
        subcommands::{
            add::add, install::install, new::new, restore::restore, uninstall::uninstall,
        },
        utils::*,
        Options, Settings,
    };
    use std::{
        env::set_current_dir,
        fs::{create_dir_all, remove_dir_all, remove_file, File},
        os::unix::fs::symlink,
        path::Path,
    };

    const TEST_DIR: &str = "testdir";
    const SETTINGS: &str = "testdir/settings.json";
    const OPTIONS: &Options = &Options::new(false, false);

    fn prepare_test() {
        create_dir_all(TEST_DIR).unwrap();
    }

    fn finish_test() {
        remove_dir_all(TEST_DIR).unwrap();
    }

    #[test]
    fn test_file_name() {
        assert_eq!(file_name("a/b.txt").unwrap(), "b.txt".to_string());
        assert_eq!(file_name("a/").unwrap(), "a".to_string());
    }

    #[test]
    fn test_print_already_exists() {
        print_already_exists("a.txt");
    }

    #[test]
    fn test_print_not_a_symlink() {
        print_not_a_symlink("a.txt");
    }

    #[test]
    fn test_settings() {
        prepare_test();

        Settings::new(vec![]).write(SETTINGS).unwrap();

        assert_eq!(Settings::read(SETTINGS).unwrap(), Settings::new(vec![]));

        finish_test();
    }

    #[test]
    fn test_new() {
        prepare_test();

        let test_repo = format!("{}/test_repo", TEST_DIR);

        new(&vec![test_repo.clone()], OPTIONS).unwrap();

        assert!(Path::new(&test_repo).exists());
        assert!(Path::new(&format!("{}/.git", &test_repo)).exists());
        assert!(Path::new(&format!("{}/settings.json", &test_repo)).exists());
        assert!(Path::new(&format!("{}/README.md", &test_repo)).exists());

        finish_test()
    }

    #[test]
    fn test_add() {
        prepare_test();

        File::create("a.txt").unwrap();
        File::create("b.txt").unwrap();

        let settings = Settings::new(vec!["../a.txt".to_string(), "../b.txt".to_string()]);
        settings.write(SETTINGS).unwrap();

        set_current_dir(TEST_DIR).unwrap();

        add(&settings, OPTIONS).unwrap();

        assert!(Path::new("a.txt").exists());
        assert!(Path::new("b.txt").exists());

        set_current_dir("../").unwrap();

        finish_test();
    }

    #[test]
    fn test_install() {
        prepare_test();

        let settings = Settings::new(vec!["../a.txt".to_string(), "../b.txt".to_string()]);
        settings.write(SETTINGS).unwrap();

        set_current_dir(TEST_DIR).unwrap();

        File::create("a.txt").unwrap();
        File::create("b.txt").unwrap();

        install(&settings, OPTIONS).unwrap();

        set_current_dir("../").unwrap();

        assert!(Path::new("a.txt").is_symlink());
        assert!(Path::new("b.txt").is_symlink());

        remove_file("a.txt").unwrap();
        remove_file("b.txt").unwrap();

        finish_test();
    }

    #[test]
    fn test_restore() {
        prepare_test();

        let mut settings = Settings::new(vec!["../a.txt".to_string(), "../b.txt".to_string()]);
        settings.write(SETTINGS).unwrap();

        set_current_dir(TEST_DIR).unwrap();

        File::create("a.txt").unwrap();
        File::create("b.txt").unwrap();

        restore(
            &mut settings,
            &vec!["a.txt".to_string(), "b.txt".to_string()],
            OPTIONS,
        )
        .unwrap();

        set_current_dir("../").unwrap();

        assert!(Path::new("a.txt").exists());
        assert!(Path::new("b.txt").exists());

        remove_file("a.txt").unwrap();
        remove_file("b.txt").unwrap();

        finish_test();
    }

    #[test]
    fn test_uninstall() {
        prepare_test();

        let settings = Settings::new(vec!["../a.txt".to_string(), "../b.txt".to_string()]);
        settings.write(SETTINGS).unwrap();

        set_current_dir(TEST_DIR).unwrap();

        File::create("a.txt").unwrap();
        File::create("b.txt").unwrap();

        symlink("a.txt", "../a.txt").unwrap();
        symlink("b.txt", "../b.txt").unwrap();

        uninstall(&settings, OPTIONS).unwrap();

        set_current_dir("../").unwrap();

        assert!(!Path::new("a.txt").is_symlink() && !Path::new("a.txt").exists());
        assert!(!Path::new("b.txt").is_symlink() && !Path::new("b.txt").exists());

        finish_test();
    }
}
