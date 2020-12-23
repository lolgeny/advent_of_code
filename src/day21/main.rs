use std::collections::{HashMap, HashSet};

fn main() {
    let input = std::fs::read_to_string("src/day21/input.txt").unwrap();
    let mut possibles = HashMap::new(); // <allergen, HashSet<possible ingredients>>
    // let mut ingredient_count = HashMap::new(); // <ingredient, appearances>

    //// parse the input
    for line in input.lines() {
        // parse line
        let mut chars = line.chars().peekable();
        let mut ingredients = HashSet::new();
        while *chars.peek().unwrap() != '(' {
            let mut ingredient = String::new();
            while *chars.peek().unwrap() != ' ' {
                ingredient.push(chars.next().unwrap());
            }
            chars.next();
            ingredients.insert(ingredient.clone());
            // if !ingredient_count.contains_key(&ingredient) {ingredient_count.insert(ingredient.clone(), 0);}
            // ingredient_count.insert(ingredient.clone(), ingredient_count[&ingredient] + 1);
        }
        chars.next();
        while *chars.peek().unwrap() != ' ' {chars.next();}
        chars.next();
        let mut allergens = HashSet::new();
        while *chars.peek().unwrap() != ')' {
            let mut allergen = String::new();
            while chars.peek().unwrap().is_alphabetic() {
                allergen.push(chars.next().unwrap());
            }
            allergens.insert(allergen);
            if *chars.peek().unwrap() == ')' {break}
            chars.next();
            chars.next();
        }
        // Update possible allergens to ingredients+
        for allergen in allergens {
            if !possibles.contains_key(&allergen) {possibles.insert(allergen.clone(), ingredients.clone());}
            let mut new_possible = possibles.get(&allergen).unwrap().clone();
            new_possible.retain(|i| ingredients.contains(i));
            possibles.insert(allergen.clone(), new_possible);
        }
    }

    // //// Calculate unused ingredients
    // dbg!(possibles);
    /*let mut total_unused = 0;
    'ingredients: for (ingredient, num) in ingredient_count {
        for (_, ingredients) in possibles.iter() {
            if ingredients.contains(&ingredient) {continue 'ingredients;}
        }
        total_unused += num;
    }
    println!("Total unused {}", total_unused)*/

    //// Figure out which ingredient each allergen refers too
    let mut allergens = HashMap::new(); // <allergen, ingredient>
    let mut found_ingredients: HashSet<String> = HashSet::new();
    while possibles.iter().any(|(_, ingredients)| !ingredients.is_empty()) {
        // dbg!(&possibles);
        for (allergen, ingredients) in possibles.iter_mut() {
            for found in found_ingredients.iter() {
                ingredients.remove(found);
            }
            if ingredients.len() == 1 {
                let ingredient = ingredients.iter().next().unwrap();
                allergens.insert(allergen.clone(), ingredient.clone());
                found_ingredients.insert(ingredient.clone());
            }
        }
    }

    //// sort the allergens and print
    let mut allergens: Vec<(String, String)> = allergens.iter().map(|(k, v)| (k.clone(), v.clone())).collect();
    allergens.sort_by(|(allergen, _), (other, _)| allergen.cmp(other));
    let mut ingredient_list = String::new();
    dbg!(&allergens);
    for (_, ingredient) in allergens {
        ingredient_list.push_str(ingredient.as_str());
        ingredient_list.push(',');
    }
    println!("{}", ingredient_list);
}