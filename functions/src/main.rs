fn main() {
    let scores_array: [i8; 5] = [85, 90, 78, 92, 88];
    for individual_score in scores_array {
        println!("{}",get_letter_grade(individual_score));
    }
}


fn calculate_sum(a:i32, b:i32) -> i32 {
    let sum = a + b;        // statements: line has to be terminated by ';'
    sum                     // expression: no termination of the line as this will return the value
}

fn get_letter_grade(students_score:i8) -> char {
    let students_grade:char;
    if students_score >= 90 {
        students_grade = 'A';
    } else if ((students_score < 90) && (students_score > 79)) {
        students_grade = 'B';
    } else if ((students_score < 80) && (students_score > 69)) {
        students_grade = 'c';
    } else if ((students_score < 70) && (students_score > 59)) {
        students_grade = 'D';
    } else {
        students_grade = 'F';
    }
    students_grade
}