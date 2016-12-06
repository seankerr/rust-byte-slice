// +-----------------------------------------------------------------------------------------------+
// | Copyright 2016 Sean Kerr                                                                      |
// |                                                                                               |
// | Licensed under the Apache License, Version 2.0 (the "License");                               |
// | you may not use this file except in compliance with the License.                              |
// | You may obtain a copy of the License at                                                       |
// |                                                                                               |
// |  http://www.apache.org/licenses/LICENSE-2.0                                                   |
// |                                                                                               |
// | Unless required by applicable law or agreed to in writing, software                           |
// | distributed under the License is distributed on an "AS IS" BASIS,                             |
// | WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.                      |
// | See the License for the specific language governing permissions and                           |
// | limitations under the License.                                                                |
// +-----------------------------------------------------------------------------------------------+
// | Author: Sean Kerr <sean@metatomic.io>                                                         |
// +-----------------------------------------------------------------------------------------------+

#[macro_use]
pub mod macros;

#[cfg(test)]
mod test;

use std::fmt;

/// Default byte stream type.
///
/// All stream macros will accept any struct given as `$context`, as long as they contain the
/// following four fields:
///
/// - `byte` (u8) The most recent byte.
/// - `mark_index` (usize) Starting index of a collection of marked bytes.
/// - `stream` (&[u8]) Stream of bytes.
/// - `stream_index` (usize) Current stream index.
pub struct ByteStream<'a> {
    /// Current byte.
    pub byte: u8,

    /// Callback mark index.
    pub mark_index: usize,

    /// Stream data.
    pub stream: &'a [u8],

    /// Stream index.
    pub stream_index: usize
}

impl<'a> ByteStream<'a> {
    /// Create a new `ByteStream`.
    pub fn new(stream: &'a [u8]) -> ByteStream<'a> {
        ByteStream{ byte:         0,
                 mark_index:   0,
                 stream:       stream,
                 stream_index: 0 }
    }
}

impl<'a> fmt::Debug for ByteStream<'a> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        if is_visible_8bit!(self.byte) || self.byte == 0x20 || self.byte == 0xFF {
            write!(formatter, "ByteStream(byte[{}]='{}', mark_index={}, stream_index={})",
                   self.byte, self.byte as char, self.mark_index, self.stream_index)
        } else {
            write!(formatter, "ByteStream(byte[{}]='', mark_index={}, stream_index={})",
                   self.byte, self.mark_index, self.stream_index)
        }
    }
}

impl<'a> fmt::Display for ByteStream<'a> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        if is_visible_8bit!(self.byte) || self.byte == 0x20 || self.byte == 0xFF {
            write!(formatter, "byte[{}]='{}', mark_index={}, stream_index={}",
                   self.byte, self.byte as char, self.mark_index, self.stream_index)
        } else {
            write!(formatter, "byte[{}]='', mark_index={}, stream_index={}",
                   self.byte, self.mark_index, self.stream_index)
        }
    }
}
