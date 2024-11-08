// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2024, John McNamara, jmcnamara@cpan.org

use crate::common;
use rust_xlsxwriter::{Shape, Workbook, XlsxError};

// Create rust_xlsxwriter file to compare against Excel file.
fn create_new_xlsx_file_1(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let worksheet = workbook.add_worksheet();

    // Use longer syntax.
    let textbox = Shape::textbox().set_text("This is some text");

    worksheet.insert_shape(8, 4, &textbox)?;

    let textbox = Shape::textbox().set_text("Some more text");

    worksheet.insert_shape(17, 7, &textbox)?;

    workbook.save(filename)?;

    Ok(())
}

// Test inserting objects with offsets from a single cell.
fn create_new_xlsx_file_2(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let worksheet = workbook.add_worksheet();

    let textbox = Shape::textbox().set_text("This is some text");

    worksheet.insert_shape_with_offset(0, 0, &textbox, 256, 160)?;

    let textbox = Shape::textbox().set_text("Some more text");

    worksheet.insert_shape_with_offset(0, 0, &textbox, 448, 340)?;

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn test_textbox02_1() {
    let test_runner = common::TestRunner::new()
        .set_name("textbox02")
        .unique("1")
        .set_function(create_new_xlsx_file_1)
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}

#[test]
fn test_textbox02_2() {
    let test_runner = common::TestRunner::new()
        .set_name("textbox02")
        .unique("2")
        .set_function(create_new_xlsx_file_2)
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}
