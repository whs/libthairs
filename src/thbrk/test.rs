////////////////////////////////////////////////////////////////////////////////
// Copyright (C) 2022 Manatsawin Hanmongkolchai
//
// This library is free software; you can redistribute it and/or
// modify it under the terms of the GNU Lesser General Public
// License as published by the Free Software Foundation; either
// version 2.1 of the License, or (at your option) any later version.
//
// This library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public
// License along with this library; if not, write to the Free Software
// Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301  USA
////////////////////////////////////////////////////////////////////////////////

use crate::thbrk::TisBreaker;
use encoding_rs::WINDOWS_874;
use itertools::Itertools;

struct TestSample {
    txt: String,
    brk_pos: Vec<usize>,
    ins_str: String,
}

pub fn test_thbrk<T: TisBreaker>(breaker: &T) {
    // Port of libthai test_thbrk.c
    let test_samples = vec![
        TestSample {
            txt: "".to_string(),
            brk_pos: vec![],
            ins_str: "".to_string(),
        },
        TestSample {
            txt: "12% is 12 % and (12.00)% is (12.00) %".to_string(),
            brk_pos: vec![4, 7, 12, 16, 25, 28],
            ins_str: "12% |is |12 % |and |(12.00)% |is |(12.00) %".to_string(),
        },
        TestSample {
            txt: "ten% of all สิบ%ของทั้งหมด".to_string(),
            brk_pos: vec![5, 8, 12, 16, 19],
            ins_str: "ten% |of |all |สิบ%|ของ|ทั้งหมด".to_string(),
        },
        TestSample {
            txt: "$12 is $ 12 and $(12.00) is $ (12.00)".to_string(),
            brk_pos: vec![4, 7, 12, 16, 25, 28],
            ins_str: "$12 |is |$ 12 |and |$(12.00) |is |$ (12.00)".to_string(),
        },
        TestSample {
            txt: "Brazilian R$2 Nicaraguan C$10 U$S5 US$5 5$ 5 $ Micro$oft C++".to_string(),
            brk_pos: vec![10, 14, 25, 30, 35, 40, 47, 57],
            ins_str: "Brazilian |R$2 |Nicaraguan |C$10 |U$S5 |US$5 |5$ 5 $ |Micro$oft |C++"
                .to_string(),
        },
        TestSample {
            txt: "฿12 บ้างเขียน 12฿ หรือ ฿ 12 หรือ 12 ฿".to_string(),
            brk_pos: vec![4, 8, 14, 18, 23, 28, 33],
            ins_str: "฿12 |บ้าง|เขียน |12฿ |หรือ |฿ 12 |หรือ |12 ฿".to_string(),
        },
        TestSample {
            txt: "฿(12.00) บ้างก็เขียน ฿ (12.00) หรือ (12.00)฿ หรือ (12.00) ฿".to_string(),
            brk_pos: vec![9, 13, 15, 21, 31, 36, 45, 50],
            ins_str: "฿(12.00) |บ้าง|ก็|เขียน |฿ (12.00) |หรือ |(12.00)฿ |หรือ |(12.00) ฿".to_string(),
        },
        TestSample {
            txt: "#hashtag #แฮชแท็ก #1 5# helloสวัสดี".to_string(),
            brk_pos: vec![9, 18, 21, 24, 29],
            ins_str: "#hashtag |#แฮชแท็ก |#1 |5# |hello|สวัสดี".to_string(),
        },
        TestSample {
            txt: "e.g. Ph.D.Engineering M.A.(Linguistics) ม.ค.มกราคมศกนี้".to_string(),
            brk_pos: vec![5, 10, 22, 26, 40, 44, 50, 52],
            ins_str: "e.g. |Ph.D.|Engineering |M.A.|(Linguistics) |ม.ค.|มกราคม|ศก|นี้".to_string(),
        },
        TestSample {
            txt: "(s)he (may) meet person(s)".to_string(),
            brk_pos: vec![6, 12, 17],
            ins_str: "(s)he |(may) |meet |person(s)".to_string(),
        },
        TestSample {
            txt: "สวัสดีครับ กอ.รมน. นี่เป็นการทดสอบตัวเอง".to_string(),
            brk_pos: vec![6, 11, 19, 22, 26, 29, 34],
            ins_str: "สวัสดี|ครับ |กอ.รมน. |นี่|เป็น|การ|ทดสอบ|ตัวเอง".to_string(),
        },
        TestSample {
            txt: "พันธุ์ข้าว กข43 น้ำตาลต่ำ กิโลละ 40บาท กระดาษ A4 ใน 4.3BSD".to_string(),
            brk_pos: vec![6, 11, 16, 22, 26, 30, 33, 35, 39, 46, 49, 52],
            ins_str: "พันธุ์|ข้าว |กข43 |น้ำตาล|ต่ำ |กิโล|ละ |40|บาท |กระดาษ |A4 |ใน |4.3BSD".to_string(),
        },
    ];

    for (index, case) in test_samples.iter().enumerate() {
        // utf8_to_tis
        let (input_tis, _, _) = WINDOWS_874.encode(&case.txt);
        let (output_tis, _, _) = WINDOWS_874.encode(&case.ins_str);

        let break_string: Vec<u8> =
            Itertools::intersperse(breaker.split_tis(&input_tis).into_iter(), "|".as_bytes())
                .flatten()
                .cloned()
                .collect();
        let (break_string_utf, _, _) = WINDOWS_874.decode(&break_string);
        assert_eq!(
            break_string,
            output_tis.as_ref(),
            "Failed at case {}: brk \"{}\" != expected \"{}\"",
            index + 1,
            break_string_utf,
            case.ins_str,
        );

        let res = breaker.find_breaks_tis(&input_tis, input_tis.len());
        assert_eq!(
            res,
            case.brk_pos,
            "Failed at case {}: brk {:?} != expected {:?}",
            index + 1,
            res,
            case.brk_pos
        );
    }
}
