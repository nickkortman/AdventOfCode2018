use std::io::{self, BufRead};
use std::collections::HashMap;
use std::collections::HashSet;

fn find_prototype_box(boxes: &Vec<String>)
{
    let num_chars = boxes[0].len();
    for i in 0..num_chars
    {
        let mut box_ids: HashSet<String> = HashSet::new();
        
        for box_id_ref in boxes
        {
            let mut box_id = box_id_ref.clone();
            box_id.remove(i);
            if box_ids.contains(&box_id)
            {
                println!("Prototype Box: {}", box_id);
                return;
            }
            box_ids.insert(box_id);
        }
    }
}

fn does_string_repeat(box_id: &String, two_count: &mut i128, three_count: &mut i128)
{
    let mut seen_letters = HashMap::new();

    for letter in box_id.chars()
    {
        let counter = seen_letters.entry(letter).or_insert(0);
        *counter += 1;
    }

    for count in seen_letters.values()
    {
        if *count == 2
        {
            *two_count += 1;
            break;
        }
    }

    for count in seen_letters.values()
    {
        if *count == 3
        {
            *three_count += 1;
            break;
        }
    }
}

// need id because claims might overlap with each other in multiple places, but should only be counted once.
fn map_claim(claim: &String, fabric: &mut HashMap<u128, HashMap<u128, u128>>)
{
    let (id, rest) = claim.split_at(claim.find('@').unwrap());
    let (coords, dims) = rest.split_at(rest.find(':').unwrap());
}

// Read until @
// hash on coords, map of maps
// count
fn main()
{
    let mut fabric: HashMap<u128, HashMap<u128, u128>> = HashMap::new();
    let mut two_count = 0i128;
    let mut three_count = 0i128;
    let mut boxes = Vec::new();
    let stdin = io::stdin();
    let input = stdin.lock();

    for line in input.lines()
    {
        match line
        {
            Ok(box_id) => 
            {
                does_string_repeat(&box_id, &mut two_count, &mut three_count);
                boxes.push(box_id);
            }
            Err(error) => println!("IO Error: {}", error),
        }
    }

    println!("Checksum: {}", two_count * three_count);
    find_prototype_box(&boxes);
}
