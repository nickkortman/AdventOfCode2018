use std::io::{self, BufRead};
use std::collections::HashSet;

fn monitor_frequency(change: &String, frequency: &mut i128, seen_frequencies: &mut HashSet<i128>) -> bool
{
	let (sign, num) = change.split_at(1);
	let num_val = num.parse::<i128>();
	match num_val
	{
		Ok(value) =>
		{
			if sign == "+"
			{
				*frequency += value;
			}
			else
			{
				*frequency -= value;
			}

            if seen_frequencies.contains(&*frequency)
            {
                println!("Repeated frequency: {}", *frequency);
                return true;
            }
            seen_frequencies.insert(*frequency);
		}
		Err(error) => println!("Parse Error: {}", error),
	}

    false
}

fn main()
{
    let mut frequency = 0i128;
    let mut seen_frequencies: HashSet<i128> = HashSet::new();
	let mut changes: Vec<String> = Vec::new();
    let stdin = io::stdin();
	let input = stdin.lock();

    for line in input.lines()
    {
		match line
		{
        	Ok(change) => 
			{
				if monitor_frequency(&change, &mut frequency, &mut seen_frequencies)
                {
                    return;
                }
				changes.push(change);
			}
			Err(error) => println!("IO Error: {}", error),
		}
    }
	
	loop
	{
		for change in &changes
		{
			if monitor_frequency(&change, &mut frequency, &mut seen_frequencies)
			{
				return;
			}
		}
	}
}
