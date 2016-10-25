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

/// Collect all sequential digit bytes into `$var` (u8), and convert them into an unsigned integer.
/// If `$on_byte` is supplied, for each new byte execute `$on_byte`. Upon locating end-of-stream
/// execute `$on_eos`. If an overflow would occur, execute `$on_overflow`.
///
/// Exit the collection loop upon locating a non-digit byte.
#[macro_export]
macro_rules! bs_collect_digits8 {
    ($context:expr, $var:expr, $on_byte:expr, $on_overflow:expr, $on_eos:expr) => ({
        bs_collect!($context,
            if is_digit!($context.byte) {
                if let Some(value) = ($var as u8).checked_mul(10) {
                    if let Some(value) = value.checked_add($context.byte - b'0') {
                        $var = value;
                        $on_byte
                    } else {
                        $on_overflow;
                    }
                } else {
                    $on_overflow;
                }
            } else {
                break;
            },
            $on_eos
        );
    });

    ($context:expr, $var:expr, $on_overflow:expr, $on_eos:expr) => ({
        bs_collect!($context,
            if is_digit!($context.byte) {
                if let Some(value) = ($var as u8).checked_mul(10) {
                    if let Some(value) = value.checked_add($context.byte - b'0') {
                        $var = value;
                    } else {
                        $on_overflow;
                    }
                } else {
                    $on_overflow;
                }
            } else {
                break;
            },
            $on_eos
        );
    });
}

/// Collect all sequential digit bytes into `$var` (u16), and convert them into an unsigned integer.
/// If `$on_byte` is supplied, for each new byte execute `$on_byte`. Upon locating end-of-stream
/// execute `$on_eos`. If an overflow would occur, execute `$on_overflow`.
///
/// Exit the collection loop upon locating a non-digit byte.
#[macro_export]
macro_rules! bs_collect_digits16 {
    ($context:expr, $var:expr, $on_byte:expr, $on_overflow:expr, $on_eos:expr) => ({
        bs_collect!($context,
            if is_digit!($context.byte) {
                if let Some(value) = ($var as u16).checked_mul(10) {
                    if let Some(value) = value.checked_add(($context.byte - b'0') as u16) {
                        $var = value;
                        $on_byte
                    } else {
                        $on_overflow
                    }
                } else {
                    $on_overflow
                }
            } else {
                break;
            },
            $on_eos
        );
    });

    ($context:expr, $var:expr, $on_overflow:expr, $on_eos:expr) => ({
        bs_collect!($context,
            if is_digit!($context.byte) {
                if let Some(value) = ($var as u16).checked_mul(10) {
                    if let Some(value) = value.checked_add(($context.byte - b'0') as u16) {
                        $var = value;
                    } else {
                        $on_overflow
                    }
                } else {
                    $on_overflow
                }
            } else {
                break;
            },
            $on_eos
        );
    });
}

/// Collect all sequential digit bytes into `$var` (u32), and convert them into an unsigned integer.
/// If `$on_byte` is supplied, for each new byte execute `$on_byte`. Upon locating end-of-stream
/// execute `$on_eos`. If an overflow would occur, execute `$on_overflow`.
///
/// Exit the collection loop upon locating a non-digit byte.
#[macro_export]
macro_rules! bs_collect_digits32 {
    ($context:expr, $var:expr, $on_byte:expr, $on_overflow:expr, $on_eos:expr) => ({
        bs_collect!($context,
            if is_digit!($context.byte) {
                if let Some(value) = ($var as u32).checked_mul(10) {
                    if let Some(value) = value.checked_add(($context.byte - b'0') as u32) {
                        $var = value;
                        $on_byte
                    } else {
                        $on_overflow
                    }
                } else {
                    $on_overflow
                }
            } else {
                break;
            },
            $on_eos
        );
    });

    ($context:expr, $var:expr, $on_overflow:expr, $on_eos:expr) => ({
        bs_collect!($context,
            if is_digit!($context.byte) {
                if let Some(value) = ($var as u32).checked_mul(10) {
                    if let Some(value) = value.checked_add(($context.byte - b'0') as u32) {
                        $var = value;
                    } else {
                        $on_overflow
                    }
                } else {
                    $on_overflow
                }
            } else {
                break;
            },
            $on_eos
        );
    });
}

