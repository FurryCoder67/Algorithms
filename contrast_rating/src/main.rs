fn get_contrast_rating(contrast: &str, large_text: bool) -> String {
    let ratio: f64 = contrast.parse().unwrap();
    if large_text {
        if ratio >= 4.5 {
            "AAA".to_string()
        } else if ratio >= 3.0 {
            "AA".to_string()
        } else {
            "Fail".to_string()
        }
    } else {
        if ratio >= 7.0 {
            "AAA".to_string()
        } else if ratio >= 4.5 {
            "AA".to_string()
        } else {
            "Fail".to_string()
        }
    }
}

fn main() {
    assert_eq!(get_contrast_rating("7.5", false), "AAA");
    println!("Test 1 passed!");

    assert_eq!(get_contrast_rating("4.8", false), "AA");
    println!("Test 2 passed!");

    assert_eq!(get_contrast_rating("4.2", false), "Fail");
    println!("Test 3 passed!");

    assert_eq!(get_contrast_rating("4.5", true), "AAA");
    println!("Test 4 passed!");

    assert_eq!(get_contrast_rating("3.0", true), "AA");
    println!("Test 5 passed!");

    assert_eq!(get_contrast_rating("2.7", false), "Fail");
    println!("Test 6 passed!");

    println!("All tests passed!");
}