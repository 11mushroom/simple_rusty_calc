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

use core::panic;
use std::{char, env, f32::INFINITY, fmt::Display};

/**
* enum to identify operations
*/
#[derive(Debug)]
pub enum Ops {
    Add,
    Sub,
    Div,
    Mul,
    Pow,
}

/**
* enum to store tokens
*/
#[derive(Copy, Clone)]
pub enum Token {
    Op{op:Ops, priority: u32},
    Num{num:f32},
    SOE,
    EOE,
    OpenParenth,
    CloseParenth,
}

/**
* enum used to build expression tree
*/
#[derive(Clone)]
pub enum Nodes {
    Op{
        op:Ops,
        priority:u32,
        childs: Vec<Nodes>,
    },
    Num{
        num:f32,
    },
    Nab,
    SOE,
    EOE,
}

impl Nodes {

    ///function to solve expression tree recursively
    pub fn rec_solve(&self) -> Option<f32> {
        match self {
            Nodes::Op { op, childs, .. } => {
                let left = match &childs[0] {
                    Nodes::Num { num } => Some(*num),
                    x @ Nodes::Op { .. } => x.rec_solve(),
                    _ => None,
                };

                let right = match &childs[1] {
                    Nodes::Num { num } => Some(*num),
                    x @ Nodes::Op { .. } => x.rec_solve(),
                    _ => None,
                };

                match op {
                    Ops::Add => {
                        match (left, right) {
                            (Some(l), Some(r)) => {
                                return Some(l+r)
                            },
                            _=>None,
                        }
                    },
                    Ops::Sub => {
                        match (left, right) {
                            (Some(l), Some(r)) => {
                                return Some(l-r)
                            },
                            _=>None,
                        }
                    },
                    Ops::Mul => {
                        match (left, right) {
                            (Some(l), Some(r)) => {
                                return Some(l*r)
                            },
                            _=>None,
                        }
                    },
                    Ops::Pow => {
                        match (left, right) {
                            (Some(l), Some(r)) => {
                                return Some(l.powf(r))
                            },
                            _=>None,
                        }
                    },
                    Ops::Div => {
                        match (left, right) {
                            (Some(l), Some(r)) => {
                                if r == 0_f32 {
                                    return Some(INFINITY)
                                }
                                return Some(l/r)
                            },
                            _=>None,
                        }
                    },
                }
            },
            Nodes::Num { num } => {
                return Some(*num)
            },
            _ => {
                return None
            },
        }
    }

    fn new_op(op:Ops, pr:u32) -> Self {
       Self::Op { op: op, priority: pr, childs: vec![Nodes::Nab;2] }
    }

    ///checks if operation Node has all 2 childrens set
    fn op_complete(&self) -> bool {
        match self {
            Nodes::Op { childs, .. } => {
                !(matches!(childs[0], Nodes::Nab) || matches!(childs[1], Nodes::Nab))
            },
            _ => false,
            
        }
    }
    
    fn set_child(&mut self, ind:usize, chld:Nodes) {
        match self {
            Nodes::Op { childs, .. } => {
                childs[ind&1]=chld;
            },
            _=>{}
        }
    }
}

impl std::fmt::Debug for Nodes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Nodes::Op { op, priority, childs } => {
                write!(f, "{op}{}({:?}, {:?})", *priority, childs[0], childs[1])
            },
            Nodes::Num { num } => {
                write!(f, "{}", *num)
            },
            Nodes::SOE => {
                write!(f, "(")
            },
            Nodes::EOE => {
                write!(f, ")")
            },
            Nodes::Nab => {
                write!(f, "_")
            },
            
        }
    }
}

impl Display for Nodes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Nodes::Op { op, childs, .. } => {
                write!(f, "{op}({}, {})", childs[0], childs[1])
            },
            Nodes::Num { num } => {
                write!(f, "{}", *num)
            },
            Nodes::SOE => {
                write!(f, "(")
            },
            Nodes::EOE => {
                write!(f, ")")
            },
            Nodes::Nab => {
                write!(f, "_")
            },
            
        }
        
    }
}

impl std::fmt::Debug for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Op { op, priority } => {
                f.debug_tuple("").field(op).field(priority).finish()
            },
            Token::Num { num } => {
                write!(f, "{}", *num)
            },
            Token::OpenParenth => {
                write!(f, "(")
            },
            Token::CloseParenth => {
                write!(f, ")")
            },
            Token::SOE => {
                write!(f, "start")
            } ,
            Token::EOE => {
                write!(f, "end")
            } ,
            
        }
    }
}

