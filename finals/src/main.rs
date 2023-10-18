use std::io;

struct Grade {
    homework_grade: f32,
    midterm_grade: f32,
    final_grade: f32,
}

impl Grade {
    fn new(hw: f32, mg: f32, fg: f32) -> Self {
        Self {
            homework_grade: hw,
            midterm_grade: mg,
            final_grade: fg,
        }
    }

    fn report(&self) -> f32 {
        return self.homework_grade * 0.4 + self.midterm_grade * 0.2 + self.final_grade * 0.4;
    }
}

fn read_grade(p: String) -> f32 {
    let mut s: String = String::new();
    println!("Enter the {} grade: ", p);
    io::stdin().read_line(&mut s);
    s.pop();
    s.parse::<f32>().unwrap()
}

fn main() {
    //Write a programme that allows you to input students' midterm,
    //final and homework scores, and calculate a weighted score.
    //Use the following weights: 20% midterm, 40% final, 40% median homework.
    let hg: f32 = read_grade(String::from("homework"));
    let mg: f32 = read_grade(String::from("midterm"));
    let fg: f32 = read_grade(String::from("final"));
    let grade: Grade = Grade::new(hg, mg, fg);
    println!("The grade is {}", grade.report());
}
