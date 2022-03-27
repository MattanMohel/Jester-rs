// assert!( match_format!("1-2__3-4", "{}-{}__{}-{}", |c| { c.parse::<usize>::().is_ok() }) )

// macro_rules! match_format {
//     ( $($arg:tt)*) => ($crate::std::io::_print($crate::format_args!($($arg)*)));
// }


// const DELIMINATOR: &str = "{}";

// fn test(str: &String, pat: &String) -> bool {
//     let mut s = 0;
//     let mut p = 0;
//     while s < str.len() && p < pat.len() {
//         if pat[p..p + 2] == *DELIMINATOR {
//             for (i, c) in str[s..].chars().enumerate() {
//                 if c.to_string().as_str() == &pat[p + 2..p + 3] {
                    
//                 }
//             }
//         } 
//     }

//     match_format!("{}");
 
//     false
// }
