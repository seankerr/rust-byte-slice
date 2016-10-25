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

use ByteStream;

#[test]
fn collect_eos() {
    let mut c = ByteStream::new(b"stream data");

    bs_collect!(c, {
        }, {
            assert_eq!(bs_slice!(c), b"stream data");
            break;
        }
    );
}

#[test]
fn collect_one() {
    let mut c = ByteStream::new(b"stream data");

    bs_collect!(c, {
            break;
        }, {
            panic!();
        }
    );

    assert_eq!(c.byte, b's');
    assert_eq!(bs_slice!(c), b"s");
}

#[test]
fn collect_word() {
    let mut c = ByteStream::new(b"stream data");

    bs_collect!(c, {
            if c.byte == b' ' {
                break;
            }
        }, {
            panic!();
        }
    );

    assert_eq!(c.byte, b' ');
    assert_eq!(bs_slice!(c), b"stream ");
    assert_eq!(bs_slice_ignore!(c), b"stream");
}

#[test]
fn collect_digits8_eos() {
    let mut c = ByteStream::new(b"42");
    let mut v = 0;

    bs_collect_digits8!(c, v, break, {
            assert_eq!(v, 42);
            break;
        }
    );
}

#[test]
fn collect_digits8_max() {
    let mut c = ByteStream::new(b"42");
    let mut v = 0;

    bs_collect_digits8!(c, v, break,
        if v > 41 {
            break;
        }, {
            panic!();
        }
    );
}

#[test]
fn collect_digits8_stop() {
    let mut c = ByteStream::new(b"42stop");
    let mut v = 0;

    bs_collect_digits8!(c, v, break, {
            panic!();
        }
    );

    assert_eq!(v, 42);
    assert_eq!(c.byte, b's');
}

#[test]
fn collect_digits8_test() {
    for n in 0..255 {
        if is_digit!(n) && n != b'0' {
            let a     = &[n];
            let mut c = ByteStream::new(a);
            let mut v = 0;

            bs_collect_digits8!(c, v, break, {
                    assert!(v > 0);
                    break;
                }
            );
        }
    }
}

#[test]
fn collect_digits16_eos() {
    let mut c = ByteStream::new(b"42");
    let mut v = 0;

    bs_collect_digits16!(c, v, break, {
            assert_eq!(v, 42);
            break;
        }
    );
}

#[test]
fn collect_digits16_max() {
    let mut c = ByteStream::new(b"42");
    let mut v = 0;

    bs_collect_digits16!(c, v, break,
        if v > 41 {
            break;
        }, {
            panic!();
        }
    );
}

#[test]
fn collect_digits16_stop() {
    let mut c = ByteStream::new(b"42stop");
    let mut v = 0;

    bs_collect_digits16!(c, v, break, {
            panic!();
        }
    );

    assert_eq!(v, 42);
    assert_eq!(c.byte, b's');
}

#[test]
fn collect_digits16_test() {
    for n in 0..255 {
        if is_digit!(n) && n != b'0' {
            let a     = &[n];
            let mut c = ByteStream::new(a);
            let mut v = 0;

            bs_collect_digits16!(c, v, break, {
                    assert!(v > 0);
                    break;
                }
            );
        }
    }
}

#[test]
fn collect_digits32_eos() {
    let mut c = ByteStream::new(b"42");
    let mut v = 0;

    bs_collect_digits32!(c, v, break, {
            assert_eq!(v, 42);
            break;
        }
    );
}

#[test]
fn collect_digits32_max() {
    let mut c = ByteStream::new(b"42");
    let mut v = 0;

    bs_collect_digits32!(c, v, break,
        if v > 41 {
            break;
        }, {
            panic!();
        }
    );
}

#[test]
fn collect_digits32_stop() {
    let mut c = ByteStream::new(b"42stop");
    let mut v = 0;

    bs_collect_digits32!(c, v, break, {
            panic!();
        }
    );

    assert_eq!(v, 42);
    assert_eq!(c.byte, b's');
}

#[test]
fn collect_digits32_test() {
    for n in 0..255 {
        if is_digit!(n) && n != b'0' {
            let a     = &[n];
            let mut c = ByteStream::new(a);
            let mut v = 0;

            bs_collect_digits32!(c, v, break, {
                    assert!(v > 0);
                    break;
                }
            );
        }
    }
}

#[test]
fn collect_digits64_eos() {
    let mut c = ByteStream::new(b"42");
    let mut v = 0;

    bs_collect_digits64!(c, v, break, {
            assert_eq!(v, 42);
            break;
        }
    );
}