impl Copy for Ops { }

impl Clone for Ops {
    fn clone(&self) -> Self {
        *self
    }
}

impl Display for Ops {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Add => {
                write!(f, "+")
            },
            Self::Sub => {
                write!(f, "-")
            },
            Self::Mul => {
                write!(f, "*")
            },
            Self::Pow => {
                write!(f, "^")
            },
            Self::Div => {
                write!(f, "/")
            },
        }
    }
}

impl Ops {
    
    const MAXP:u32=2;

    const fn priority(&self) -> u32 {
        match self {
            Ops::Add|Ops::Sub => 0,
            Ops::Mul|Ops::Div => 1,
            Ops::Pow => 2,
        }
    }
}

///function to tokenize input string
pub fn calc_tokenize(src:&String) -> Vec<Token> {
    let chrs:Vec<char>=src.chars().collect();
    let mut res1:Vec<Token>=Vec::new();
    let mut i:usize=0;
    let mut pdepth:u32=0;

    res1.push(Token::SOE);

    'scan:while let Some(c)=chrs.get(i) {
        match c {
            '0'..='9' => {
                let mut a=1;
                let mut num_buff:String = String::new();
                num_buff.push(*c);
                while let Some(digit) = chrs.get(i+a) {
                    if digit.is_digit(10) || *digit=='.' {
                        num_buff.push(*digit);
                        a+=1;
                        continue;
                    }
                    break;
                }
                let num_num=num_buff.as_str().parse::<f32>();
                if num_num.is_ok() {
                    res1.push(Token::Num{num:num_num.unwrap()});
                }
                i+=a;
                continue;
            },
            '-' => {
                res1.push(Token::Op { op: Ops::Sub, priority: Ops::Sub.priority()+(Ops::MAXP+1)*pdepth});
            },
            '+' => {
                res1.push(Token::Op { op: Ops::Add, priority: Ops::Add.priority()+(Ops::MAXP+1)*pdepth});
            },
            '*' => {
                res1.push(Token::Op { op: Ops::Mul, priority: Ops::Mul.priority()+(Ops::MAXP+1)*pdepth});
            },
            '/' => {
                res1.push(Token::Op { op: Ops::Div, priority: Ops::Div.priority()+(Ops::MAXP+1)*pdepth});
            },
            '^' => {
                res1.push(Token::Op { op: Ops::Pow, priority: Ops::Pow.priority()+(Ops::MAXP+1)*pdepth});
            },
            '(' => {
                res1.push(Token::OpenParenth);
                pdepth+=1;
            },
            ')' if pdepth>0 => {
                res1.push(Token::CloseParenth);
                pdepth-=1;
            },
            ' ' => {},
            _ => {},
        }
        i+=1;
    }
    res1.push(Token::EOE);

    let mut resf:Vec<Token>=Vec::new();
    let mut res1_i:usize=0;
    
    'finalize:while let Some(t) = res1.get(res1_i) {
        match t {
            Token::Num{..} => {
                resf.push(*t);
            },
            Token::Op{op:Ops::Sub, ..} if matches!(res1.get(res1_i+1), Some(Token::Num{..}))
                                          && !matches!(res1.get(res1_i-1), Some(Token::Num {..})) => {
                match res1[res1_i+1] {
                    Token::Num { num } => {
                        resf.push(Token::Num { num:-num });
                    },
                    _=>{},
                }
                res1_i+=1;
            },
            Token::Op {..} => {
                resf.push(*t);
            },
            Token::OpenParenth|Token::CloseParenth => {
                //resf.push(*t);
            },
            Token::SOE|Token::EOE => {
                resf.push(*t);
            },
        }
        res1_i+=1;
    }

    resf
}

