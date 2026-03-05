/*
*   simple_rusty_calc simple calculator
*   Copyright (C) 2025  11mushroom
*
*   This program is free software: you can redistribute it and/or modify
*   it under the terms of the GNU General Public License as published by
*   the Free Software Foundation, either version 3 of the License, or
*   (at your option) any later version.
*
*   This program is distributed in the hope that it will be useful,
*   but WITHOUT ANY WARRANTY; without even the implied warranty of
*   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
*   GNU General Public License for more details.
*
*   You should have received a copy of the GNU General Public License
*   along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

use simple_rusty_calc::{*};
use std::env;

macro_rules! usage_doc {
    () => {
"
simple_rusty_calc \"<math expression>\" \"<v>\"
    math expression - for example \"2+2*2\"
    v - can be anything, if it set then additional info will be shown

example:
    simple_rusty_calc \"2+2*2\"
"
    };
}
fn main() {
    let args:Vec<String>=env::args().collect();
    
    if args.len()<2 {
        println!(usage_doc!());
        return
    }

    let verbose=args.len()>2;

    let tokens=calc_tokenize(&args[1]);
    if verbose {
      println!("tokens:\n  {:?}\n", tokens);
    }

    let tree=treefication(tokens);
    if verbose {
      println!("expression tree with info:\n  {:?}\n", tree);
    }

    if verbose {
      match &tree {
          Some(t) => {println!("expression tree:\n  {t}\n")},
          _=>{},
      }
    }

    if verbose {
        print!("result: ");
    }
    
    match &tree {
        Some(t) => {
            let res = t.rec_solve();
            if verbose {
                println!("{:?}", res);
            } else {
                if let Some(n) = res {
                    println!("{n}");
                } else {
                    println!("None");
                }
            }


        },
        None => {
            println!("None");
        }
    }

}
