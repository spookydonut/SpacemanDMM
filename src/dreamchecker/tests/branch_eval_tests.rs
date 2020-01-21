
extern crate dreamchecker as dc;

use dc::test_helpers::*;

pub const CONST_EVAL_ERRORS: &[(u32, u16, &str)] = &[
    (2, 7, "control flow condition is a static term"),
    (2, 7, "if condition is always true"),
];

#[test]
fn const_eval() {
    let code = r##"
/proc/test()
    if(1)
        return
    return
"##.trim();
    check_errors_match(code, CONST_EVAL_ERRORS);
}

pub const IF_ELSE_ERRORS: &[(u32, u16, &str)] = &[
    (6, 5, "possible unreachable code here"),
];

#[test]
fn if_else() {
    let code = r##"
/proc/test()
    if(prob(50))
        return
    else
        return
    return
"##.trim();
    check_errors_match(code, IF_ELSE_ERRORS);
}

pub const IF_ARMS_ERRORS: &[(u32, u16, &str)] = &[
    (2, 7, "control flow condition is a static term"),
    (2, 7, "if condition is always true"),
    (4, 12, "unreachable if block, preceeding if/elseif condition(s) are always true"),
    // TODO: fix location reporting on this
    (2, 5, "unreachable else block, preceeding if/elseif condition(s) are always true"),
];

#[test]
fn if_arms() {
    let code = r##"
/proc/test()
    if(1)
        return
    else if(prob(50))
        return
    else
        return
"##.trim();
    check_errors_match(code, IF_ARMS_ERRORS);
}

pub const DO_WHILE_ERRORS: &[(u32, u16, &str)] = &[
    (2, 5, "do while terminates without ever reaching condition"),
];

#[test]
fn do_while() {
    let code = r##"
/proc/test()
    do
        return
    while(prob(50))
"##.trim();
    check_errors_match(code, DO_WHILE_ERRORS);
}
