use std::env;
use std::process;

fn main() {
    let args: Vec<_> = env::args().collect();
    
    if args.len() == 1 {
        println!("No Number Provided for conversion");
        process::exit(1);
    }
    
    if args.len() >= 3 {
        println!("Too many parameters provided");
        process::exit(1);
    }
    
    let n: u32 = parse_string(&args[1]);

    convert_to_roman_numeral(&n);
}

fn convert_to_roman_numeral(input_arg: &u32) {

    if input_arg == &0 {
        println!("Input: {}", input_arg);
        println!("Output: No Roman Numeral for 0!");
        process::exit(0);
    }

    let mut output_string;

    //Not Happy about building a string and the order of the function calls is required
    output_string = convert_thousands(input_arg);
    output_string.push_str(&convert_hundreds(input_arg));
    output_string.push_str(&convert_tens(input_arg));
    output_string.push_str(&convert_ones(input_arg));

    println!("Input: {}", input_arg);
    println!("Output: {}", output_string);
}

//Not happy that this is a special case outside generic convert
fn convert_thousands(input_number: &u32) -> String {
   let mut roman_thousands = String::new();
   for _x in 0..(input_number - (input_number % 1000)) / 1000 {
       roman_thousands.push('M');
   }
   return roman_thousands;
}

fn convert_hundreds(input_number: &u32) -> String {
    return generic_convert(input_number, &100, 'C', 'D');
}

fn convert_tens(input_number: &u32) -> String {
    return generic_convert(input_number, &10, 'X', 'L'); 
}

fn convert_ones(input_number: &u32) -> String {
    return generic_convert(input_number, &1, 'I', 'V');
}

fn generic_convert(number_to_convert: &u32, place: &u32, unus_char: char, quint_char: char) -> String {
    let mut roman_conversion = String::new();
    
    //Default conversion
    for _x in 0..(number_to_convert % (place * 10)) / place {
        roman_conversion.push(unus_char);
    }
    
    //Special Case 4
    if roman_conversion.len() == 4 {
        roman_conversion = "".to_string();
        roman_conversion.push(unus_char);
        roman_conversion.push(quint_char);
    }
   
    // 5 or more
    if roman_conversion.len() >= 5 {
        let mut concat = quint_char.to_string();
        let truncate_len = roman_conversion.len() - 5;
        roman_conversion.truncate(truncate_len);
        concat.push_str(&roman_conversion);
        roman_conversion = concat;
    }

    return roman_conversion;   
}

fn parse_string(number_str: &str) -> u32 {
    let number: u32 = match number_str.parse::<u32>() {
        Ok(n) => n,
        Err(_err) => {
            println!("Provided argument is not a whole number");
            process::exit(1);
        },
    };
    return number;
}
