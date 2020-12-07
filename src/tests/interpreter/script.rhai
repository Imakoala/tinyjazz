fn to_bits(x) {
    let a = [];
    while x > 0 {
        a.push(x%2 == 1);
        x = x/2
    }
    while a.len < 16 {
        a.push(false);
    }
    while a.len > 16 {
        a.pop();
    }
    return [a]
}

if is_def_var("x") {
    x = x + 1;
    return to_bits(x)
}
let x = 1;
return to_bits(x)
