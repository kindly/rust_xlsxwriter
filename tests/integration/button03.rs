// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2024, John McNamara, jmcnamara@cpan.org

use crate::common;
use rust_xlsxwriter::{Button, Workbook, XlsxError};

// Create rust_xlsxwriter file to compare against Excel file.
fn create_new_xlsx_file_1(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let worksheet = workbook.add_worksheet();
    let button = Button::new();

    worksheet.insert_button(1, 2, &button)?;
    worksheet.insert_button(4, 4, &button)?;

    workbook.save(filename)?;

    Ok(())
}

// Test inserting objects with offsets from a single cell.
fn create_new_xlsx_file_2(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let worksheet = workbook.add_worksheet();
    let button = Button::new();

    worksheet.insert_button_with_offset(0, 0, &button, 128, 20)?;
    worksheet.insert_button_with_offset(0, 0, &button, 256, 80)?;

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn test_button03_1() {
    let test_runner = common::TestRunner::new()
        .set_name("button03")
        .unique("1")
        .set_function(create_new_xlsx_file_1)
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}

#[test]
fn test_button03_2() {
    let test_runner = common::TestRunner::new()
        .set_name("button03")
        .unique("2")
        .set_function(create_new_xlsx_file_2)
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}
