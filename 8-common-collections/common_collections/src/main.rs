// the number of bytes it takes to encode “Здравствуйте” in UTF-8, because each Unicode
// scalar value in that string takes 2 bytes of storage. Therefore, an index into the string’s
// bytes will not always correlate to a valid Unicode scalar value.

// DO NOT USE RANGE to get String partially.
// it prevents you from having to handle errors involving non-ASCII characters later in your development life cycle

// By default, HashMap uses a hashing function called SipHash that can provide resistance to Denial of Service (DoS) attacks involving hash tables
// Not the fastest but safe.
use std::collections::HashMap;

fn pig_latin(s: &str) -> String {
    let vowels = ['a', 'e', 'i', 'o', 'u'];

    let mut pig_latin_words = Vec::new();

    for word in s.split_whitespace() {
        let first_char = word.chars().next().unwrap();
        let mut pig_latin_word = String::new();

        if vowels.contains(&first_char) {
            pig_latin_word.push_str(word);
            pig_latin_word.push_str("-hay");
        } else {
            // let rest = word.chars().skip(1).as_str(); can be used as alternative.
            let (_, rest) = word.split_at(1);
            pig_latin_word.push_str(rest);
            pig_latin_word.push('-');
            pig_latin_word.push(first_char);
            pig_latin_word.push_str("ay");
        }

        pig_latin_words.push(pig_latin_word);
    }

    pig_latin_words.join(" ")
}

fn main() {
    let v: Vec<i32> = Vec::new(); // Similar to String::new();
    let mut z = vec![1, 2, 3]; // vec! is easier way to create a vector
    z.push(5);
    z.push(6);
    let fourth: &i32 = &z[3];
    println!("The fourth element is {fourth}");

    let third_of_v: Option<&i32> = v.get(2);
    match third_of_v {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    let mut mut_vector: Vec<i32> = vec![1, 2, 3, 4, 5];
    {
        let first = &mut_vector[0];
        // cannot push here with "mut_vector.push(6);". Reason is simple it may need to reassign the memory and make variable first invalid
    }
    mut_vector.push(6);

    let mut mut_vector2: Vec<i32> = vec![1, 2, 3, 4, 5];
    let first2 = mut_vector2[0];
    // this works because we have not get the reference
    mut_vector2.push(6);

    for i in &mut mut_vector2 {
        // use * dereference operator to get to the value in i before we can use the += operator
        *i += 50;
    }
    println!(
        "The first element is: {}, vector is : {:?}",
        first2, mut_vector2
    );

    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    // We used Enum to be able to keep different types in a vector. (They are all enum hence no problem)

    // If you don’t know the exhaustive set of types a program will get at runtime to store in a vector,
    // the enum technique won’t work. Instead, you can use a trait object, which is described in chapter 17.
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    println!("vector is : {:?}", row);

    // strings are UTF-8 encoded, so we can include any properly encoded data in them
    let mut s = String::from("initial contents"); // or "initial contents".to_string();
    s.push_str(" bar");
    s.push('|'); // add only one character
    let s1 = "hello";

    // fn add(self, s: &str) -> String { => We can only add &str to a String parameter. Compiler can force &String to &str => &s2 -> &s2[..]
    let s = s + &s1;
    let s2 = format!("{s}-{s1}");
    println!("s is : {s} and s2 is {s2}");
    // for more complicated string comnining use format!. It is like println! but returns a String instead of printing. It does not take ownership of any string
    let hello = "Здравствуйте";
    let slice = &hello[0..4]; // Зд since each character in hello occupy 2 bytes
    println!("slice of hello is: {}", slice);
    // .chars characterize string and returns the most convenient string parts as an Chars array => Chars([...])
    let hello_char = hello.chars();
    println!("hello_char: {:?}", hello_char);
    for c in hello_char {
        println!("{c}");
    }

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25); // 10 is overwritten and now Blue: 25
                                             // Easy to implement and works better with borrow checker
    scores.entry(String::from("Blue")).or_insert(50); // not overwritten since already exists
    scores.insert(String::from("Yellow"), 50);
    scores.entry(String::from("Green")).or_insert(50); // created Green as 50 since it does not exist.

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("Blue Team: {score}");

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point. If try to use it will throw compiler error.

    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    // Count how many times each word is used in string
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        // dereference count to assign to that value
        *count += 1;
    }

    println!("{:?}", map);

    // MEDIAN AND MODE LOGIC
    let nums = vec![2, 3, 2, 5, 6, 1, 5, 7, 8, 9, 5, 10, 12, 16, 12, 9];

    // Calculate the median
    let mut sorted_nums = nums.clone();
    sorted_nums.sort();
    let median = sorted_nums[sorted_nums.len() / 2];

    // Calculate the mode
    let mut frequency_map = HashMap::new();
    for num in &nums {
        let count = frequency_map.entry(num).or_insert(0);
        *count += 1;
    }
    let mut mode = None;
    let mut max_count = 0;
    for (num, count) in frequency_map {
        if count > max_count {
            max_count = count;
            mode = Some(num);
        }
    }

    println!("Median: {}", median);
    println!("Mode: {:?}", mode);

    let pig_latined = pig_latin(&"zealy Burak from Italy and Paris");
    println!("Mode: {}", pig_latined);

    // DEPARTMENT LOGIC
    
    let mut departments: HashMap<String, Vec<String>> = HashMap::new();

    loop {
        println!("Enter a command (Add <name> to <department>, List <department>, ListAll):");

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        let words: Vec<&str> = input.trim().split(' ').collect();

        if words[0].to_lowercase() == "add" && words[2].to_lowercase() == "to" {
            let name = words[1].to_string();
            let department = words[3].to_string();

            let employees = departments.entry(department).or_insert(Vec::new());
            employees.push(name);
            employees.sort();
        } else if words[0].to_lowercase() == "list" && words.len() == 2 {
            let department = words[1].to_string();

            if let Some(employees) = departments.get(&department) {
                for employee in employees {
                    println!("{}", employee);
                }
            } else {
                println!("No employees in that department");
            }
        } else if words[0].to_lowercase() == "listall" && words.len() == 1 {
            let mut departments_list: Vec<_> = departments.iter().collect();
            departments_list.sort_by_key(|(department, _)| department.to_lowercase());

            for (department, employees) in departments_list {
                println!("{}:", department);
                for employee in employees {
                    println!("  {}", employee);
                }
            }
        } else if words[0].to_lowercase() == "q" {
            println!("quit");
            break;
        } else {
            println!("invalid command");
        }
    }
}
