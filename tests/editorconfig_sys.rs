use editorconfig_sys::*;
use rand::Rng;
use std::{
    collections::HashMap,
    ffi::{CStr, CString},
    fs,
    path::Path,
    ptr,
};

const DEFAULT_CONF_FILE_NAME: &str = ".editorconfig";

#[test]
fn create_destroy_handle() {
    unsafe {
        let h = editorconfig_handle_init();
        assert!(!h.is_null());

        assert_eq!(editorconfig_handle_destroy(h), 0);
    }
}

#[test]
fn get_error_file() {
    let invalid_conf_file_path =
        fs::canonicalize(Path::new("tests/.editorconfig.invalid")).unwrap();
    let invalid_conf_file_name = invalid_conf_file_path.file_name().unwrap();
    let invalid_conf_file_name = invalid_conf_file_name.to_str().unwrap();
    let invalid_conf_file_name = CString::new(invalid_conf_file_name).unwrap();

    // We use this .rs file for testing, but this could be any file as we are
    // only interested in the errors from an invalid config file when parsing it
    let test_file_path = fs::canonicalize(file!()).unwrap();
    let test_file_path = test_file_path.to_str().unwrap();
    let test_file_path = CString::new(test_file_path).unwrap();

    unsafe {
        let h = editorconfig_handle_init();
        assert!(!h.is_null());

        // Parse test file with the default and valid config file
        let err_num = editorconfig_parse(test_file_path.as_ptr(), h);
        assert_eq!(err_num, 0);

        // No error, no error file
        let err_file = editorconfig_handle_get_err_file(h);
        assert!(err_file.is_null());

        // Set invalid config file name
        editorconfig_handle_set_conf_file_name(h, invalid_conf_file_name.as_ptr());

        // Parse test file with an invalid config file
        let err_num = editorconfig_parse(test_file_path.as_ptr(), h);
        assert_eq!(err_num, 3, "Error at line 3 in invalid config file");

        let err_file_path = editorconfig_handle_get_err_file(h);
        assert!(!err_file_path.is_null());

        let err_file_path = CStr::from_ptr(err_file_path).to_str().unwrap();
        assert_eq!(err_file_path, invalid_conf_file_path.to_str().unwrap());

        assert_eq!(editorconfig_handle_destroy(h), 0);
    }
}

#[test]
fn get_version() {
    let mut major = -1;
    let mut minor = -1;
    let mut patch = -1;

    unsafe {
        let h = editorconfig_handle_init();
        assert!(!h.is_null());

        editorconfig_handle_get_version(h, &mut major, &mut minor, &mut patch);

        assert_eq!(editorconfig_handle_destroy(h), 0);
    }

    assert_eq!(major, 0);
    assert_eq!(minor, 0);
    assert_eq!(patch, 0);
}

#[test]
fn set_get_version() {
    let mut rng = rand::thread_rng();

    let mut out_major = -1;
    let mut out_minor = -1;
    let mut out_patch = -1;

    for _ in 1..1000 {
        let in_major = rng.gen_range(0..1000);
        let in_minor = rng.gen_range(1..1000);
        let in_patch = rng.gen_range(0..1000);

        unsafe {
            let h = editorconfig_handle_init();
            assert!(!h.is_null());

            editorconfig_handle_set_version(h, in_major, in_minor, in_patch);
            editorconfig_handle_get_version(h, &mut out_major, &mut out_minor, &mut out_patch);

            assert_eq!(editorconfig_handle_destroy(h), 0);
        }

        assert_eq!(in_major, out_major);
        assert_eq!(in_minor, out_minor);
        assert_eq!(in_patch, out_patch);
    }
}

#[test]
fn get_conf_file_name() {
    unsafe {
        let h = editorconfig_handle_init();
        assert!(!h.is_null());

        let conf_file_name = editorconfig_handle_get_conf_file_name(h);
        assert!(conf_file_name.is_null());
    }
}

#[test]
fn set_get_conf_file_name() {
    let conf_file_name = CString::new(DEFAULT_CONF_FILE_NAME).unwrap();

    unsafe {
        let h = editorconfig_handle_init();
        assert!(!h.is_null());

        editorconfig_handle_set_conf_file_name(h, conf_file_name.as_ptr());

        let conf_file_name = editorconfig_handle_get_conf_file_name(h);
        assert!(!conf_file_name.is_null());

        let conf_file_name = CStr::from_ptr(conf_file_name).to_str().unwrap();
        assert_eq!(conf_file_name, DEFAULT_CONF_FILE_NAME);

        assert_eq!(editorconfig_handle_destroy(h), 0);
    }
}

