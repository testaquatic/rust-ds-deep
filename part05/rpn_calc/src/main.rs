fn rpn_calc(expr: &str) -> Result<f64, String> {
    let mut stack = Vec::new();

    for token in expr.split_whitespace() {
        match token {
            "+" | "-" | "*" | "/" => {
                let b = stack.pop().ok_or("피연산자 부족")?;
                let a = stack.pop().ok_or("피연산자 부족")?;

                let result = match token {
                    "+" => a + b,
                    "-" => a - b,
                    "*" => a * b,
                    "/" => {
                        if b == 0.0 {
                            return Err("0으로 나눌 수 없습니다.".to_string());
                        } else {
                            a / b
                        }
                    }
                    _ => unreachable!(),
                };
                stack.push(result);
            }
            num => {
                let n = num
                    .parse::<f64>()
                    .map_err(|_| format!("잘못된 토큰: {}", num))?;
                stack.push(n);
            }
        }
    }
    if stack.len() == 1 {
        Ok(stack.pop().unwrap())
    } else {
        Err(format!("수식 오류: 스택에 {}개 남음", stack.len()))
    }
}

fn main() {
    println!("{:?}", rpn_calc("3 4 + 2 *"));
    println!("{:?}", rpn_calc("5 1 2 + 4 * + 3 -"));
    println!("{:?}", rpn_calc("4 0 /"));
    println!("{:?}", rpn_calc("3 +"));
}