#[test]
fn collect_digits64_max() {
    let mut c = ByteStream::new(b"42");
    let mut v = 0;

    bs_collect_digits64!(c, v, break,
        if v > 41 {
            break;
        }, {
            panic!();
        }
    );
}

#[test]
fn collect_digits64_stop() {
    let mut c = ByteStream::new(b"42stop");
    let mut v = 0;

    bs_collect_digits64!(c, v, break, {
            panic!();
        }
    );

    assert_eq!(v, 42);
    assert_eq!(c.byte, b's');
}

#[test]
fn collect_digits64_test() {
    for n in 0..255 {
        if is_digit!(n) && n != b'0' {
            let a     = &[n];
            let mut c = ByteStream::new(a);
            let mut v = 0;

            bs_collect_digits64!(c, v, break, {
                    assert!(v > 0);
                    break;
                }
            );
        }
    }
}

#[test]
fn collect_hex() {
    let mut c = ByteStream::new(b"AbC1f3");
    let mut v = 0;

    bs_collect_hex!(c, v, {
            break;
        }
    );

    assert_eq!(v, 0xABC1F3);
}

#[test]
fn collect_hex_break() {
    let mut c = ByteStream::new(b"AbC1f3Q");
    let mut v = 0;

    bs_collect_hex!(c, v, {
            break;
        }
    );

    assert_eq!(v, 0xABC1F3);
    assert_eq!(c.byte, b'Q');
}

#[test]
fn collect_hex_max() {
    let mut c = ByteStream::new(b"AbC1f3Q");
    let mut v = 0;

    bs_collect_hex!(c, v,
        if v > 0xABC {
            break;
        }, {
            panic!();
        }
    );

    assert_eq!(v, 0xABC1);
    assert_eq!(c.byte, b'1');
}

#[test]
fn collect_hex_test() {
    for n in 0..255 {
        if is_hex!(n) && n != b'0' {
            let a     = &[n];
            let mut c = ByteStream::new(a);
            let mut v = 0;

            bs_collect_hex!(c, v, {
                    assert!(v > 0);
                    break;
                }
            );
        }
    }
}

#[test]
fn collect_length() {
    let mut c = ByteStream::new(b"stream data");

    bs_collect_length!(c, 11);

    assert_eq!(bs_slice!(c), b"stream data");
}

#[test]
fn collect_length_check() {
    let mut c = ByteStream::new(b"stream data");

    bs_collect_length!(c, 11,
        if c.byte == b' ' {
            break;
        }, {
            panic!();
        }
    );

    assert_eq!(bs_slice!(c), b"stream ");
    assert_eq!(bs_slice_ignore!(c), b"stream");
}

#[test]
fn collect_when() {
    let mut c = ByteStream::new(b"stream data");

    bs_collect_when!(c,
        is_alpha!(c.byte),
        {
            break;
        }
    );

    assert_eq!(c.byte, b' ');
    assert_eq!(bs_slice!(c), b"stream ");
    assert_eq!(bs_slice_ignore!(c), b"stream");
}

#[test]
fn collect_when_eos() {
    let mut c = ByteStream::new(b"streamdata");

    bs_collect_when!(c,
        is_alpha!(c.byte),
        {
            break;
        }
    );

    assert_eq!(c.byte, b'a');
    assert_eq!(bs_slice!(c), b"streamdata");
}

#[test]
fn collect_until() {
    let mut c = ByteStream::new(b"stream data");

    bs_collect_until!(c,
        !is_alpha!(c.byte),
        {
            break;
        }
    );

    assert_eq!(c.byte, b' ');
    assert_eq!(bs_slice!(c), b"stream ");
    assert_eq!(bs_slice_ignore!(c), b"stream");
}

#[test]
fn collect_until_eos() {
    let mut c = ByteStream::new(b"streamdata");

    bs_collect_until!(c,
        !is_alpha!(c.byte),
        {
            break;
        }
    );

    assert_eq!(c.byte, b'a');
    assert_eq!(bs_slice!(c), b"streamdata");
}

#[test]
fn count() {
    let c = ByteStream::new(b"fancy stream data");

    assert_eq!(bs_count!(c, b'a'), 4);
}

#[test]
fn count_when() {
    let mut c = ByteStream::new(b"fancy stream data");

    assert_eq!(bs_count_when!(c, c.byte == b' '), 2);
    assert_eq!(c.byte, b'f');
}

#[test]
fn find1() {
    let c = ByteStream::new(b"stream data");

    if let Some(6) = bs_find!(c, b' ') {
    } else {
        panic!();
    }
}

