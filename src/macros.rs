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
// | Author: Sean Kerr <sean@code-box.org>                                                         |
// +-----------------------------------------------------------------------------------------------+

//! Byte stream collection macros.

/// Retrieve the amount of readable bytes.
#[macro_export]
macro_rules! bs_available {
    ($context:expr) => ({
        $context.stream.len() - bs_index!($context)
    });
}

/// Iterate `$context.stream`, and for each byte execute `$on_byte`. Upon locating end-of-stream
/// execute `$on_eos`.
#[macro_export]
macro_rules! bs_collect {
    ($context:expr, $on_byte:expr, $on_eos:expr) => ({
        loop {
            if bs_is_eos!($context) {
                $on_eos
            } else {
                bs_next!($context);

                $on_byte
            }
        }
    });
}

/// Collect all sequential digit bytes into `$var` (u64), and convert them into an unsigned integer.
/// If `$on_byte` is supplied, for each new byte execute `$on_byte`. Upon locating end-of-stream
/// execute `$on_eos`.
///
/// Exit the collection loop upon locating a non-digit byte.
#[macro_export]
macro_rules! bs_collect_digits {
    ($context:expr, $var:expr, $on_byte:expr, $on_eos:expr) => ({
        bs_collect!($context,
            if is_digit!($context.byte) {
                $var *= 10;
                $var += ($context.byte - b'0') as u64;

                $on_byte
            } else {
                break;
            },
            $on_eos
        );
    });

    ($context:expr, $var:expr, $on_eos:expr) => ({
        bs_collect!($context,
            if is_digit!($context.byte) {
                $var *= 10;
                $var += ($context.byte - b'0') as u64;
            } else {
                break;
            },
            $on_eos
        );
    });
}

/// Collect all sequential hex bytes into `$var` (u64), and convert them into an unsigned integer. If
/// `$on_byte` is supplied, for each new byte execute `$on_byte`. Upon locating end-of-stream
/// execute `$on_eos`.
///
/// Exit the collection loop upon locating a non-hex byte.
#[macro_export]
macro_rules! bs_collect_hex {
    ($context:expr, $var:expr, $on_byte:expr, $on_eos:expr) => ({
        bs_collect!($context,
            if $context.byte > 0x2F && $context.byte < 0x3A {
                $var <<= 4;
                $var  |= ($context.byte - 0x30) as u64;

                $on_byte
            } else if $context.byte > 0x40 && $context.byte < 0x47 {
                $var <<= 4;
                $var  |= ($context.byte - 0x37) as u64;

                $on_byte
            } else if $context.byte > 0x60 && $context.byte < 0x67 {
                $var <<= 4;
                $var  |= ($context.byte - 0x57) as u64;

                $on_byte
            } else {
                break;
            },
            $on_eos
        );
    });

    ($context:expr, $var:expr, $on_eos:expr) => ({
        bs_collect_hex!($context, $var, {}, $on_eos)
    });
}

/// Collect `$length` bytes. If `$on_byte` and `$on_eos` are supplied, for each new byte execute
/// `$on_byte`. Upon locating end-of-stream execute `$on_eos`.
///
/// When `$on_byte` and `$on_eos` are not supplied, this macro assumes that `$length` bytes are
/// available for reading, and immediately advances the `$context.stream_index` by `$length` bytes.
#[macro_export]
macro_rules! bs_collect_length {
    ($context:expr, $length:expr, $on_byte:expr, $on_eos:expr) => ({
        bs_collect!($context, {
                $on_byte

                if bs_index!($context) == $context.mark_index + $length {
                    break;
                }
            },
            $on_eos
        );
    });

    ($context:expr, $length:expr) => ({
        if bs_available!($context) >= $length {
            $context.stream_index += $length;
        } else {
            $context.stream_index = $context.stream.len();
        }
    });
}

