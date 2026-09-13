#![allow(dead_code, unused_variables, unused_mut)]

/// Entry point for Chapter 4.3: The Slice Type
///
/// Slices let you reference a contiguous sequence of elements in a collection
/// without taking ownership. This file explores the problem slices solve (data out-of-sync),
/// string slice range syntax, string literals as slices, idiomatic function signatures
/// using `&str`, borrow checker compile-time safety, and generic array slices (`&[T]`).
fn main() {
    the_slice_problem();
    // string_slices_basics();
    // string_literals_and_deref();
    // slice_borrow_checker_safety();
    // array_slices();
}

/// # The Problem Slices Solve (Out-of-Sync Data)
/// - Without slices, tracking sub-parts of a collection requires storing manual indices (e.g., `usize`).
/// - Manual indices are completely detached from the original container's state.
/// - If the underlying container is mutated or cleared, stored indices become invalid,
///   leading to logical bugs or runtime panics.
fn the_slice_problem() {
    println!("\n{:=>80}", "");
    println!("the_slice_problem()\n");

    let mut user_email = String::from("admin@datacenter-north.com");

    // We locate the `@` symbol index manually so we can separate username from domain.
    let at_index = find_at_symbol_index(&user_email); // Returns 5

    println!("Original email: '{user_email}'");
    println!("Found '@' symbol at index: {at_index}");

    // Now imagine `user_email` is modified or cleared elsewhere in the application...
    user_email.clear(); // `user_email` is now empty ("")!

    // `at_index` is STILL 5! It has no connection to `user_email`'s actual lifecycle.
    println!("Index variable still holds: {at_index}");

    // Trying to use `at_index` on `user_email` now will panic at runtime (index out of bounds)!
    // let domain = &user_email[at_index..]; // Uncomment to see runtime panic!
}

/// # String Slices (`&str`)
/// - A string slice is a reference to a contiguous portion of a `String`.
/// - Internal structure: A pointer to the starting byte on the heap and a length.
/// - Range syntax `[start..end]`: `start` is inclusive, `end` is exclusive.
/// - Range shorthands:
///   - `[0..2]` is equivalent to `[..2]`
///   - `[3..len]` is equivalent to `[3..]`
///   - `[0..len]` is equivalent to `[..]`
fn string_slices_basics() {
    println!("\n{:=>80}", "");
    println!("string_slices_basics()\n");

    let server_id = String::from("srv-eu-west-904");

    // Extracting parts using full range syntax:
    let prefix: &str = &server_id[0..3]; // "srv"
    let region: &str = &server_id[4..11]; // "eu-west"

    println!("Full ID: {server_id}");
    println!("Prefix:  {prefix}");
    println!("Region:  {region}");

    // Using range shorthands:
    let start_shorthand = &server_id[..3]; // Same as [0..3]
    let end_shorthand = &server_id[12..]; // Same as [12..15]
    let full_shorthand = &server_id[..]; // Slices the entire string

    println!("Start shorthand: {start_shorthand}");
    println!("End shorthand:   {end_shorthand}");
    println!("Full shorthand:  {full_shorthand}");

    // NOTE: String slice indices MUST occur at valid UTF-8 character boundaries!
    // Slicing in the middle of a multi-byte character (like some emojis) causes a runtime panic.
}

/// # String Literals Are Slices & Idiomatic Function Parameters
/// - Hardcoded string literals (e.g., `"hello"`) are of type `&str`. They point to a specific
///   address inside the compiled binary text segment.
/// - Because string literals are already `&str`, slice parameters make functions much more flexible.
/// - Declaring functions with `fn func(s: &str)` allows passing:
///   1. A string literal (`&str`).
///   2. A slice of a `String` (`&my_string[..]`).
///   3. A reference to a `String` (`&my_string`), thanks to Rust's **Deref Coercion**.
fn string_literals_and_deref() {
    println!("\n{:=>80}", "");
    println!("string_literals_and_deref()\n");

    // String literal (type is &str)
    let static_endpoint: &str = "https://api.v1.service.org/health";

    // Dynamic Heap String (type is String)
    let dynamic_endpoint: String = String::from("https://api.v2.service.org/metrics");

    // `extract_domain` accepts `&str`. We can pass a string literal directly:
    let domain1 = extract_domain(static_endpoint);

    // We can pass a slice of a heap `String`:
    let domain2 = extract_domain(&dynamic_endpoint[..]);

    // Or we can pass `&String` directly—Rust automatically converts `&String` to `&str` (Deref Coercion):
    let domain3 = extract_domain(&dynamic_endpoint);

    println!("Domain 1: {domain1}");
    println!("Domain 2: {domain2}");
    println!("Domain 3: {domain3}");
}