#[test]
fn find2() {
    let c = ByteStream::new(b"");

    if let None = bs_find!(c, b' ') {
    } else {
        panic!();
    }
}

#[test]
fn find3() {
    let c = ByteStream::new(b"stream data");

    if let Some(6) = bs_find!(c, 6, b' ') {
    } else {
        panic!();
    }
}

#[test]
fn find4() {
    let c = ByteStream::new(b"stream data");

    if let None = bs_find!(c, 7, b' ') {
    } else {
        panic!();
    }
}

#[test]
fn find_pattern1() {
    let c = ByteStream::new(b"stream data pattern search");

    if let Some(12) = bs_find_pattern!(c, b"pattern") {
    } else {
        panic!();
    }
}

#[test]
fn find_pattern2() {
    let c = ByteStream::new(b"");

    if let None = bs_find_pattern!(c, b"pattern") {
    } else {
        panic!();
    }
}

#[test]
fn find_pattern3() {
    let c = ByteStream::new(b"stream data pattern search");

    if let Some(12) = bs_find_pattern!(c, 12, b"pattern") {
    } else {
        panic!();
    }
}

#[test]
fn find_pattern4() {
    let c = ByteStream::new(b"stream data pattern search");

    if let None = bs_find_pattern!(c, 13, b"pattern") {
    } else {
        panic!();
    }
}

#[test]
fn has_bytes() {
    let c = ByteStream::new(b"stream data");

    assert!(bs_has_bytes!(c, 11));
    assert!(!bs_has_bytes!(c, 12));
}

#[test]
fn index() {
    let mut c = ByteStream::new(b"stream data");

    assert_eq!(bs_index!(c), 0);

    bs_next!(c);

    assert_eq!(bs_index!(c), 1);

    bs_next!(c);

    assert_eq!(bs_index!(c), 2);

    bs_next!(c);

    assert_eq!(bs_index!(c), 3);
}

#[test]
fn is_alpha() {
    for n in 0..255 {
        if (n >= b'A' && n <= b'Z')
        || (n >= b'a' && n <= b'z') {
            assert!(is_alpha!(n));
        } else {
            assert!(!is_alpha!(n));
        }
    }
}

#[test]
fn is_control() {
    for n in 0..255 {
        if n < 0x20 || n == 0x7F {
            assert!(is_control!(n));
        } else {
            assert!(!is_control!(n));
        }
    }
}

#[test]
fn is_digit() {
    for n in 0..255 {
        if n >= b'0' && n <= b'9' {
            assert!(is_digit!(n));
        } else {
            assert!(!is_digit!(n));
        }
    }
}

#[test]
fn bs_is_eos() {
    let mut c = ByteStream::new(b"abc");

    assert!(!bs_is_eos!(c));

    bs_next!(c);

    assert!(!bs_is_eos!(c));

    bs_next!(c);

    assert!(!bs_is_eos!(c));

    bs_next!(c);

    assert!(bs_is_eos!(c));
}

#[test]
fn is_hex() {
    for n in 0..255 {
        if (n >= b'0' && n <= b'9')
        || (n >= b'A' && n <= b'F')
        || (n >= b'a' && n <= b'f') {
            assert!(is_hex!(n));
        } else {
            assert!(!is_hex!(n));
        }
    }
}

#[test]
fn is_not_visible_7bit() {
    for n in 0..255 {
        if n < b'!' || n > b'~' {
            assert!(is_not_visible_7bit!(n));
        } else {
            assert!(!is_not_visible_7bit!(n));
        }
    }
}

#[test]
fn is_not_visible_8bit() {
    for n in 0..255 {
        if n < b'!' || n == 0x7F || n == 0xFF {
            assert!(is_not_visible_8bit!(n));
        } else {
            assert!(!is_not_visible_8bit!(n));
        }
    }
}

#[test]
fn is_visible_7bit() {
    for n in 0..255 {
        if n >= b'!' && n <= b'~' {
            assert!(is_visible_7bit!(n));
        } else {
            assert!(!is_visible_7bit!(n));
        }
    }
}

#[test]
fn is_visible_8bit() {
    for n in 0..255 {
        if (n >= b'!' && n <= b'~')
        || (n >= 0x80 && n < 0xFF) {
            assert!(is_visible_8bit!(n));
        } else {
            assert!(!is_visible_8bit!(n));
        }
    }
}

