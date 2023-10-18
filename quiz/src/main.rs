extern crate yaml_rust;

use std::io;
use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use clap::Parser;
use yaml_rust::yaml;


#[derive(Parser)]
struct Args {
    #[arg(short,long)]
    file: String,
}

#[derive(Debug, Clone)]
enum QuizError {
    NoMoreQuestionsError,
}

impl fmt::Display for QuizError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            QuizError::NoMoreQuestionsError => {
                write!(f, "No more questions!")
            },
        }
    }
}

struct QuizQuestion {
    point_value: f64,
    question_text: String,
    question_answer: String,
    question_id: u32,
    answer: Option<String>,
    mark: bool,
}

impl QuizQuestion {
    fn new(id: u32, points: f64, question: String, answer: String) -> Self {
        Self {
            point_value: points,
            question_text: question,
            question_answer: answer,
            question_id: id,
            answer: None,
            mark: false,
        }
    }

    fn is_correct(&self) -> bool {
        match &self.answer {
            Some(a) => &self.question_answer == a,
            None => false,
        }
    }

    fn is_complete(&self) -> bool {
        match &self.answer {
            Some(_) => true,
            None => false,
        }
    }

    fn is_marked(self) -> bool {
        self.mark
    }
}

struct Quiz {
    name: String,
    questions: Vec<QuizQuestion>,
    score: f64,
    count_correct: u32,
}

impl Quiz {
    fn new(doc: &yaml::Yaml) -> Self {
        let mut s = Self {
            name: String::from(doc["name"].as_str().unwrap()),
            questions: Vec::<QuizQuestion>::new(),
            score: 0.0,
            count_correct: 0,
        };

        for i in doc["questions"].as_vec().unwrap() {
            s._add_question(
                i["id"].as_i64().unwrap().try_into().unwrap(),
                i["points"].as_f64().unwrap().try_into().unwrap(),
                String::from(i["question"].as_str().unwrap()),
                String::from(i["answer"].as_str().unwrap()),
            );
        }

        s
    }

    fn _add_question(
        &mut self,
        id: u32,
        points: f64,
        question: String,
        answer: String,
    ) {
        self.questions.push(
            QuizQuestion::new(id, points, question, answer)
        );
    }

    /// Returns the index of the next unanswered question
    fn _next_question(&self) -> Result<usize, QuizError> {
        for i in 0..self.questions.len() {
            if !self.questions[i].is_complete() {
                return Ok(i)
            }
        }
        Result::Err(QuizError::NoMoreQuestionsError)
    }

    fn answer_question(&self, question: QuizQuestion) {
        
    }

    fn mark_question(&self, question: QuizQuestion) {

    }

    fn take_quiz(&mut self) {
        loop {
            match self._next_question() {
                Ok(q) => {
                    let mut cq = &mut self.questions[q];
                    println!("Question {}:\n\n\t{}", cq.question_id, cq.question_text);
                    let mut a = String::new();
                    io::stdin().read_line(&mut a);
                    a.pop();
                    println!("");
                    cq.answer = Some(a);
                },
                Err(QuizError::NoMoreQuestionsError) => {
                    println!("No more questions!");
                    break
                },
            }
        }

    }

    fn score_quiz(&self) {
        let mut score: f64 = 0.0;
        let mut total: f64 = 0.0;
        for q in &self.questions {
            total += q.point_value;
            if q.is_correct() {
                score += q.point_value;
            }
        }
        println!("On the \"{}\" quiz you scored {}% ({} points)", self.name, score/total*100.0, score);

    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let mut f = File::open(args.file)?;
    let mut s: String = String::new();

    f.read_to_string(&mut s)?;

    let data = yaml::YamlLoader::load_from_str(&s)?;
    let doc = &data[0];

    let mut quiz = Quiz::new(&doc["quiz"]);
    quiz.take_quiz();
    quiz.score_quiz();

    Ok(())
}