///function to build expression tree from tokens
pub fn treefication(src:Vec<Token>) -> Option<Nodes> {
    let mut old_res:Vec<Nodes>=Vec::with_capacity(src.len());
    let mut new_res:Vec<Nodes>;

    //first phase
    //growing trees from tokens
    let mut src_i:usize = 0;

    let mut eoe=false;
    while src_i < src.len() && !eoe {
      

      match src[src_i] {
          Token::SOE => {
            old_res.push(Nodes::SOE);
          },
          Token::Num { ref num } => {
            let new_num=Nodes::Num { num:*num };
            let old_len=old_res.len()-1;
            match (&mut old_res[old_len], &src[src_i+1]) {
                (Nodes::Op { priority:lp, childs:lc, .. }, Token::Op { op:rop, priority: rp}) => {
                    let mut new_op = Nodes::Op {
                            op: *rop,
                            priority: *rp,
                            childs: vec![Nodes::Nab;2]
                        };

                    if *lp>=*rp {
                      lc[1]=new_num;
                    } else {
                      new_op.set_child(0, new_num);
                    }

                    old_res.push(new_op);
                    src_i+=1;
                },
                (Nodes::SOE, Token::Op { op:rop, priority:rp }) => {
                    let new_op = Nodes::Op {
                            op: *rop,
                            priority: *rp,
                            childs: vec![new_num, Nodes::Nab]
                        };
                    old_res.push(new_op);
                    src_i+=1;
                },
                (Nodes::Op { childs, .. }, Token::EOE) => {
                    childs[1]=new_num;
                },
                _=>{},
            }
          },
          Token::Op { op, priority } => {
            old_res.push(Nodes::Op { op:op, priority:priority, childs: vec![Nodes::Nab;2] });
          },
          Token::EOE => {
            old_res.push(Nodes::EOE);
            eoe=true;
          },
          _=>{
          },
      }
      src_i+=1;
    }

    //second phase
    //merging trees into one

    new_res=vec![Nodes::Nab;old_res.len()];
    let mut old_i:usize=0;
    let mut new_i:usize=0;

    loop {
      eoe=false;
      while old_i < old_res.len() && !eoe {
          match old_res[old_i] {
              ref x @ Nodes::SOE => {
                  new_res[new_i]=x.clone();
                  new_i+=1;
              },
              ref s @ Nodes::Op { ref op, ref priority, ref childs } => {
                  if matches!(childs[0], Nodes::Nab) || matches!(childs[1], Nodes::Nab) {
                      new_res[new_i] = s.clone();
                      new_i+=1;

                  } else {
                      let this_node=s.clone();
                      match (&mut new_res[new_i-1], &old_res[old_i+1]) {
                          (Nodes::Op { priority:lpr, childs:lch, .. }, r @ Nodes::Op { priority: rpr, childs: rch, .. }) => {
                              let mut right=r.clone();

                              if *lpr>=*rpr && matches!(lch[1], Nodes::Nab) {
                                  lch[1]=this_node;

                              } else if rpr>lpr && matches!(rch[0], Nodes::Nab) {
                                  right.set_child(0, this_node);

                              } else {
                                  new_res[new_i]=this_node;
                                  new_i+=1;
                              }

                              old_i+=1;
                              new_res[new_i]=right;
                              new_i+=1;
                          },
                          (Nodes::SOE, r @ Nodes::Op { childs:rch, .. }) => {
                              let mut right = r.clone();
                              if matches!(rch[0], Nodes::Nab) {
                                  right.set_child(0, this_node);
                              } else {
                                  new_res[new_i]=this_node;
                                  new_i+=1;
                              }

                              old_i+=1;
                              new_res[new_i]=right;
                              new_i+=1;
                          },
                          (Nodes::Op { childs:lch, .. }, Nodes::EOE) => {
                              if matches!(lch[1], Nodes::Nab) {
                                  lch[1]=this_node;
                              } else {
                                  new_res[new_i]=this_node;
                                  new_i+=1;
                              }
                          },
                          (Nodes::SOE, Nodes::EOE) => {
                              new_res[new_i]=this_node;
                              new_i+=1;
                          },
                          _ => {panic!("wrong neighbours")},
                      }
                  }
              },
              ref x @ Nodes::EOE => {
                  new_res[new_i]=x.clone();
                  new_i+=1;
                  eoe=true;
              },
              _=>{},
          }
          old_i+=1;
      }

      old_res.truncate(old_i);
      new_res.truncate(new_i);
      
      if new_i == 3 && new_res[1].op_complete() {
        return Some(new_res.swap_remove(1))
      } else if old_i==new_i {
        break;
      } else {
        (old_i, new_i)=(0,0);
        (old_res, new_res) = (new_res, old_res);
      }
    }

    return None
}

/**
* function that combines tokenization, treefication and returns result of solving
* in other words, it just returns result of math expression
*/
pub fn calculate(expr:&String) -> Option<f32> {
    let tree=treefication(calc_tokenize(expr));
    match &tree {
        Some(t) => t.rec_solve(),
        _ => return None,
    }
}

