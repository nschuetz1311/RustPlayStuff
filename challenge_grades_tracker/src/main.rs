use std::collections::HashMap;

fn main() {

    let mut students_hash_map = HashMap::new();

    students_hash_map.insert(String::from("student1"), 90);
    students_hash_map.insert(String::from("student2"), 60);
    students_hash_map.insert(String::from("student3"), 55);
    students_hash_map.insert(String::from("student4"), 78);

    find_student_grade("student3", &students_hash_map);

    update_student_grade("student3", &mut students_hash_map, 70);

    remove_student("student2", &mut students_hash_map);

    display_all_grades(&students_hash_map);
}


fn find_student_grade(student: &str, grades: &HashMap<String, i32>) {
    match grades.get(student) {
        Some(grade) => println!("{} has a grade: {}", student, grade),
        None => println!("{} was not found", student),
    }
}


fn update_student_grade(student: &str, grades: &mut HashMap<String, i32>, target_grade: i32) {
    grades.insert(String::from(student), target_grade);
}

fn remove_student(student: &str, grades: &mut HashMap<String, i32>) {
    grades.remove(student);
}

fn display_all_grades(grades: &HashMap<String, i32>) {
    println!("{:?}", grades);
}
