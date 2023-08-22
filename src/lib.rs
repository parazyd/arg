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

#[cfg(test)]
mod tests;

#[allow(clippy::type_complexity)]
pub struct Args<'a> {
    argv: Vec<String>,
    argc_: Option<char>,
    i_: usize,
    brk_: bool,
    callback: Option<Box<dyn FnMut(&mut Args<'a>, char) + 'a>>,
}

impl<'a> Args<'a> {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Args<'a> {
        Args {
            argv: std::env::args().collect(),
            argc_: None,
            i_: 0,
            brk_: false,
            callback: None,
        }
    }

    pub fn with_cb<C>(mut self, cb: C) -> Self
    where
        C: FnMut(&mut Args, char) + 'a,
    {
        self.callback = Some(Box::new(cb));
        self
    }

    pub fn eargf(&mut self) -> &str {
        if self.argv[self.i_ + 1..].is_empty() || self.argv[self.i_ + 1].is_empty() {
            panic!("Expected an argument, but found none.");
        } else {
            self.brk_ = true;
            if !self.argv[self.i_ + 1].is_empty() {
                &self.argv[self.i_ + 1]
            } else {
                self.i_ += 1;
                &self.argv[self.i_]
            }
        }
    }

    pub fn argf(&mut self) -> Option<&str> {
        if self.argv[self.i_ + 1..].is_empty() || self.argv[self.i_ + 1].is_empty() {
            None
        } else {
            self.brk_ = true;
            if !self.argv[self.i_ + 1].is_empty() {
                Some(&self.argv[self.i_ + 1])
            } else {
                self.i_ += 1;
                Some(&self.argv[self.i_])
            }
        }
    }

    pub fn parse(&mut self) -> Vec<String> {
        self.i_ = 1; // skip the program name itself
        while self.i_ < self.argv.len() {
            if self.argv[self.i_].starts_with('-') {
                self.argc_ = self.argv[self.i_].chars().nth(1);
                if let Some(flag) = self.argc_ {
                    if let Some(mut cb) = self.callback.take() {
                        cb(self, flag);
                        self.callback = Some(cb); // restore the callback after using it
                    }
                    if self.brk_ {
                        self.brk_ = false;
                        self.i_ += 2; // skip the flag and its value
                    } else {
                        self.i_ += 1;
                    }
                }
            } else {
                break;
            }
        }

        // Return what's left
        self.argv[self.i_..].to_vec()
    }
}