#[test]
fn parse_config_file_and_determine_rules_for_rust_file() {
    // As defined in .editorconfig
    let mut rs_file_rules = HashMap::new();
    rs_file_rules.insert("charset", "utf-8");
    rs_file_rules.insert("end_of_line", "lf");
    rs_file_rules.insert("insert_final_newline", "true");
    rs_file_rules.insert("trim_trailing_whitespace", "true");

    // We use this .rs file for testing, but libeditorconfig requires absolute paths
    let test_file_path = fs::canonicalize(file!()).unwrap();
    let test_file_path = test_file_path.to_str().unwrap();
    let test_file_path = CString::new(test_file_path).unwrap();

    unsafe {
        let h = editorconfig_handle_init();
        assert!(!h.is_null());

        let err_num = editorconfig_parse(test_file_path.as_ptr(), h);
        assert_eq!(err_num, 0);

        let rule_count = editorconfig_handle_get_name_value_count(h);
        assert_eq!(rule_count as usize, rs_file_rules.len());

        let (mut rule_name, mut rule_value) = (ptr::null(), ptr::null());

        for rule_index in 0..rule_count {
            editorconfig_handle_get_name_value(h, rule_index, &mut rule_name, &mut rule_value);

            assert!(!rule_name.is_null());
            assert!(!rule_value.is_null());

            let rule_name = CStr::from_ptr(rule_name).to_str().unwrap();
            let rule_value = CStr::from_ptr(rule_value).to_str().unwrap();
            assert_eq!(rs_file_rules.get(rule_name).unwrap(), &rule_value);
        }

        assert_eq!(editorconfig_handle_destroy(h), 0);
    }
}

#[test]
fn no_parse_get_rule_count() {
    unsafe {
        let h = editorconfig_handle_init();
        assert!(!h.is_null());

        let rule_count = editorconfig_handle_get_name_value_count(h);
        assert_eq!(rule_count, 0);

        assert_eq!(editorconfig_handle_destroy(h), 0);
    }
}

#[test]
fn relative_file_path_error() {
    let relative_file_path = CString::new(file!()).unwrap();

    unsafe {
        let h = editorconfig_handle_init();
        assert!(!h.is_null());

        // Compare error number with error constant
        let err_num = editorconfig_parse(relative_file_path.as_ptr(), h);
        assert_eq!(err_num, EDITORCONFIG_PARSE_NOT_FULL_PATH);

        assert_eq!(editorconfig_handle_destroy(h), 0);
    }
}

#[test]
fn version_too_new_error() {
    let test_file_path = fs::canonicalize(file!()).unwrap();
    let test_file_path = test_file_path.to_str().unwrap();
    let test_file_path = CString::new(test_file_path).unwrap();

    unsafe {
        let h = editorconfig_handle_init();
        assert!(!h.is_null());

        editorconfig_handle_set_version(h, i32::MAX, i32::MAX, i32::MAX);

        // Compare error number with error constant
        let err_num = editorconfig_parse(test_file_path.as_ptr(), h);
        assert_eq!(err_num, EDITORCONFIG_PARSE_VERSION_TOO_NEW);

        assert_eq!(editorconfig_handle_destroy(h), 0);
    }
}

#[test]
fn get_error_message_no_error() {
    let no_err_num = 0;

    unsafe {
        let err_msg = editorconfig_get_error_msg(no_err_num);
        assert!(!err_msg.is_null());

        let err_msg = CStr::from_ptr(err_msg).to_str().unwrap();
        assert!(err_msg.len() > 0);
    }
}

#[test]
fn get_error_message_parse_error() {
    let mut rng = rand::thread_rng();

    // Any error > 0 is a parsing error at that line
    let parse_err_line_num = rng.gen_range(1..=i32::MAX);

    unsafe {
        let err_msg = editorconfig_get_error_msg(parse_err_line_num);
        assert!(!err_msg.is_null());

        let err_msg = CStr::from_ptr(err_msg).to_str().unwrap();
        assert!(err_msg.len() > 0);
    }
}

#[test]
fn get_error_message_relative_path_error() {
    unsafe {
        let err_msg = editorconfig_get_error_msg(EDITORCONFIG_PARSE_NOT_FULL_PATH);
        assert!(!err_msg.is_null());

        let err_msg = CStr::from_ptr(err_msg).to_str().unwrap();
        assert!(err_msg.len() > 0);
    }
}

#[test]
fn get_error_message_memory_error() {
    unsafe {
        let err_msg = editorconfig_get_error_msg(EDITORCONFIG_PARSE_MEMORY_ERROR);
        assert!(!err_msg.is_null());

        let err_msg = CStr::from_ptr(err_msg).to_str().unwrap();
        assert!(err_msg.len() > 0);
    }
}

#[test]
fn get_error_message_version_error() {
    unsafe {
        let err_msg = editorconfig_get_error_msg(EDITORCONFIG_PARSE_VERSION_TOO_NEW);
        assert!(!err_msg.is_null());

        let err_msg = CStr::from_ptr(err_msg).to_str().unwrap();
        assert!(err_msg.len() > 0);
    }
}

#[test]
fn lib_get_version() {
    let mut major = -1;
    let mut minor = -1;
    let mut patch = -1;

    unsafe {
        editorconfig_get_version(&mut major, &mut minor, &mut patch);
    }

    // libeditorconfig 0.12.5 is currently the minimum supported version
    assert!(major >= 0);
    assert!(minor >= 12);
    assert!(patch >= 5);
}
