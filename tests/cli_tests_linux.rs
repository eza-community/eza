mod cli_tests_helpers;

use cli_tests_helpers::TestDirectory;

#[test]
#[cfg(target_os = "linux")]
// This test needs locales en_US, fr_FR and ja_JP.
fn cli_tests_linux_date_current_year() {
    use std::fs::FileTimes;
    use std::time::SystemTime;

    use chrono::{Datelike, Local, TimeZone};

    let test_dir = TestDirectory::create("linux", "date_current_year");

    let current_year = Local::now().year();

    let old_date: SystemTime = Local
        .with_ymd_and_hms(current_year, 3, 3, 0, 0, 0)
        .unwrap()
        .into();
    let med_date: SystemTime = Local
        .with_ymd_and_hms(current_year, 6, 15, 23, 14, 29)
        .unwrap()
        .into();
    let new_date: SystemTime = Local
        .with_ymd_and_hms(current_year, 12, 22, 10, 38, 53)
        .unwrap()
        .into();

    // Sleep between each create as we can not modified the created time
    let peach = test_dir.create_file("peach");
    let peach_times = FileTimes::new().set_modified(med_date);
    peach.set_times(peach_times).unwrap();

    let plum = test_dir.create_file("plum");
    let plum_times = FileTimes::new().set_modified(new_date);
    plum.set_times(plum_times).unwrap();

    let pear = test_dir.create_file("pear");
    let pear_times = FileTimes::new().set_modified(old_date);
    pear.set_times(pear_times).unwrap();

    test_dir.run_tests();
}

#[test]
#[cfg(target_os = "linux")]
// This test needs locales en_US, fr_FR and ja_JP.
fn cli_tests_linux_date_locale() {
    use std::fs::FileTimes;
    use std::thread;
    use std::time::{Duration, SystemTime};

    use chrono::{Local, TimeZone};

    let test_dir = TestDirectory::create("linux", "date_locale");

    let old_date: SystemTime = Local.with_ymd_and_hms(2003, 3, 3, 0, 0, 0).unwrap().into();
    let med_date: SystemTime = Local
        .with_ymd_and_hms(2006, 6, 15, 23, 14, 29)
        .unwrap()
        .into();
    let new_date: SystemTime = Local
        .with_ymd_and_hms(2009, 12, 22, 10, 38, 53)
        .unwrap()
        .into();

    // Sleep between each create as we can not modified the created time
    let peach = test_dir.create_file("peach");
    thread::sleep(Duration::from_millis(100));
    let peach_times = FileTimes::new()
        .set_modified(med_date)
        .set_accessed(new_date);
    peach.set_times(peach_times).unwrap();

    let plum = test_dir.create_file("plum");
    thread::sleep(Duration::from_millis(100));
    let plum_times = FileTimes::new()
        .set_modified(new_date)
        .set_accessed(old_date);
    plum.set_times(plum_times).unwrap();

    let pear = test_dir.create_file("pear");
    let pear_times = FileTimes::new()
        .set_modified(old_date)
        .set_accessed(med_date);
    pear.set_times(pear_times).unwrap();

    test_dir.run_tests();
}

#[test]
#[cfg(target_os = "linux")]
fn cli_tests_linux_xattr() {
    let test_dir = TestDirectory::create("linux", "xattr");

    test_dir.create_files(&["file_no_attributes", "file_attributes"]);
    test_dir.create_dirs(&["dir_no_attributes", "dir_attributes"]);

    // command from package `attr`
    test_dir.run(
        "setfattr",
        &[
            "--name=user.xdg.tags",
            "--value=foo,bar,baz",
            "file_attributes",
        ],
    );

    test_dir.run(
        "setfattr",
        &[
            "--name=user.xdg.tags",
            "--value=foo,bar,baz",
            "dir_attributes",
        ],
    );

    test_dir.run_tests();
}