/// # Slice Safety & The Borrow Checker
/// - Returning a slice (`&str`) ties the borrow directly to the underlying `String`.
/// - Because a slice holds an immutable reference to the data, the Borrow Checker enforces that
///   the original `String` CANNOT be mutated while the slice is active.
/// - This completely eliminates the out-of-sync index bug at compile time!
fn slice_borrow_checker_safety() {
    println!("\n{:=>80}", "");
    println!("slice_borrow_checker_safety()\n");

    let mut user_email = String::from("developer@rust-lang.org");

    // `extract_username` returns a `&str` slice pointing into `user_email`.
    let username = extract_username(&user_email);

    // `username` holds an IMMUTABLE borrow of `user_email`.
    // Attempting to mutate `user_email` here will fail at compile time!
    // user_email.clear(); // Uncomment to see compile error: cannot borrow as mutable!

    // We can read both safely:
    println!("Username slice: '{username}'");
    println!("Full email:     '{user_email}'");

    // After `username` is no longer used, `user_email` can be mutated again (NLL).
    user_email.clear();
    println!("Email cleared successfully after slice scope ended.");
}

/// # Array Slices (`&[T]`)
/// - Slices are not limited to strings; they work on all generic, contiguous collections.
/// - An array slice has type `&[T]`, where `T` is the element type.
/// - Array slices store a pointer to the first element and a length count.
fn array_slices() {
    println!("\n{:=>80}", "");
    println!("array_slices()\n");

    let telemetry_data: [i32; 6] = [102, 105, 110, 340, 420, 108];

    // Take a slice of elements from index 1 to 4 (indices 1, 2, 3):
    let normal_readings: &[i32] = &telemetry_data[0..3];
    let spike_readings: &[i32] = &telemetry_data[3..5];

    println!("Original array length: {}", telemetry_data.len());
    println!("Normal slice length:   {}", normal_readings.len());

    let avg = calculate_average(normal_readings);
    println!("Average of normal readings: {avg}");

    println!("Spike readings slice: {:?}", spike_readings);
}

// ========================================================================= //
// NOTE! Helper functions demonstrating manual indices vs. slice returns.
// ========================================================================= //

/// Returns the raw byte index of the `@` symbol in a `String`.
/// Demonstrates the unsafe manual index approach.
fn find_at_symbol_index(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b'@' {
            return i;
        }
    }

    s.len()
}

/// Accepts a string slice (`&str`) and returns a slice corresponding to the domain name.
/// Using `&str` parameter allows passing `&String`, `&str`, or sub-slices.
fn extract_domain(url: &str) -> &str {
    // Look for "//" prefix in URL
    let path_start = match url.find("//") {
        Some(index) => index + 2,
        None => 0,
    };

    let rest = &url[path_start..];

    // Find end of domain marked by '/'
    match rest.find('/') {
        Some(index) => &rest[..index],
        None => rest,
    }
}

/// Returns a slice containing only the username portion of an email.
fn extract_username(email: &str) -> &str {
    let bytes = email.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b'@' {
            return &email[..i];
        }
    }

    &email[..]
}

/// Accepts a generic integer slice (`&[i32]`) and calculates the arithmetic mean.
fn calculate_average(numbers: &[i32]) -> i32 {
    if numbers.is_empty() {
        return 0;
    }

    let mut sum = 0;
    for &num in numbers {
        sum += num;
    }

    sum / numbers.len() as i32
}