/// Collect if `$when` yields `true`.
///
/// Exit the collection loop when `$when` yields `false`.
#[macro_export]
macro_rules! bs_collect_when {
    ($context:expr, $when:expr, $on_eos:expr) => ({
        bs_collect!($context,
            if $when {
            } else {
                break;
            },
            $on_eos
        );
    });
}

/// Count each occurrence of `$byte` starting at `$context.stream_index` until end-of-stream.
#[macro_export]
macro_rules! bs_count {
    ($context:expr, $byte:expr) => ({
        let mut count = 0;

        for n in bs_index!($context)..bs_available!($context) {
            if $context.stream[n] == $byte {
                count += 1;
            }
        }

        count
    });
}

/// Count each byte loop starting at `$context.stream_index` until end-of-stream, when `$when`
/// yields `true`.
#[macro_export]
macro_rules! bs_count_when {
    ($context:expr, $when:expr) => ({
        let mut count = 0;

        for n in bs_index!($context)..bs_available!($context) {
            $context.byte = $context.stream[n];

            if $when {
                count += 1;
            }
        }

        $context.byte = $context.stream[bs_index!($context)];

        count
    });
}

/// Find the first occurrence of `$byte` and return the index relative to `$context.stream_index`.
///
/// `$start` is the starting index relative to `$context.stream_index`.
#[macro_export]
macro_rules! bs_find {
    ($context:expr, $start:expr, $byte:expr) => ({
        let mut index = None;

        if bs_index!($context) + $start < $context.stream.len() {
            for n in bs_index!($context) + $start..$context.stream.len() {
                if $byte == $context.stream[n] {
                    index = Some(n);
                    break;
                }
            }
        }

        index
    });

    ($context:expr, $byte:expr) => (
        bs_find!($context, 0, $byte);
    );
}

/// Find the first occurrence of `$pattern` and return the index relative to
/// `$context.stream_index`.
///
/// `$start` is the starting index relative to `$context.stream_index`.
#[macro_export]
macro_rules! bs_find_pattern {
    ($context:expr, $start:expr, $pattern:expr) => ({
        let mut index = None;

        if bs_index!($context) + $start < $context.stream.len() {
            'outer:
            for s in bs_index!($context) + $start..$context.stream.len() {
                for p in 0..$pattern.len() {
                    if $context.stream.len() <= s + p || $pattern[p] != $context.stream[s + p] {
                        break;
                    } else if $pattern.len() == p + 1 {
                        index = Some(s);

                        break 'outer;
                    }
                }
            }
        }

        index
    });

    ($context:expr, $pattern:expr) => (
        bs_find_pattern!($context, 0, $pattern);
    );
}

/// Indicates that a specified amount of bytes are available for reading.
#[macro_export]
macro_rules! bs_has_bytes {
    ($context:expr, $length:expr) => (
        bs_index!($context) + $length <= $context.stream.len()
    );
}

/// Retrieve the current stream index.
#[macro_export]
macro_rules! bs_index {
    ($context:expr) => (
        $context.stream_index
    );
}

/// Indicates that we're at the end of the stream.
#[macro_export]
macro_rules! bs_is_eos {
    ($context:expr) => ({
        bs_index!($context) == $context.stream.len()
    });
}

/// Jump `$length` bytes.
///
/// This macro assumes that `$length` bytes are available for reading.
#[macro_export]
macro_rules! bs_jump {
    ($context:expr, $length:expr) => ({
        $context.stream_index += $length;
    });
}

/// Set `$context.mark_index` to the current stream index or `$index`.
#[macro_export]
macro_rules! bs_mark {
    ($context:expr) => ({
        $context.mark_index = bs_index!($context);
    });

    ($context:expr, $index:expr) => ({
        $context.mark_index = $index;
    });
}

/// Advance `$context.stream_index` one byte and set `$context.byte` to the new byte.
#[macro_export]
macro_rules! bs_next {
    ($context:expr) => ({
        $context.byte          = $context.stream[bs_index!($context)];
        $context.stream_index += 1;
    });
}