#[test]
fn jump() {
    let mut c = ByteStream::new(b"stream");

    bs_jump!(c, 1);
    bs_next!(c);

    assert_eq!(c.byte, b't');

    bs_jump!(c, 1);
    bs_next!(c);

    assert_eq!(c.byte, b'e');

    bs_jump!(c, 1);
    bs_next!(c);

    assert_eq!(c.byte, b'm');
}

#[test]
fn mark() {
    let mut c = ByteStream::new(b"stream");

    bs_mark!(c);

    assert_eq!(c.mark_index, 0);

    bs_next!(c);

    assert_eq!(bs_slice!(c), b"s");

    bs_next!(c);
    bs_next!(c);
    bs_next!(c);

    assert_eq!(bs_slice!(c), b"stre");
}

#[test]
fn peek() {
    let mut c = ByteStream::new(b"stream");

    bs_mark!(c);

    assert_eq!(bs_peek!(c, 6), b"stream");

    bs_next!(c);

    assert_eq!(bs_peek!(c, 5), b"tream");

    bs_next!(c);

    assert_eq!(bs_peek!(c, 4), b"ream");

    bs_next!(c);

    assert_eq!(bs_peek!(c, 3), b"eam");

    bs_next!(c);

    assert_eq!(bs_peek!(c, 2), b"am");

    bs_next!(c);

    assert_eq!(bs_peek!(c, 1), b"m");
}

#[test]
fn remaining() {
    let mut c = ByteStream::new(b"stream");

    assert_eq!(bs_remaining!(c), b"stream");

    bs_next!(c);

    assert_eq!(bs_remaining!(c), b"tream");

    bs_next!(c);

    assert_eq!(bs_remaining!(c), b"ream");

    bs_next!(c);

    assert_eq!(bs_remaining!(c), b"eam");

    bs_next!(c);

    assert_eq!(bs_remaining!(c), b"am");

    bs_next!(c);

    assert_eq!(bs_remaining!(c), b"m");

    bs_next!(c);

    assert_eq!(bs_remaining!(c), b"");
}

#[test]
fn replay() {
    let mut c = ByteStream::new(b"stream");

    bs_mark!(c);
    bs_next!(c);
    bs_next!(c);
    bs_next!(c);

    assert_eq!(bs_slice!(c), b"str");

    bs_replay!(c);
    bs_replay!(c);

    assert_eq!(bs_slice!(c), b"s");
}

#[test]
fn rewind() {
    let mut c = ByteStream::new(b"stream");

    bs_mark!(c);
    bs_next!(c);
    bs_next!(c);
    bs_next!(c);
    bs_next!(c);
    bs_next!(c);
    bs_next!(c);

    assert_eq!(bs_slice!(c), b"stream");

    bs_rewind!(c, 1);

    assert_eq!(bs_slice!(c), b"strea");

    bs_rewind!(c, 1);

    assert_eq!(bs_slice!(c), b"stre");

    bs_rewind!(c, 1);

    assert_eq!(bs_slice!(c), b"str");

    bs_rewind!(c, 1);

    assert_eq!(bs_slice!(c), b"st");

    bs_rewind!(c, 1);

    assert_eq!(bs_slice!(c), b"s");

    bs_rewind!(c, 1);

    assert_eq!(bs_slice!(c), b"");
}

#[test]
fn rewind_to() {
    let mut c = ByteStream::new(b"stream");

    bs_mark!(c);
    bs_next!(c);
    bs_next!(c);
    bs_next!(c);
    bs_next!(c);
    bs_next!(c);
    bs_next!(c);

    assert_eq!(bs_slice!(c), b"stream");

    bs_rewind_to!(c, 3);

    assert_eq!(bs_slice!(c), b"str");

    bs_rewind_to!(c, 0);

    assert_eq!(bs_slice!(c), b"");
}

#[test]
fn slice_length() {
    let mut c = ByteStream::new(b"stream");

    bs_mark!(c);

    assert_eq!(0, bs_slice_length!(c));
    assert_eq!(bs_slice!(c), b"");

    bs_next!(c);

    assert_eq!(1, bs_slice_length!(c));
    assert_eq!(bs_slice!(c), b"s");

    bs_next!(c);

    assert_eq!(2, bs_slice_length!(c));
    assert_eq!(bs_slice!(c), b"st");

    bs_next!(c);

    assert_eq!(3, bs_slice_length!(c));
    assert_eq!(bs_slice!(c), b"str");

    bs_next!(c);

    assert_eq!(4, bs_slice_length!(c));
    assert_eq!(bs_slice!(c), b"stre");

    bs_next!(c);

    assert_eq!(5, bs_slice_length!(c));
    assert_eq!(bs_slice!(c), b"strea");

    bs_next!(c);

    assert_eq!(6, bs_slice_length!(c));
    assert_eq!(bs_slice!(c), b"stream");
}

