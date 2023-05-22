// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2023, John McNamara, jmcnamara@cpan.org

//! The following example demonstrates writing a url to a worksheet.

use rust_xlsxwriter::{Workbook, XlsxError};

fn main() -> Result<(), XlsxError> {
    // Create a new Excel file object.
    let mut workbook = Workbook::new();

    // Add a worksheet to the workbook.
    let worksheet = workbook.add_worksheet();

    // Write a url with a simple string argument.
    worksheet.write_url(0, 0, "https://www.rust-lang.org")?;

    // Save the file to disk.
    workbook.save("worksheet.xlsx")?;

    Ok(())
}
