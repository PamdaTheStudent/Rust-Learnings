use std::ops::Index;
use std::io;
use std::io::Write;


struct Student 
{
    name: String,
    scores: Vec<f64>,
}


fn main() 
{
    let mut students: Vec<Student> = Vec::new();

    println!("Hello, world!");
    let mut student = Student {
        name: String::from("Alice"),
        scores: vec![85.0, 90.0, 78.0],
    };
    add_score(&mut student, 92.0);
    println!("{}'s scores: {:?}", student.name, student.scores);
    remove_score(&mut student, 0);
    println!("{}'s scores: {:?}", student.name, student.scores);
    add_student(String::from("Fake"), vec![15.1, 61.3, 92.0], &mut students);
    display_menu(&mut students);
}


fn display_menu(students: &mut Vec<Student>)
{
    loop{
        println!("\n========== MENU ==========");
        println!("Menu options:");
        println!("Menu options:");
        println!("add student");
        println!("add score");
        println!("remove student");
        println!("remove score");
        println!("exit ");

        print_same_line(String::from("Menu Choice: "));
        let input = recieve_input();
        println!("{}",input);

        match input.as_str() {
            "remove student" => {
                println!("Current students:");
                for (i, student) in students.iter().enumerate() {
                    println!("{}: {}", i, student.name);
                }
                print_same_line(String::from("Enter student index to remove: "));
                let input: String = recieve_input();
                match input.trim().parse::<usize>() {
                    Ok(index) if index < students.len() => {
                        println!("Removed student: {}", students[index].name);
                        students.remove(index);
                    }
                    _ => println!("Invalid index."),
                }

            }
            "remove score" => {
                println!("Current students:");
                for (i, student) in students.iter().enumerate() {
                    println!("{}: {}", i, student.name);
                }
                print_same_line(String::from("Enter student index: "));
                let input: String = recieve_input();
                match input.trim().parse::<usize>() {
                    Ok(student_index) if student_index < students.len() => {
                        let student = &mut students[student_index];
                        println!("{}'s scores: {:?}", student.name, student.scores);
                        print_same_line(String::from("Enter score index to remove: "));
                        let input: String = recieve_input();
                        match input.trim().parse::<usize>() {
                            Ok(score_index) if score_index < student.scores.len() => {
                                student.scores.remove(score_index);
                                println!("Score removed! Scores are now: {:?}", student.scores);
                            }
                            _ => println!("Invalid score index."),
                        }
                    }
                    _ => println!("Invalid student index."),
                }
            }
            "add student" => {
                println!("adding Student:");
                print_same_line(String::from("Student Name: "));
                let name: String = recieve_input();

                let mut scores: Vec<f64> = Vec::new();
                loop {
                    print_same_line(String::from("Enter a score (or 'done' to finish): "));
                    let input: String = recieve_input();

                    if input == "done" {
                        break;
                    }

                    match input.trim().parse::<f64>() {
                        Ok(score) => scores.push(score),
                        Err(_) => println!("Invalid score, please enter a number."),
                    }
                }
                add_student(name, scores, students);
            }
            "add score" => {
                println!("Current students:");
                for (i, student) in students.iter().enumerate() {
                    println!("{}: {}", i, student.name);
                }
                print_same_line(String::from("Enter student index: "));
                let input: String = recieve_input();
                match input.trim().parse::<usize>() {
                    Ok(index) if index < students.len() => {
                        let student = &mut students[index];
                        print_same_line(String::from("Enter score to add: "));
                        let input: String = recieve_input();
                        match input.trim().parse::<f64>() {
                            Ok(score) => {
                                add_score(student, score);
                                println!("Score added! Scores are now: {:?}", student.scores);
                            }
                            Err(_) => println!("Invalid score."),
                        }
                    }
                    _ => println!("Invalid student index."),
                }
            }
            "exit" => break,
            _ => println!("unknown option"),

        }
    }

}

fn add_score(student: &mut Student, score: f64) 
{
    student.scores.push(score);
}

fn remove_score(student: &mut Student, index:usize)
{
    student.scores.remove(index);
}

fn add_student(name: String, scores: Vec<f64>, students: &mut Vec<Student>)
{
    students.push(Student{name: name, scores: scores});
}

fn recieve_input() -> String
{
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("failed to read line");
        return input.trim().to_string();
}

fn print_same_line(display_string: String)
{
    print!("{}", display_string);
    io::stdout().flush().expect("failed to flush");
}