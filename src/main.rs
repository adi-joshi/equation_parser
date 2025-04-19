use std::io::{stdin, stdout, Write};

#[derive(Debug)]
enum EntityType {
    Add,
    Subtract,
    Multiply,
    Number(i32),
}

#[derive(Debug)]
struct LinkedList {
    entity: EntityType,
    next: Option<Box<LinkedList>>,
}

fn evaluate_ll(mut ll: Option<Box<LinkedList>>) -> i32 {
    let mut result: i32 = 0;
    loop {
        println!("{}: {:?}", result, ll);
        if let Some(op) = &ll {
            result = match op.entity {
                EntityType::Number(num) => {
                    ll = ll.unwrap().next;
                    result += num;
                    result
                },
                EntityType::Add => {
                    if let Some(node) = &ll.as_ref().unwrap().next {
                        if let EntityType::Number(num) = node.entity {
                            result += num;
                            ll = ll.unwrap().next.unwrap().next;
                            result
                        } else {
                            panic!();
                        }
                    } else {
                        panic!();
                    }
                },
                EntityType::Subtract => {
                    if let Some(node) = &ll.as_ref().unwrap().next {
                        if let EntityType::Number(num) = node.entity {
                            result -= num;
                            ll = ll.unwrap().next.unwrap().next;
                            result
                        } else {
                            panic!();
                        }
                    } else {
                        panic!();
                    }
                }
                EntityType::Multiply => {
                    if let Some(node) = &ll.as_ref().unwrap().next {
                        if let EntityType::Number(num) = node.entity {
                            result *= num;
                            ll = ll.unwrap().next.unwrap().next;
                            result
                        } else {
                            panic!();
                        }
                    } else {
                        panic!();
                    }
                }
            }
        } else {
            break;
        }
    }
    result
}


fn parse_num(num_string: String) -> i32 {
    // num.into(int)
    // println!("In parse num: {}", _num_iter.as_str());
    // _num_iter.next();
    num_string.parse().unwrap()
}

fn parse_eq(eq: String) -> i32 {
    let first_char = eq.chars().next().expect("empty equation");
    if !(first_char.is_numeric()) && !(first_char == '-') {
        panic!();
    }

    let mut root : Option<Box<LinkedList>> =
        Some(Box::new(LinkedList {
            entity: EntityType::Number(0),
            next: Some(Box::new(LinkedList {
                entity: EntityType::Add,
                next: None })) })); 

    let mut ll : &mut Option<Box<LinkedList>> = &mut root.as_mut().unwrap().next;

    let mut eq_chars = eq.chars().peekable();

    loop {
        let c = eq_chars.peek();
        if let Some(x) = c {
            if (*x >= '0' && *x <= '9') || (*x == '-') {
                let mut num_str: String = (*x).to_string();
                eq_chars.next();
                loop {
                    if let Some(n) = eq_chars.peek() {
                        println!("{}", n);
                        if *n >= '0' && *n <= '9' { // cannot have - in the middle of a number
                            num_str += &eq_chars.next().unwrap().to_string();
                            println!("Num str: {}", num_str);
                        } else {
                            break;
                        }
                    } else {
                        break;
                    }
                }
                let num: i32 = parse_num(num_str);
                let node: Option<Box<LinkedList>> = Some(Box::new(LinkedList {
                    entity: EntityType::Number(num),
                    next: None,
                }));
                ll.as_mut().unwrap().next = node;
                ll = &mut ll.as_mut().unwrap().next;
            } else if *x == '+' {
                let node: Option<Box<LinkedList>> = Some(Box::new(LinkedList {
                    entity: EntityType::Add,
                    next: None,
                }));
                ll.as_mut().unwrap().next = node;
                ll = &mut ll.as_mut().unwrap().next;
                eq_chars.next();
            } else if *x == '-' {
                let node: Option<Box<LinkedList>> = Some(Box::new(LinkedList {
                    entity: EntityType::Subtract,
                    next: None,
                }));
                ll.as_mut().unwrap().next = node;
                ll = &mut ll.as_mut().unwrap().next;
                eq_chars.next();
            } else if *x == '*' {
                let node: Option<Box<LinkedList>> = Some(Box::new(LinkedList {
                    entity: EntityType::Multiply,
                    next: None,
                }));
                ll.as_mut().unwrap().next = node;
                ll = &mut ll.as_mut().unwrap().next;
                eq_chars.next();
            }
        } else {
            break;
        }
    }

    println!("Inside loop");

    evaluate_ll(root)
}

fn main() {
    println!("Hi! Enter an equation: ");
    let _ = stdout().flush();
    let mut eq = String::new();
    stdin().read_line(&mut eq).expect("Couldn't read the equation");

    let result: i32 = parse_eq(eq.chars().filter(|c| !c.is_whitespace()).collect());

    println!("The result is {}", result);
}
