// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2023, John McNamara, jmcnamara@cpan.org

use rust_xlsxwriter::{Format, Workbook, XlsxError};

mod common;

// Test case to demonstrate cell builtin styles.
fn create_new_xlsx_file(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let format = Format::default();

    let worksheet = workbook.add_worksheet();
    worksheet.write_string_with_format(0, 0, "Foo", &format)?;

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn styles01() {
    let test_runner = common::TestRunner::new()
        .set_name("styles01")
        .set_function(create_new_xlsx_file)
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}
