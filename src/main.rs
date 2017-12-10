#![feature(slice_concat_ext)]
extern crate unicode_width;
use unicode_width::UnicodeWidthStr;

use std::io;
use std::io::prelude::*;
use std::env;
use std::iter;
use std::slice::SliceConcatExt;

fn input() -> Vec<Vec<String>> {
    let stdin = io::stdin();
    let lock = stdin.lock();
    let mut cols = None;

    // we cannot omit the 'res' variable. is it the limitation of rust type system?
    let res = lock.lines().map(|x| {
        let line = x.unwrap();
        let splitted: Vec<_> = line.split(',').map(|x| x.to_string()).collect();
        if let Some(cols) = cols {
            assert_eq!(splitted.len(), cols);
        } else {
            cols = Some(splitted.len());
        }

        splitted
    }).collect();
    res
}

fn maxcols(cells: &Vec<Vec<String>>) -> Vec<usize> {
    let mut res = vec![0usize; cells[0].len()];
    for row in cells {
        for i in 0..row.len() {
            res[i] = std::cmp::max(res[i], row[i].width_cjk());
        }
    }
    res
}

// return: (blocks, transformed_table)
fn transform(cells: &Vec<Vec<String>>, line_to_wrap: usize) -> (usize, Vec<Vec<String>>) { // {{{
    let head = &cells[0];

    let orig_len = cells.len();
    let data_len = orig_len - 1;
    let cols = cells[0].len();

    let blocks = (data_len + (line_to_wrap - 1)) / line_to_wrap; // 切り上げ計算
    let mut result = vec![];
    result.push(iter::repeat(head.clone())
                .take(blocks)
                .collect::<Vec<_>>()
                .concat());
    for l in 0..line_to_wrap {
        let mut to_push = vec![];
        for i in 0..blocks {
            let orig = line_to_wrap * i + l + 1;
            let mut to_append = if orig < orig_len {
                cells[orig].clone()
            } else {
                vec!["".to_string(); cols]
            };
            to_push.append(&mut to_append);
        }
        result.push(to_push);
    }
    (blocks, result)
} // }}}

fn transform_maxcols(maxwidth: Vec<usize>, blocks: usize) -> Vec<usize> {
    iter::repeat(maxwidth).take(blocks).collect::<Vec<_>>().concat()
}

fn hline(width: &Vec<usize>, padding: usize, delim: char) -> String { // {{{
    let line = width.iter().map(|w| {
        iter::repeat(delim)
            .take(*w + (padding * 2) - 1)
            .collect::<String>() + ":"
    }).collect::<Vec<String>>().join("|");
    format!("|{}|", line)
} // }}}

// maxwidth is not implemented.
fn draw(table: Vec<Vec<String>>, transformed_maxcol: &Vec<usize>) -> String {
    let mut result_row = vec![];
    let mut first = true;
    for row in table {
        result_row.push(draw_row(row, transformed_maxcol));
        if first {
            result_row.push(hline(transformed_maxcol,1, '-'));
            first = false;
        }
    }
    result_row.join("\n")
}

fn draw_row(row: Vec<String>, transformed_maxcol: &Vec<usize>) -> String {
    let mut result = vec![];
    for (cell, w) in row.into_iter().zip(transformed_maxcol.iter()) {
        result.push(draw_cell(cell, *w));
    }
    concat_cells_in_row(result)
}

fn draw_cell(cell: String, w: usize) -> String {
    format!(" {:>w$} ", cell, w=w)
}

fn concat_cells_in_row(cells_in_row: Vec<String>) -> String {
    format!("|{}|", cells_in_row.join("|"))
}

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() <= 1 {
        eprintln!("Usage: {} line_to_wrap [maxwidth]", args[0]);
        eprintln!("          line to wrap: line to wrap the table");
        eprintln!("          maxwidth: (optional) max width of each column");
        eprintln!("                    specify such as '1,4,2,3,4'");
        std::process::exit(1);
    }
    let line_to_wrap: usize = args[1].parse()
        .expect("failed to parse argument 1: line_to_wrap.");

    let cells = input();
    if cells.len() == 0 { return; }
    let maxcols = maxcols(&cells);
    assert_eq!(maxcols.len(), cells[0].len(), "maxwidth's size is not equal to the table's columns count.");

    let (blocks, table) = transform(&cells, line_to_wrap);
    let transformed_maxcol = transform_maxcols(maxcols, blocks);
    println!("{}", draw(table, &transformed_maxcol));
}