/// Collect all sequential digit bytes into `$var` (u64), and convert them into an unsigned integer.
/// If `$on_byte` is supplied, for each new byte execute `$on_byte`. Upon locating end-of-stream
/// execute `$on_eos`. If an overflow would occur, execute `$on_overflow`.
///
/// Exit the collection loop upon locating a non-digit byte.
#[macro_export]
macro_rules! bs_collect_digits64 {
    ($context:expr, $var:expr, $on_byte:expr, $on_overflow:expr, $on_eos:expr) => ({
        bs_collect!($context,
            if is_digit!($context.byte) {
                if let Some(value) = ($var as u64).checked_mul(10) {
                    if let Some(value) = value.checked_add(($context.byte - b'0') as u64) {
                        $var = value;
                        $on_byte
                    } else {
                        $on_overflow
                    }
                } else {
                    $on_overflow
                }
            } else {
                break;
            },
            $on_eos
        );
    });

    ($context:expr, $var:expr, $on_overflow:expr, $on_eos:expr) => ({
        bs_collect!($context,
            if is_digit!($context.byte) {
                if let Some(value) = ($var as u64).checked_mul(10) {
                    if let Some(value) = value.checked_add(($context.byte - b'0') as u64) {
                        $var = value;
                    } else {
                        $on_overflow
                    }
                } else {
                    $on_overflow
                }
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
/// Exit the collection loop if `$when` yields `false`.
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

/// Collect if `$until` yields `false`.
///
/// Exit the collection loop if `$until` yields `true`.
#[macro_export]
macro_rules! bs_collect_until {
    ($context:expr, $until:expr, $on_eos:expr) => ({
        bs_collect!($context,
            if $until {
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

/// Count each byte loop starting at `$context.stream_index` until end-of-stream, if `$when`
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
                for (p, byte) in $pattern.iter().enumerate() {
                    if $context.stream.len() <= s + p || *byte != $context.stream[s + p] {
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

/// Retrieve the remaining available bytes.
#[macro_export]
macro_rules! bs_remaining {
    ($context:expr) => (
        &$context.stream[bs_index!($context)..]
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
///
/// This macro assumes that `$pattern.len()` bytes are available for reading.
#[macro_export]
macro_rules! bs_starts_with {
    ($context:expr, $pattern:expr) => ({
        let mut found = false;

        for (n, byte) in $pattern.iter().enumerate() {
            if $context.stream[bs_index!($context) + n] != *byte {
                found = false;

                break;
            }

            found = true;
        }

        found
    });
}

/// Determine if the remaining stream starts with `$pattern`, comparing only the first byte.
///
/// This macro assumes that `$pattern.len()` bytes are available for reading.
#[macro_export]
macro_rules! bs_starts_with1 {
    ($context:expr, $pattern:expr) => ({
        $context.stream[$context.stream_index] == $pattern[0]
    });
}

/// Determine if the remaining stream starts with `$pattern`, comparing only the first 2 bytes.
///
/// This macro assumes that `$pattern.len()` bytes are available for reading.
#[macro_export]
macro_rules! bs_starts_with2 {
    ($context:expr, $pattern:expr) => ({
           $context.stream[$context.stream_index]     == $pattern[0]
        && $context.stream[$context.stream_index + 1] == $pattern[1]
    });
}

/// Determine if the remaining stream starts with `$pattern`, comparing only the first 3 bytes.
///
/// This macro assumes that `$pattern.len()` bytes are available for reading.
#[macro_export]
macro_rules! bs_starts_with3 {
    ($context:expr, $pattern:expr) => ({
           $context.stream[$context.stream_index]     == $pattern[0]
        && $context.stream[$context.stream_index + 1] == $pattern[1]
        && $context.stream[$context.stream_index + 2] == $pattern[2]
    });
}

/// Determine if the remaining stream starts with `$pattern`, comparing only the first 4 bytes.
///
/// This macro assumes that `$pattern.len()` bytes are available for reading.
#[macro_export]
macro_rules! bs_starts_with4 {
    ($context:expr, $pattern:expr) => ({
           $context.stream[$context.stream_index]     == $pattern[0]
        && $context.stream[$context.stream_index + 1] == $pattern[1]
        && $context.stream[$context.stream_index + 2] == $pattern[2]
        && $context.stream[$context.stream_index + 3] == $pattern[3]
    });
}

/// Determine if the remaining stream starts with `$pattern`, comparing only the first 5 bytes.
///
/// This macro assumes that `$pattern.len()` bytes are available for reading.
#[macro_export]
macro_rules! bs_starts_with5 {
    ($context:expr, $pattern:expr) => ({
           $context.stream[$context.stream_index]     == $pattern[0]
        && $context.stream[$context.stream_index + 1] == $pattern[1]
        && $context.stream[$context.stream_index + 2] == $pattern[2]
        && $context.stream[$context.stream_index + 3] == $pattern[3]
        && $context.stream[$context.stream_index + 4] == $pattern[4]
    });
}

/// Determine if the remaining stream starts with `$pattern`, comparing only the first 6 bytes.
///
/// This macro assumes that `$pattern.len()` bytes are available for reading.
#[macro_export]
macro_rules! bs_starts_with6 {
    ($context:expr, $pattern:expr) => ({
           $context.stream[$context.stream_index]     == $pattern[0]
        && $context.stream[$context.stream_index + 1] == $pattern[1]
        && $context.stream[$context.stream_index + 2] == $pattern[2]
        && $context.stream[$context.stream_index + 3] == $pattern[3]
        && $context.stream[$context.stream_index + 4] == $pattern[4]
        && $context.stream[$context.stream_index + 5] == $pattern[5]
    });
}

/// Determine if the remaining stream starts with `$pattern`, comparing only the first 7 bytes.
///
/// This macro assumes that `$pattern.len()` bytes are available for reading.
#[macro_export]
macro_rules! bs_starts_with7 {
    ($context:expr, $pattern:expr) => ({
           $context.stream[$context.stream_index]     == $pattern[0]
        && $context.stream[$context.stream_index + 1] == $pattern[1]
        && $context.stream[$context.stream_index + 2] == $pattern[2]
        && $context.stream[$context.stream_index + 3] == $pattern[3]
        && $context.stream[$context.stream_index + 4] == $pattern[4]
        && $context.stream[$context.stream_index + 5] == $pattern[5]
        && $context.stream[$context.stream_index + 6] == $pattern[6]
    });
}

/// Determine if the remaining stream starts with `$pattern`, comparing only the first 8 bytes.
///
/// This macro assumes that `$pattern.len()` bytes are available for reading.
#[macro_export]
macro_rules! bs_starts_with8 {
    ($context:expr, $pattern:expr) => ({
           $context.stream[$context.stream_index]     == $pattern[0]
        && $context.stream[$context.stream_index + 1] == $pattern[1]
        && $context.stream[$context.stream_index + 2] == $pattern[2]
        && $context.stream[$context.stream_index + 3] == $pattern[3]
        && $context.stream[$context.stream_index + 4] == $pattern[4]
        && $context.stream[$context.stream_index + 5] == $pattern[5]
        && $context.stream[$context.stream_index + 6] == $pattern[6]
        && $context.stream[$context.stream_index + 7] == $pattern[7]
    });
}

/// Determine if the remaining stream starts with `$pattern`, comparing only the first 9 bytes.
///
/// This macro assumes that `$pattern.len()` bytes are available for reading.
#[macro_export]
macro_rules! bs_starts_with9 {
    ($context:expr, $pattern:expr) => ({
           $context.stream[$context.stream_index]     == $pattern[0]
        && $context.stream[$context.stream_index + 1] == $pattern[1]
        && $context.stream[$context.stream_index + 2] == $pattern[2]
        && $context.stream[$context.stream_index + 3] == $pattern[3]
        && $context.stream[$context.stream_index + 4] == $pattern[4]
        && $context.stream[$context.stream_index + 5] == $pattern[5]
        && $context.stream[$context.stream_index + 6] == $pattern[6]
        && $context.stream[$context.stream_index + 7] == $pattern[7]
        && $context.stream[$context.stream_index + 8] == $pattern[8]
    });
}

/// Determine if the remaining stream starts with `$pattern`, comparing only the first 10 bytes.
///
/// This macro assumes that `$pattern.len()` bytes are available for reading.
#[macro_export]
macro_rules! bs_starts_with10 {
    ($context:expr, $pattern:expr) => ({
           $context.stream[$context.stream_index]     == $pattern[0]
        && $context.stream[$context.stream_index + 1] == $pattern[1]
        && $context.stream[$context.stream_index + 2] == $pattern[2]
        && $context.stream[$context.stream_index + 3] == $pattern[3]
        && $context.stream[$context.stream_index + 4] == $pattern[4]
        && $context.stream[$context.stream_index + 5] == $pattern[5]
        && $context.stream[$context.stream_index + 6] == $pattern[6]
        && $context.stream[$context.stream_index + 7] == $pattern[7]
        && $context.stream[$context.stream_index + 8] == $pattern[8]
        && $context.stream[$context.stream_index + 9] == $pattern[9]
    });
}

/// Determine if the remaining stream starts with `$pattern`, comparing only the first 11 bytes.
///
/// This macro assumes that `$pattern.len()` bytes are available for reading.
#[macro_export]
macro_rules! bs_starts_with11 {
    ($context:expr, $pattern:expr) => ({
           $context.stream[$context.stream_index]      == $pattern[0]
        && $context.stream[$context.stream_index + 1]  == $pattern[1]
        && $context.stream[$context.stream_index + 2]  == $pattern[2]
        && $context.stream[$context.stream_index + 3]  == $pattern[3]
        && $context.stream[$context.stream_index + 4]  == $pattern[4]
        && $context.stream[$context.stream_index + 5]  == $pattern[5]
        && $context.stream[$context.stream_index + 6]  == $pattern[6]
        && $context.stream[$context.stream_index + 7]  == $pattern[7]
        && $context.stream[$context.stream_index + 8]  == $pattern[8]
        && $context.stream[$context.stream_index + 9]  == $pattern[9]
        && $context.stream[$context.stream_index + 10] == $pattern[10]
    });
}

/// Determine if the remaining stream starts with `$pattern`, comparing only the first 12 bytes.
///
/// This macro assumes that `$pattern.len()` bytes are available for reading.
#[macro_export]
macro_rules! bs_starts_with12 {
    ($context:expr, $pattern:expr) => ({
           $context.stream[$context.stream_index]      == $pattern[0]
        && $context.stream[$context.stream_index + 1]  == $pattern[1]
        && $context.stream[$context.stream_index + 2]  == $pattern[2]
        && $context.stream[$context.stream_index + 3]  == $pattern[3]
        && $context.stream[$context.stream_index + 4]  == $pattern[4]
        && $context.stream[$context.stream_index + 5]  == $pattern[5]
        && $context.stream[$context.stream_index + 6]  == $pattern[6]
        && $context.stream[$context.stream_index + 7]  == $pattern[7]
        && $context.stream[$context.stream_index + 8]  == $pattern[8]
        && $context.stream[$context.stream_index + 9]  == $pattern[9]
        && $context.stream[$context.stream_index + 10] == $pattern[10]
        && $context.stream[$context.stream_index + 11] == $pattern[11]
    });
}

/// Determine if the remaining stream starts with `$pattern`, comparing only the first 13 bytes.
///
/// This macro assumes that `$pattern.len()` bytes are available for reading.
#[macro_export]
macro_rules! bs_starts_with13 {
    ($context:expr, $pattern:expr) => ({
           $context.stream[$context.stream_index]      == $pattern[0]
        && $context.stream[$context.stream_index + 1]  == $pattern[1]
        && $context.stream[$context.stream_index + 2]  == $pattern[2]
        && $context.stream[$context.stream_index + 3]  == $pattern[3]
        && $context.stream[$context.stream_index + 4]  == $pattern[4]
        && $context.stream[$context.stream_index + 5]  == $pattern[5]
        && $context.stream[$context.stream_index + 6]  == $pattern[6]
        && $context.stream[$context.stream_index + 7]  == $pattern[7]
        && $context.stream[$context.stream_index + 8]  == $pattern[8]
        && $context.stream[$context.stream_index + 9]  == $pattern[9]
        && $context.stream[$context.stream_index + 10] == $pattern[10]
        && $context.stream[$context.stream_index + 11] == $pattern[11]
        && $context.stream[$context.stream_index + 12] == $pattern[12]
    });
}

/// Determine if the remaining stream starts with `$pattern`, comparing only the first 14 bytes.
///
/// This macro assumes that `$pattern.len()` bytes are available for reading.
#[macro_export]
macro_rules! bs_starts_with14 {
    ($context:expr, $pattern:expr) => ({
           $context.stream[$context.stream_index]      == $pattern[0]
        && $context.stream[$context.stream_index + 1]  == $pattern[1]
        && $context.stream[$context.stream_index + 2]  == $pattern[2]
        && $context.stream[$context.stream_index + 3]  == $pattern[3]
        && $context.stream[$context.stream_index + 4]  == $pattern[4]
        && $context.stream[$context.stream_index + 5]  == $pattern[5]
        && $context.stream[$context.stream_index + 6]  == $pattern[6]
        && $context.stream[$context.stream_index + 7]  == $pattern[7]
        && $context.stream[$context.stream_index + 8]  == $pattern[8]
        && $context.stream[$context.stream_index + 9]  == $pattern[9]
        && $context.stream[$context.stream_index + 10] == $pattern[10]
        && $context.stream[$context.stream_index + 11] == $pattern[11]
        && $context.stream[$context.stream_index + 12] == $pattern[12]
        && $context.stream[$context.stream_index + 13] == $pattern[13]
    });
}

/// Determine if the remaining stream starts with `$pattern`, comparing only the first 15 bytes.
///
/// This macro assumes that `$pattern.len()` bytes are available for reading.
#[macro_export]
macro_rules! bs_starts_with15 {
    ($context:expr, $pattern:expr) => ({
           $context.stream[$context.stream_index]      == $pattern[0]
        && $context.stream[$context.stream_index + 1]  == $pattern[1]
        && $context.stream[$context.stream_index + 2]  == $pattern[2]
        && $context.stream[$context.stream_index + 3]  == $pattern[3]
        && $context.stream[$context.stream_index + 4]  == $pattern[4]
        && $context.stream[$context.stream_index + 5]  == $pattern[5]
        && $context.stream[$context.stream_index + 6]  == $pattern[6]
        && $context.stream[$context.stream_index + 7]  == $pattern[7]
        && $context.stream[$context.stream_index + 8]  == $pattern[8]
        && $context.stream[$context.stream_index + 9]  == $pattern[9]
        && $context.stream[$context.stream_index + 10] == $pattern[10]
        && $context.stream[$context.stream_index + 11] == $pattern[11]
        && $context.stream[$context.stream_index + 12] == $pattern[12]
        && $context.stream[$context.stream_index + 13] == $pattern[13]
        && $context.stream[$context.stream_index + 14] == $pattern[14]
    });
}

/// Determine if the remaining stream starts with `$pattern`, comparing only the first 16 bytes.
///
/// This macro assumes that `$pattern.len()` bytes are available for reading.
#[macro_export]
macro_rules! bs_starts_with16 {
    ($context:expr, $pattern:expr) => ({
           $context.stream[$context.stream_index]      == $pattern[0]
        && $context.stream[$context.stream_index + 1]  == $pattern[1]
        && $context.stream[$context.stream_index + 2]  == $pattern[2]
        && $context.stream[$context.stream_index + 3]  == $pattern[3]
        && $context.stream[$context.stream_index + 4]  == $pattern[4]
        && $context.stream[$context.stream_index + 5]  == $pattern[5]
        && $context.stream[$context.stream_index + 6]  == $pattern[6]
        && $context.stream[$context.stream_index + 7]  == $pattern[7]
        && $context.stream[$context.stream_index + 8]  == $pattern[8]
        && $context.stream[$context.stream_index + 9]  == $pattern[9]
        && $context.stream[$context.stream_index + 10] == $pattern[10]
        && $context.stream[$context.stream_index + 11] == $pattern[11]
        && $context.stream[$context.stream_index + 12] == $pattern[12]
        && $context.stream[$context.stream_index + 13] == $pattern[13]
        && $context.stream[$context.stream_index + 14] == $pattern[14]
        && $context.stream[$context.stream_index + 15] == $pattern[15]
    });
}

/// Determine if the remaining stream starts with `$pattern`, comparing only the first 17 bytes.
///
/// This macro assumes that `$pattern.len()` bytes are available for reading.
#[macro_export]
macro_rules! bs_starts_with17 {
    ($context:expr, $pattern:expr) => ({
           $context.stream[$context.stream_index]      == $pattern[0]
        && $context.stream[$context.stream_index + 1]  == $pattern[1]
        && $context.stream[$context.stream_index + 2]  == $pattern[2]
        && $context.stream[$context.stream_index + 3]  == $pattern[3]
        && $context.stream[$context.stream_index + 4]  == $pattern[4]
        && $context.stream[$context.stream_index + 5]  == $pattern[5]
        && $context.stream[$context.stream_index + 6]  == $pattern[6]
        && $context.stream[$context.stream_index + 7]  == $pattern[7]
        && $context.stream[$context.stream_index + 8]  == $pattern[8]
        && $context.stream[$context.stream_index + 9]  == $pattern[9]
        && $context.stream[$context.stream_index + 10] == $pattern[10]
        && $context.stream[$context.stream_index + 11] == $pattern[11]
        && $context.stream[$context.stream_index + 12] == $pattern[12]
        && $context.stream[$context.stream_index + 13] == $pattern[13]
        && $context.stream[$context.stream_index + 14] == $pattern[14]
        && $context.stream[$context.stream_index + 15] == $pattern[15]
        && $context.stream[$context.stream_index + 16] == $pattern[16]
    });
}

/// Determine if the remaining stream starts with `$pattern`, comparing only the first 18 bytes.
///
/// This macro assumes that `$pattern.len()` bytes are available for reading.
#[macro_export]
macro_rules! bs_starts_with18 {
    ($context:expr, $pattern:expr) => ({
           $context.stream[$context.stream_index]      == $pattern[0]
        && $context.stream[$context.stream_index + 1]  == $pattern[1]
        && $context.stream[$context.stream_index + 2]  == $pattern[2]
        && $context.stream[$context.stream_index + 3]  == $pattern[3]
        && $context.stream[$context.stream_index + 4]  == $pattern[4]
        && $context.stream[$context.stream_index + 5]  == $pattern[5]
        && $context.stream[$context.stream_index + 6]  == $pattern[6]
        && $context.stream[$context.stream_index + 7]  == $pattern[7]
        && $context.stream[$context.stream_index + 8]  == $pattern[8]
        && $context.stream[$context.stream_index + 9]  == $pattern[9]
        && $context.stream[$context.stream_index + 10] == $pattern[10]
        && $context.stream[$context.stream_index + 11] == $pattern[11]
        && $context.stream[$context.stream_index + 12] == $pattern[12]
        && $context.stream[$context.stream_index + 13] == $pattern[13]
        && $context.stream[$context.stream_index + 14] == $pattern[14]
        && $context.stream[$context.stream_index + 15] == $pattern[15]
        && $context.stream[$context.stream_index + 16] == $pattern[16]
        && $context.stream[$context.stream_index + 17] == $pattern[17]
    });
}

/// Determine if the remaining stream starts with `$pattern`, comparing only the first 19 bytes.
///
/// This macro assumes that `$pattern.len()` bytes are available for reading.
#[macro_export]
macro_rules! bs_starts_with19 {
    ($context:expr, $pattern:expr) => ({
           $context.stream[$context.stream_index]      == $pattern[0]
        && $context.stream[$context.stream_index + 1]  == $pattern[1]
        && $context.stream[$context.stream_index + 2]  == $pattern[2]
        && $context.stream[$context.stream_index + 3]  == $pattern[3]
        && $context.stream[$context.stream_index + 4]  == $pattern[4]
        && $context.stream[$context.stream_index + 5]  == $pattern[5]
        && $context.stream[$context.stream_index + 6]  == $pattern[6]
        && $context.stream[$context.stream_index + 7]  == $pattern[7]
        && $context.stream[$context.stream_index + 8]  == $pattern[8]
        && $context.stream[$context.stream_index + 9]  == $pattern[9]
        && $context.stream[$context.stream_index + 10] == $pattern[10]
        && $context.stream[$context.stream_index + 11] == $pattern[11]
        && $context.stream[$context.stream_index + 12] == $pattern[12]
        && $context.stream[$context.stream_index + 13] == $pattern[13]
        && $context.stream[$context.stream_index + 14] == $pattern[14]
        && $context.stream[$context.stream_index + 15] == $pattern[15]
        && $context.stream[$context.stream_index + 16] == $pattern[16]
        && $context.stream[$context.stream_index + 17] == $pattern[17]
        && $context.stream[$context.stream_index + 18] == $pattern[18]
    });
}

/// Determine if the remaining stream starts with `$pattern`, comparing only the first 20 bytes.
///
/// This macro assumes that `$pattern.len()` bytes are available for reading.
#[macro_export]
macro_rules! bs_starts_with20 {
    ($context:expr, $pattern:expr) => ({
           $context.stream[$context.stream_index]      == $pattern[0]
        && $context.stream[$context.stream_index + 1]  == $pattern[1]
        && $context.stream[$context.stream_index + 2]  == $pattern[2]
        && $context.stream[$context.stream_index + 3]  == $pattern[3]
        && $context.stream[$context.stream_index + 4]  == $pattern[4]
        && $context.stream[$context.stream_index + 5]  == $pattern[5]
        && $context.stream[$context.stream_index + 6]  == $pattern[6]
        && $context.stream[$context.stream_index + 7]  == $pattern[7]
        && $context.stream[$context.stream_index + 8]  == $pattern[8]
        && $context.stream[$context.stream_index + 9]  == $pattern[9]
        && $context.stream[$context.stream_index + 10] == $pattern[10]
        && $context.stream[$context.stream_index + 11] == $pattern[11]
        && $context.stream[$context.stream_index + 12] == $pattern[12]
        && $context.stream[$context.stream_index + 13] == $pattern[13]
        && $context.stream[$context.stream_index + 14] == $pattern[14]
        && $context.stream[$context.stream_index + 15] == $pattern[15]
        && $context.stream[$context.stream_index + 16] == $pattern[16]
        && $context.stream[$context.stream_index + 17] == $pattern[17]
        && $context.stream[$context.stream_index + 18] == $pattern[18]
        && $context.stream[$context.stream_index + 19] == $pattern[19]
    });
}

/// Determine if the remaining stream starts with `$pattern`, comparing only the first 21 bytes.
///
/// This macro assumes that `$pattern.len()` bytes are available for reading.
#[macro_export]
macro_rules! bs_starts_with21 {
    ($context:expr, $pattern:expr) => ({
           $context.stream[$context.stream_index]      == $pattern[0]
        && $context.stream[$context.stream_index + 1]  == $pattern[1]
        && $context.stream[$context.stream_index + 2]  == $pattern[2]
        && $context.stream[$context.stream_index + 3]  == $pattern[3]
        && $context.stream[$context.stream_index + 4]  == $pattern[4]
        && $context.stream[$context.stream_index + 5]  == $pattern[5]
        && $context.stream[$context.stream_index + 6]  == $pattern[6]
        && $context.stream[$context.stream_index + 7]  == $pattern[7]
        && $context.stream[$context.stream_index + 8]  == $pattern[8]
        && $context.stream[$context.stream_index + 9]  == $pattern[9]
        && $context.stream[$context.stream_index + 10] == $pattern[10]
        && $context.stream[$context.stream_index + 11] == $pattern[11]
        && $context.stream[$context.stream_index + 12] == $pattern[12]
        && $context.stream[$context.stream_index + 13] == $pattern[13]
        && $context.stream[$context.stream_index + 14] == $pattern[14]
        && $context.stream[$context.stream_index + 15] == $pattern[15]
        && $context.stream[$context.stream_index + 16] == $pattern[16]
        && $context.stream[$context.stream_index + 17] == $pattern[17]
        && $context.stream[$context.stream_index + 18] == $pattern[18]
        && $context.stream[$context.stream_index + 19] == $pattern[19]
        && $context.stream[$context.stream_index + 20] == $pattern[20]
    });
}

/// Determine if the remaining stream starts with `$pattern`, comparing only the first 22 bytes.
///
/// This macro assumes that `$pattern.len()` bytes are available for reading.
#[macro_export]
macro_rules! bs_starts_with22 {
    ($context:expr, $pattern:expr) => ({
           $context.stream[$context.stream_index]      == $pattern[0]
        && $context.stream[$context.stream_index + 1]  == $pattern[1]
        && $context.stream[$context.stream_index + 2]  == $pattern[2]
        && $context.stream[$context.stream_index + 3]  == $pattern[3]
        && $context.stream[$context.stream_index + 4]  == $pattern[4]
        && $context.stream[$context.stream_index + 5]  == $pattern[5]
        && $context.stream[$context.stream_index + 6]  == $pattern[6]
        && $context.stream[$context.stream_index + 7]  == $pattern[7]
        && $context.stream[$context.stream_index + 8]  == $pattern[8]
        && $context.stream[$context.stream_index + 9]  == $pattern[9]
        && $context.stream[$context.stream_index + 10] == $pattern[10]
        && $context.stream[$context.stream_index + 11] == $pattern[11]
        && $context.stream[$context.stream_index + 12] == $pattern[12]
        && $context.stream[$context.stream_index + 13] == $pattern[13]
        && $context.stream[$context.stream_index + 14] == $pattern[14]
        && $context.stream[$context.stream_index + 15] == $pattern[15]
        && $context.stream[$context.stream_index + 16] == $pattern[16]
        && $context.stream[$context.stream_index + 17] == $pattern[17]
        && $context.stream[$context.stream_index + 18] == $pattern[18]
        && $context.stream[$context.stream_index + 19] == $pattern[19]
        && $context.stream[$context.stream_index + 20] == $pattern[20]
        && $context.stream[$context.stream_index + 21] == $pattern[21]
    });
}

/// Determine if the remaining stream starts with `$pattern`, comparing only the first 23 bytes.
///
/// This macro assumes that `$pattern.len()` bytes are available for reading.
#[macro_export]
macro_rules! bs_starts_with23 {
    ($context:expr, $pattern:expr) => ({
           $context.stream[$context.stream_index]      == $pattern[0]
        && $context.stream[$context.stream_index + 1]  == $pattern[1]
        && $context.stream[$context.stream_index + 2]  == $pattern[2]
        && $context.stream[$context.stream_index + 3]  == $pattern[3]
        && $context.stream[$context.stream_index + 4]  == $pattern[4]
        && $context.stream[$context.stream_index + 5]  == $pattern[5]
        && $context.stream[$context.stream_index + 6]  == $pattern[6]
        && $context.stream[$context.stream_index + 7]  == $pattern[7]
        && $context.stream[$context.stream_index + 8]  == $pattern[8]
        && $context.stream[$context.stream_index + 9]  == $pattern[9]
        && $context.stream[$context.stream_index + 10] == $pattern[10]
        && $context.stream[$context.stream_index + 11] == $pattern[11]
        && $context.stream[$context.stream_index + 12] == $pattern[12]
        && $context.stream[$context.stream_index + 13] == $pattern[13]
        && $context.stream[$context.stream_index + 14] == $pattern[14]
        && $context.stream[$context.stream_index + 15] == $pattern[15]
        && $context.stream[$context.stream_index + 16] == $pattern[16]
        && $context.stream[$context.stream_index + 17] == $pattern[17]
        && $context.stream[$context.stream_index + 18] == $pattern[18]
        && $context.stream[$context.stream_index + 19] == $pattern[19]
        && $context.stream[$context.stream_index + 20] == $pattern[20]
        && $context.stream[$context.stream_index + 21] == $pattern[21]
        && $context.stream[$context.stream_index + 22] == $pattern[22]
    });
}

/// Determine if the remaining stream starts with `$pattern`, comparing only the first 24 bytes.
///
/// This macro assumes that `$pattern.len()` bytes are available for reading.
#[macro_export]
macro_rules! bs_starts_with24 {
    ($context:expr, $pattern:expr) => ({
           $context.stream[$context.stream_index]      == $pattern[0]
        && $context.stream[$context.stream_index + 1]  == $pattern[1]
        && $context.stream[$context.stream_index + 2]  == $pattern[2]
        && $context.stream[$context.stream_index + 3]  == $pattern[3]
        && $context.stream[$context.stream_index + 4]  == $pattern[4]
        && $context.stream[$context.stream_index + 5]  == $pattern[5]
        && $context.stream[$context.stream_index + 6]  == $pattern[6]
        && $context.stream[$context.stream_index + 7]  == $pattern[7]
        && $context.stream[$context.stream_index + 8]  == $pattern[8]
        && $context.stream[$context.stream_index + 9]  == $pattern[9]
        && $context.stream[$context.stream_index + 10] == $pattern[10]
        && $context.stream[$context.stream_index + 11] == $pattern[11]
        && $context.stream[$context.stream_index + 12] == $pattern[12]
        && $context.stream[$context.stream_index + 13] == $pattern[13]
        && $context.stream[$context.stream_index + 14] == $pattern[14]
        && $context.stream[$context.stream_index + 15] == $pattern[15]
        && $context.stream[$context.stream_index + 16] == $pattern[16]
        && $context.stream[$context.stream_index + 17] == $pattern[17]
        && $context.stream[$context.stream_index + 18] == $pattern[18]
        && $context.stream[$context.stream_index + 19] == $pattern[19]
        && $context.stream[$context.stream_index + 20] == $pattern[20]
        && $context.stream[$context.stream_index + 21] == $pattern[21]
        && $context.stream[$context.stream_index + 22] == $pattern[22]
        && $context.stream[$context.stream_index + 23] == $pattern[23]
    });
}

/// Determine if the remaining stream starts with `$pattern`, comparing only the first 25 bytes.
///
/// This macro assumes that `$pattern.len()` bytes are available for reading.
#[macro_export]
macro_rules! bs_starts_with25 {
    ($context:expr, $pattern:expr) => ({
           $context.stream[$context.stream_index]      == $pattern[0]
        && $context.stream[$context.stream_index + 1]  == $pattern[1]
        && $context.stream[$context.stream_index + 2]  == $pattern[2]
        && $context.stream[$context.stream_index + 3]  == $pattern[3]
        && $context.stream[$context.stream_index + 4]  == $pattern[4]
        && $context.stream[$context.stream_index + 5]  == $pattern[5]
        && $context.stream[$context.stream_index + 6]  == $pattern[6]
        && $context.stream[$context.stream_index + 7]  == $pattern[7]
        && $context.stream[$context.stream_index + 8]  == $pattern[8]
        && $context.stream[$context.stream_index + 9]  == $pattern[9]
        && $context.stream[$context.stream_index + 10] == $pattern[10]
        && $context.stream[$context.stream_index + 11] == $pattern[11]
        && $context.stream[$context.stream_index + 12] == $pattern[12]
        && $context.stream[$context.stream_index + 13] == $pattern[13]
        && $context.stream[$context.stream_index + 14] == $pattern[14]
        && $context.stream[$context.stream_index + 15] == $pattern[15]
        && $context.stream[$context.stream_index + 16] == $pattern[16]
        && $context.stream[$context.stream_index + 17] == $pattern[17]
        && $context.stream[$context.stream_index + 18] == $pattern[18]
        && $context.stream[$context.stream_index + 19] == $pattern[19]
        && $context.stream[$context.stream_index + 20] == $pattern[20]
        && $context.stream[$context.stream_index + 21] == $pattern[21]
        && $context.stream[$context.stream_index + 22] == $pattern[22]
        && $context.stream[$context.stream_index + 23] == $pattern[23]
        && $context.stream[$context.stream_index + 24] == $pattern[24]
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
