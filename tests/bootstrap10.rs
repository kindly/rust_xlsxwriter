// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2023, John McNamara, jmcnamara@cpan.org

use rust_xlsxwriter::{Format, Workbook, XlsxError};

mod common;

// Test case to demonstrate creating a basic file with some string cell data.
fn create_new_xlsx_file(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let format = Format::new().set_bold();

    let worksheet = workbook.add_worksheet();
    worksheet.write_string_with_format(0, 0, "Hello", &format)?;

    let worksheet = workbook.add_worksheet();
    worksheet.write_string_with_format(0, 0, "World", &format)?;

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn bootstrap10_bold_text_on_two_sheets() {
    let test_runner = common::TestRunner::new()
        .set_name("bootstrap10")
        .set_function(create_new_xlsx_file)
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}
