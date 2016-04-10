fn main() {
    println!("-- Euler Project Answers --");
    exercise1();
}

fn format_answer(exercise: i32, answer: i32) {
    println!("Excerise {}: {}", exercise, answer);
}

fn exercise1() {
    let mut sum = 0;
    for number in 0..1000 {
        if number % 3 == 0 || number % 5 == 0 {
            sum += number
        }
    }
    format_answer(1, sum);
}