/// Peek at a slice of bytes.
///
/// This macro assumes that `$length` bytes are available for reading.
#[macro_export]
macro_rules! bs_peek {
    ($context:expr, $length:expr) => (
        &$context.stream[bs_index!($context)..bs_index!($context) + $length]
    );
}

/// Replay the most recent byte, but do not change the current `$context.byte`.
#[macro_export]
macro_rules! bs_replay {
    ($context:expr) => ({
        bs_rewind!($context, 1);
    });
}

/// Rewind `$context.stream_index` by `$length` bytes, but do not change the current
/// `$context.byte`.
#[macro_export]
macro_rules! bs_rewind {
    ($context:expr, $length:expr) => ({
        $context.stream_index -= $length;
    });
}

/// Rewind `$context.stream_index` to index `$index`, but do not change the current `$context.byte`.
#[macro_export]
macro_rules! bs_rewind_to {
    ($context:expr, $index:expr) => ({
        $context.stream_index = $index;
    });
}

/// Retrieve the slice of marked bytes.
#[macro_export]
macro_rules! bs_slice {
    ($context:expr) => (
        &$context.stream[$context.mark_index..bs_index!($context)];
    );
}

/// Retrieve the slice of marked bytes ignoring the very last byte.
#[macro_export]
macro_rules! bs_slice_ignore {
    ($context:expr) => (
        &$context.stream[$context.mark_index..bs_index!($context) - 1];
    );
}

/// Retrieve the length of marked bytes.
#[macro_export]
macro_rules! bs_slice_length {
    ($context:expr) => ({
        bs_index!($context) - $context.mark_index
    });
}

/// Determine if the remaining stream starts with `$pattern`.
#[macro_export]
macro_rules! bs_starts_with {
    ($context:expr, $pattern:expr) => ({
        let mut found = false;

        if $pattern.len() <= bs_available!($context) {
            for n in 0..$pattern.len() {
                if $context.stream[bs_index!($context) + n] != $pattern[n] {
                    found = false;

                    break;
                }

                found = true;
            }
        }

        found
    });
}

/// Indicates that a byte is alphabetical.
#[macro_export]
macro_rules! is_alpha {
    ($byte:expr) => ({
           ($byte > 0x40 && $byte < 0x5B)
        || ($byte > 0x60 && $byte < 0x7B)
    });
}

/// Indicates that a byte is a control character.
#[macro_export]
macro_rules! is_control {
    ($byte:expr) => ({
        $byte < 0x20 || $byte == 0x7F
    });
}

/// Indicates that a byte is a digit.
#[macro_export]
macro_rules! is_digit {
    ($byte:expr) => ({
        $byte > 0x2F && $byte < 0x3A
    });
}

/// Indicates that a byte is a hex character.
#[macro_export]
macro_rules! is_hex {
    ($byte:expr) => ({
           ($byte > 0x2F && $byte < 0x3A)
        || ($byte > 0x40 && $byte < 0x47)
        || ($byte > 0x60 && $byte < 0x67)
    });
}

/// Indicates that a byte is not a visible 7-bit character. Space are not considered visible.
#[macro_export]
macro_rules! is_not_visible_7bit {
    ($byte:expr) => ({
        $byte < 0x21 || $byte > 0x7E
    })
}

/// Indicates that a byte is not a visible 8-bit character. Space not considered visible.
#[macro_export]
macro_rules! is_not_visible_8bit {
    ($byte:expr) => ({
           $byte < 0x21 || $byte == 0x7F || $byte == 0xFF
    })
}

/// Indicates that a byte is a visible 7-bit character. Space is not considered visible.
#[macro_export]
macro_rules! is_visible_7bit {
    ($byte:expr) => ({
        $byte > 0x20 && $byte < 0x7F
    })
}

/// Indicates that a byte is a visible 8-bit character. Space is not considered visible.
#[macro_export]
macro_rules! is_visible_8bit {
    ($byte:expr) => ({
           $byte > 0x20 && $byte < 0xFF && $byte != 0x7F
    })
}
