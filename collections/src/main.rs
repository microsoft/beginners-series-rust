fn main() {
    let mut students = vec![Student {
        name: String::from("Ryan"),
    }];

    students.push(Student {
        name: "Li".to_string(),
    });

    assert!(
        &students[0]
            == &Student {
                name: String::from("Ryan")
            }
    );
    assert!(
        students.get(0)
            == Some(&Student {
                name: String::from("Ryan")
            })
    );
    assert!(students.get(100) == None);

    for student in students.iter() {
        println!("Student name: {}", student.name);
    }

    use std::collections::HashMap;

    let mut enrollment = HashMap::new();
    enrollment.insert(String::from("biology"), students);

    let bio_students = enrollment.get("biology");
    let bio_students = enrollment.remove("biology");
}

#[derive(PartialEq, Eq)]
struct Student {
    name: String,
}
