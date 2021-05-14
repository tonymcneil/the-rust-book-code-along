use rust_book_ch12_01_minigrep::*;

extern crate spectral;
use spectral::prelude::*;

#[test]
fn config_new_fn_returns_ok_result_given_complete_args() {
    // given
    let args = ["minigrep", "query", "filename"]
        .iter()
        .map(|s| s.to_string());

    // when
    let res = Config::new(args);

    // then
    assert_that!(res).is_ok();

    let config = res.unwrap();
    assert_that!(config.query).is_equal_to(String::from("query"));
    assert_that!(config.filename).is_equal_to(String::from("filename"));
}

#[test]
fn config_new_fn_returns_err_result_given_missing_filename_arg() {
    // given
    let args_missing_filename = ["minigrep", "query" /*, filename*/]
        .iter()
        .map(|s| s.to_string());

    // when
    let res = Config::new(args_missing_filename);

    // then
    assert_that!(res)
        .is_err()
        .contains("Didn't get a filename!");
}

#[test]
fn config_new_fn_returns_err_result_given_missing_args() {
    // given
    let args_missing_query_and_filename = ["minigrep" /*"query", "filename"*/]
        .iter()
        .map(|s| s.to_string());

    // when
    let res = Config::new(args_missing_query_and_filename);

    // then
    assert_that!(res)
        .is_err()
        .contains("Didn't get a query string!");
}

#[test]
fn run_fn_returns_ok_given_query_on_existing_file() {
    // given
    let config = Config {
        query: String::from("tony"),
        filename: String::from("poem.txt"),
        case_sensitive: true,
    };

    // when
    let res = run(&config);

    // then
    assert_that!(res).is_ok_containing(());
}

// #[test]
// fn run_fn_writes_output_given_query_on_existing_file() {
//     // given
//     let config = Config {
//         query: String::from("tony"),
//         filename: String::from("poem.txt"),
//     };
//     let mut output = Vec::new();

//     // when
//     let res = run(&config, &mut output);

//     // then
//     assert_that!(String::from_utf8(output).unwrap()).contains("INFO: contents:\n");
// }

#[test]
fn run_fn_returns_err_given_query_on_non_existant_file() {
    // given
    let config = Config {
        query: String::from("tony"),
        filename: String::from("file_doesn't_exist.na"),
        case_sensitive: true,
    };

    // when
    let res = run(&config);

    // then
    assert_that!(res).is_err().contains("No such file");
}
