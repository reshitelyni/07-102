/* ====================================================
*   Copyright (C) 2023  All rights reserved.
*
*   Author        : 6607changchun
*   Email         : luobojiaozi@163.com
*   File Name     : main.rs
*   Last Modified : 2023-09-12 16:19
*   Describe      : Client executable.
*
* ====================================================*/
use arccal::ArcArgs;
use arccal::Actor;

fn main() {
    let args = ArcArgs::new();
    Actor::new(args).run();
}
