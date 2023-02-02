// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2023, John McNamara, jmcnamara@cpan.org

use rust_xlsxwriter::{Format, Workbook, XlsxError};

mod common;

// Test to demonstrate worksheet protection.
fn create_new_xlsx_file(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let worksheet = workbook.add_worksheet();

    let unlocked = Format::new().set_unlocked();
    let hidden = Format::new().set_unlocked().set_hidden();

    worksheet.protect();

    worksheet.unprotect_range_with_options(0, 0, 0, 0, "", "password")?;
    worksheet.unprotect_range_with_options(0, 2, 2, 2, "", "")?;
    worksheet.unprotect_range_with_options(3, 6, 5, 8, "MyRange", "")?;
    worksheet.unprotect_range_with_options(6, 10, 6, 10, "", "foobar")?;

    worksheet.write_number(0, 0, 1)?;
    worksheet.write_number_with_format(1, 0, 2, &unlocked)?;
    worksheet.write_number_with_format(2, 0, 3, &hidden)?;

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn test_protect06() {
    let test_runner = common::TestRunner::new()
        .set_name("protect06")
        .set_function(create_new_xlsx_file)
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}
