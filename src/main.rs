use std::io;
use std::cmp;
use rayon::prelude::*;
use std::sync::{Arc, Mutex};

const RADIX: u32 = 10;


type Row = Vec<u32>;

fn init_rows(row_size: usize, value_count: u32) -> Vec<Row> {
    let mut res: Vec<Row> = Vec::new();
    let mut gen: Row = Row::new();
    gen.resize(row_size, 0);
    res.push(gen.clone());
    let mut i = 0usize;
    while i < row_size {
        let val = & mut gen[i];
        *val += 1;
        if *val < value_count {
            i = 0;
            res.push(gen.clone());
        } else {
            i += 1;
            
            *val = 0;
        }
    }

    return res;
}

fn count_white_blacks(lhs: &[u32], rhs: &Row, value_count: u32) -> (u32, u32) {
    let mut b = 0_u32;
    let mut w = 0_u32;

    for num in 0..value_count {
        let mut wl = 0_u32;
        let mut wr = 0_u32;
        for (lhs, rhs) in lhs.iter().zip(rhs.iter()) {
            if *lhs == num {
                if *rhs == *lhs {
                    b += 1;
                } else {
                    wl += 1;
                }
            } else if *rhs == num {
                wr += 1;
            }
        }
        w += cmp::min(wl, wr);
    }
    return (b, w);
}

fn filter(rows: & mut Vec<Row>, crit: &Row, w: u32, b: u32, value_count: u32) {

    rows.retain(|r| {
        let (b2, w2) = count_white_blacks(r, &crit, value_count);
        return b == b2 && w == w2;
    });
}

fn calc_min_eliminated(rows: & Vec<Row>, row: &Row, value_count: u32) -> u32 {

    let mut res: Row = Row::new();
    res.resize((((row.len() + 1) * row.len() + 3) / 2) as usize, rows.len() as u32);
    for r in rows {
        let (b, w) = count_white_blacks(r, &row, value_count);
        let n = row.len() as u32 - b;
        res[((n * n.wrapping_sub(1)) / 2 + w) as usize] -= 1;
    }

    return *res.iter().min().unwrap();
}

#[allow(dead_code)]
fn best(rows: & Vec<Row>, value_count: u32) -> &Row {
    let best = rows.iter().map(|row| {
        let n = calc_min_eliminated(rows, &row, value_count);
        return (n, row);
    }).max_by(|x, y| {
        return x.0.cmp(&y.0); 
    });
    return best.unwrap().1;
}

#[allow(dead_code)]
fn best2(rows: & Vec<Row>, value_count: u32) -> &Row {

    let best: (u32, Option<&Row>) = (0, None);
    let data = Arc::new(Mutex::new(best));
    rows.par_iter().for_each(|row| {
         let n = calc_min_eliminated(rows, &row, value_count);
         let mut data = data.lock().unwrap();
         let (n2, _) = *data;
         if n > n2 {
             *data = (n, Some(&row));
         }
     });
     let data = data.lock().unwrap();
     let (_, row) = *data;
     return row.unwrap();
}
 

#[allow(dead_code)]
fn best3(rows: & Vec<Row>, value_count: u32) -> &Row {
     let best = rows.par_iter().map(|row| {
         let n = calc_min_eliminated(rows, &row, value_count);
         return (n, row);
     }).max_by(|x, y| {
         return x.0.cmp(&y.0); 
     });
     return best.unwrap().1;
 }

fn main() {
    let mut line = String::new();

    println!("row size:");
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line");

    let row_size: usize = line.trim().parse().expect("Please type a number!");

    println!("values count:");
    line.clear();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line");


    let value_count: u32 = line.trim().parse().expect("Please type a number!");

    let mut rows = init_rows(row_size, value_count);
    let mut row = best3(&rows, value_count).clone();
    loop {
        println!("{:?}", row);
        line.clear();
        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");

        let values : Row = line.trim().chars().map(
            |c| c.to_digit(RADIX).unwrap()).collect();
        assert_eq!(values.len(), 2);

        filter(& mut rows, &row, values[1], values[0], value_count);
        if rows.len() == 1 {
            row = rows[0].clone();
            break;
        }
        row = best3(&rows, value_count).clone();
    }
    println!("{:?}", row);
}
