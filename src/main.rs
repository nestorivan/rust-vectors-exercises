// fn main() {
//     // empty vector
//     // explicit type anotation is needed
//     let mut v = Vec::new();
//     v.push(String::from("One"));
//     v.push(String::from("Two"));
//     v.push(String::from("Three"));

//     let v2 = vec![1, 2, 3];

//     let s = &v2[0];

//     //it removes the element and shifts all elements to the right
//     // let s2 = v.remove(2);

//     // it returns an Option with a reference to the element if its found
//     if let Some(e) = v.get(0) {
//         println!("{e}")
//     }

//     for s in &mut v {
//         s.push_str("!")
//     }

//     for s in &v {
//         println!("{s}")
//     }

//     let mut v3 = vec![];

//     // we can consume a vector if we dont pass its value
//     for s in v {
//         v3.push(s);
//     }

//     // trying to accessing a vector after it was consumed produce an error
//     // let i = v.get(0);
// }

//exercise 1
// fn append(nums: &mut Vec<i32>, num: i32) -> &mut Vec<i32> {
//     nums.push(num);
//     nums
// }

// fn main() {
//     let mut nums = vec![1, 2, 5, 6];
//     append(&mut nums, 8);
//     append(&mut nums, 3);
//     assert_eq!(nums.len(), 6);
// }

//exercise 2
// fn remove_if_odd(nums: &mut Vec<i32>, index: usize) -> &mut Vec<i32> {
//     println!("nums: {:?} \nindex: {}", nums, index);

//     if index > nums.len() {
//         println!("Index out of bounds");
//         return nums;
//     }

//     if nums[index] % 2 != 0 {
//         println!("value in index {index} is odd");
//         nums.remove(index);
//         return nums;
//     }

//     nums
// }

// fn main() {
//     let mut nums = vec![1, 2, 6, 9];
//     let nums_ref = &mut nums;
//     remove_if_odd(nums_ref, 0);
//     remove_if_odd(nums_ref, 1);
//     remove_if_odd(nums_ref, nums_ref.len() - 1);
//     assert_eq!(nums.len(), 2);
// }

//exercise 3
// fn main() {
//     let names = vec!["Alice", "Bob", "Cindy"];
//     let index = 2;
//     if let Some(name) = names.get(index) {
//         println!("{name} is present at index {index}");
//     } else {
//         println!("invalid index {index}");
//     }
// }

//exercise 4
// Fix the code so that it compiles.

// struct Student {
//     name: String,
//     marks: u8,
// }

// impl Student {
//     fn new(name: &str, marks: u8) -> Self {
//         Self {
//             name: name.to_string(),
//             marks,
//         }
//     }
// }

// fn main() {
//     let students = vec![
//         Student::new("Harry", 75),
//         Student::new("Hermoine", 99),
//         Student::new("Ron", 60),
//     ];
//     let mut grades = Vec::new();

//     for student in &students {
//         if student.marks > 80 {
//             grades.push('A');
//         } else if student.marks > 60 {
//             grades.push('B');
//         } else {
//             grades.push('C');
//         }
//     }
//     for i in 0..grades.len() {
//         println!("{} got {}!", students[i].name, grades[i]);
//     }
// }

// exercise 5
struct Student {
    name: String,
    marks: u8,
    grade: char,
}

impl Student {
    fn new(name: &str, marks: u8) -> Self {
        Self {
            name: name.to_string(),
            marks,
            grade: 'X',
        }
    }
}

fn main() {
    let mut students = vec![
        Student::new("Harry", 75),
        Student::new("Hermoine", 99),
        Student::new("Ron", 60),
    ];
    for student in &mut students {
        student.grade = if student.marks > 80 {
            'A'
        } else if student.marks > 60 {
            'B'
        } else {
            'C'
        };
    }
    for student in students {
        println!("{} got {}!", student.name, student.grade);
    }
}
