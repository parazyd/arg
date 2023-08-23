/*
 * Copyright (C) 2023 parazyd <parazyd@dyne.org>
 *
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License version 2 as
 * published by the Free Software Foundation.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License along
 * with this program; if not, write to the Free Software Foundation, Inc.,
 * 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA.
 */

use super::Args;

fn setup_args(args: Vec<&str>) -> Args<'_> {
    let mut args = Args {
        argv: args.into_iter().map(String::from).collect(),
        argv0: String::new(),
        argc_: None,
        i_: 0,
        brk_: false,
        callback: None,
    };

    args.argv0 = args.argv[0].clone();
    args
}

#[test]
fn eargf_working() {
    let mut aflag = false;
    let mut bflag = false;
    let mut bvalue = String::new();

    {
        let mut args =
            setup_args(vec!["program", "-a", "-b", "value"]).with_cb(|args, flag| match flag {
                'a' => aflag = true,
                'b' => {
                    bflag = true;
                    bvalue = args.eargf().to_string();
                }
                _ => { /* Usually usage() */ }
            });

        args.parse();
    }

    assert!(aflag);
    assert!(bflag);
    assert_eq!(bvalue, "value");
}

#[test]
#[should_panic]
fn eargf_panic() {
    let mut args = setup_args(vec!["program", "-a", "-b"]).with_cb(|args, flag| match flag {
        'a' => {}
        'b' => {
            let _ = args.eargf().to_string();
        }
        _ => { /* Usually usage() */ }
    });

    args.parse();
}

#[test]
fn argf_working() {
    let mut aflag = false;
    let mut bflag = false;
    let mut bvalue = None;

    {
        let mut args =
            setup_args(vec!["program", "-a", "-b", "value"]).with_cb(|args, flag| match flag {
                'a' => aflag = true,
                'b' => {
                    bflag = true;
                    bvalue = args.argf().map(String::from);
                }
                _ => { /* Usually usage() */ }
            });

        args.parse();
    }

    assert!(aflag);
    assert!(bflag);
    assert_eq!(bvalue, Some("value".to_string()));
}

#[test]
fn argf_none() {
    let mut aflag = false;
    let mut bflag = false;
    let mut bvalue = None;

    {
        let mut args = setup_args(vec!["program", "-a", "-b"]).with_cb(|args, flag| match flag {
            'a' => aflag = true,
            'b' => {
                bflag = true;
                bvalue = args.argf().map(String::from);
            }
            _ => { /* Usually usage() */ }
        });

        args.parse();
    }

    assert!(aflag);
    assert!(bflag);
    assert_eq!(bvalue, None);
}

#[test]
fn remaining() {
    let mut aflag = false;
    let mut bflag = false;
    let mut bvalue = None;
    let remaining;

    {
        let mut args =
            setup_args(vec!["program", "-a", "-b", "value", "imhere"]).with_cb(|args, flag| {
                match flag {
                    'a' => aflag = true,
                    'b' => {
                        bflag = true;
                        bvalue = args.argf().map(String::from);
                    }
                    _ => { /* Usually usage() */ }
                }
            });

        remaining = args.parse();
    }

    assert!(aflag);
    assert!(bflag);
    assert_eq!(bvalue, Some("value".to_string()));

    assert!(remaining.len() == 1);
    assert!(remaining[0] == "imhere")
}

#[test]
fn empty() {
    let mut args = setup_args(vec!["program"]).with_cb(|_args, flag| match flag {
        'a' => {}
        'b' => {}
        _ => {}
    });

    let remaining = args.parse();
    assert!(remaining.is_empty());
    assert!(args.argv0() == "program");
}
