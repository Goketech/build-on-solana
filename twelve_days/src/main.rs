// Main function - entry point of the program
fn main() {
    // Print the title
    println!("🎄 The Twelve Days of Christmas 🎄\n");
    
    // Iterate through days 1 to 12
    // 1..=12 creates an inclusive range that includes both 1 and 12
    for day in 1..=12 {
        // Call the function to print the verse for this day
        print_verse(day);
        // Print a blank line between verses for readability
        println!();
    }
}

// Function to print a complete verse for a given day
// Takes a day number (1-12) as a parameter
fn print_verse(day: u32) {
    // Print the opening line with the ordinal number (first, second, etc.)
    println!(
        "On the {} day of Christmas, my true love gave to me:",
        get_ordinal(day)
    );
    
    // Print gifts in reverse order (from current day down to day 1)
    // This takes advantage of the repetition in the song
    // Each verse builds on the previous one by adding gifts in reverse
    for gift_day in (1..=day).rev() {
        // .rev() reverses the range iterator
        // So for day 3: it goes 3, 2, 1 instead of 1, 2, 3
        print_gift(gift_day, day);
    }
}

// Function to get the ordinal number name (first, second, third, etc.)
// Returns a string slice (&str) which is a reference to string data
// The 'static lifetime means this string exists for the entire program
fn get_ordinal(day: u32) -> &'static str {
    // Match expression to return the appropriate ordinal
    // This is like a switch statement in other languages
    match day {
        1 => "first",
        2 => "second",
        3 => "third",
        4 => "fourth",
        5 => "fifth",
        6 => "sixth",
        7 => "seventh",
        8 => "eighth",
        9 => "ninth",
        10 => "tenth",
        11 => "eleventh",
        12 => "twelfth",
        // The _ pattern matches anything not covered above (default case)
        // This should never happen in our program, but Rust requires exhaustive matching
        _ => "unknown",
    }
}

// Function to print the gift for a specific day
// Parameters:
//   gift_day: which gift to print (1-12)
//   current_day: the current verse day (used to handle "and" on day 1)
fn print_gift(gift_day: u32, current_day: u32) {
    // Get the gift text for this day
    let gift = match gift_day {
        1 => {
            // Special case for the first gift
            // If we're beyond day 1, add "And" before the last line
            // This is because "And a partridge in a pear tree" is used
            // for all verses except the very first verse
            if current_day > 1 {
                "And a partridge in a pear tree."
            } else {
                "A partridge in a pear tree."
            }
        }
        2 => "Two turtle doves,",
        3 => "Three French hens,",
        4 => "Four calling birds,",
        5 => "Five golden rings,",  // Often emphasized when sung!
        6 => "Six geese a-laying,",
        7 => "Seven swans a-swimming,",
        8 => "Eight maids a-milking,",
        9 => "Nine ladies dancing,",
        10 => "Ten lords a-leaping,",
        11 => "Eleven pipers piping,",
        12 => "Twelve drummers drumming,",
        // Default case - should never be reached
        _ => "Unknown gift,",
    };
    
    // Print the gift line
    println!("{}", gift);
}

// Learning notes:
// 
// 1. **Ranges**: We use 1..=12 (inclusive) and .rev() to reverse iteration
//    - 1..12 would exclude 12 (exclusive range)
//    - 1..=12 includes 12 (inclusive range)
//
// 2. **String slices (&str)**: Efficient references to string data
//    - Don't own the data, just point to it
//    - 'static lifetime means the data lives for the entire program
//    - Stored in the program's binary (immutable)
//
// 3. **Match expressions**: Exhaustive pattern matching
//    - All possible values must be covered
//    - Returns a value (can be assigned to variables)
//    - More powerful than switch statements in many languages
//
// 4. **Functions**: Modular code organization
//    - Each function has a single responsibility
//    - Makes code easier to read, test, and maintain
//
// 5. **Loops and iterators**: for loops work with anything that implements Iterator
//    - Ranges (1..=12) are iterators
//    - .rev() transforms the iterator to go backwards
//
// 6. **Conditional logic in match**: The first gift changes based on context
//    - Shows how to handle special cases elegantly
//    - Demonstrates Rust's expression-oriented nature