#[test]
fn starts_with() {
    let mut c = ByteStream::new(b"stream data");

    assert!(bs_starts_with!(c, b"stream"));

    c = ByteStream::new(b"stream data");

    assert!(!bs_starts_with!(c, b"data"));

    c = ByteStream::new(b"stream data");

    assert!(bs_starts_with!(c, b"stream data"));
}

#[test]
fn starts_with1() {
    let c = ByteStream::new(b"s");

    assert!(bs_starts_with!(c, b"s"));
}

#[test]
fn starts_with2() {
    let c = ByteStream::new(b"st");

    assert!(bs_starts_with!(c, b"st"));
}

#[test]
fn starts_with3() {
    let c = ByteStream::new(b"str");

    assert!(bs_starts_with!(c, b"str"));
}

#[test]
fn starts_with4() {
    let c = ByteStream::new(b"stre");

    assert!(bs_starts_with!(c, b"stre"));
}

#[test]
fn starts_with5() {
    let c = ByteStream::new(b"strea");

    assert!(bs_starts_with!(c, b"strea"));
}

#[test]
fn starts_with6() {
    let c = ByteStream::new(b"stream");

    assert!(bs_starts_with!(c, b"stream"));
}

#[test]
fn starts_with7() {
    let c = ByteStream::new(b"stream ");

    assert!(bs_starts_with!(c, b"stream "));
}

#[test]
fn starts_with8() {
    let c = ByteStream::new(b"stream d");

    assert!(bs_starts_with!(c, b"stream d"));
}

#[test]
fn starts_with9() {
    let c = ByteStream::new(b"stream da");

    assert!(bs_starts_with!(c, b"stream da"));
}

#[test]
fn starts_with10() {
    let c = ByteStream::new(b"stream dat");

    assert!(bs_starts_with!(c, b"stream dat"));
}

#[test]
fn starts_with11() {
    let c = ByteStream::new(b"stream data");

    assert!(bs_starts_with!(c, b"stream data"));
}

#[test]
fn starts_with12() {
    let c = ByteStream::new(b"stream data ");

    assert!(bs_starts_with!(c, b"stream data "));
}

#[test]
fn starts_with13() {
    let c = ByteStream::new(b"stream data s");

    assert!(bs_starts_with!(c, b"stream data s"));
}

#[test]
fn starts_with14() {
    let c = ByteStream::new(b"stream data st");

    assert!(bs_starts_with!(c, b"stream data st"));
}

#[test]
fn starts_with15() {
    let c = ByteStream::new(b"stream data str");

    assert!(bs_starts_with!(c, b"stream data str"));
}

#[test]
fn starts_with16() {
    let c = ByteStream::new(b"stream data stre");

    assert!(bs_starts_with!(c, b"stream data stre"));
}

#[test]
fn starts_with17() {
    let c = ByteStream::new(b"stream data strea");

    assert!(bs_starts_with!(c, b"stream data strea"));
}

#[test]
fn starts_with18() {
    let c = ByteStream::new(b"stream data stream");

    assert!(bs_starts_with!(c, b"stream data stream"));
}

#[test]
fn starts_with19() {
    let c = ByteStream::new(b"stream data stream ");

    assert!(bs_starts_with!(c, b"stream data stream "));
}

#[test]
fn starts_with20() {
    let c = ByteStream::new(b"stream data stream d");

    assert!(bs_starts_with!(c, b"stream data stream d"));
}

#[test]
fn starts_with21() {
    let c = ByteStream::new(b"stream data stream da");

    assert!(bs_starts_with!(c, b"stream data stream da"));
}

#[test]
fn starts_with22() {
    let c = ByteStream::new(b"stream data stream dat");

    assert!(bs_starts_with!(c, b"stream data stream dat"));
}

#[test]
fn starts_with23() {
    let c = ByteStream::new(b"stream data stream data");

    assert!(bs_starts_with!(c, b"stream data stream data"));
}

#[test]
fn starts_with24() {
    let c = ByteStream::new(b"stream data stream data ");

    assert!(bs_starts_with!(c, b"stream data stream data "));
}

#[test]
fn starts_with25() {
    let c = ByteStream::new(b"stream data stream data s");

    assert!(bs_starts_with!(c, b"stream data stream data s"));
}
