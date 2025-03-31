use std::collections::HashMap;

#[derive(Debug)]
struct PreferenceData {
    users_count: u32,
}

pub fn work_with_hashmaps() -> () {
   let mut devs_preferences: HashMap<String, i32> =  HashMap::new(); 

   devs_preferences.insert(String::from("VSCode"), 43);
   devs_preferences.insert(String::from("NeoVim"), 5);

    println!("HashMap content: {devs_preferences:#?}");

    let preference = "VSCode";
    // NOTE: With the `copied()` method from the `Option` enum, we are gaining ownership
    // over the returned value (if not placed, we would've gotten the pointer to the hashmap's value)
    // NOTE: With `unwrap_or` we are obtaining the actual value from the optional (if contains `Some`)
    // or the fallback provided within the brackets as method argument
    let vscode_preference = devs_preferences.get(preference)
        .copied()
        .unwrap_or(0);
    println!("The dev preference for {preference} is {vscode_preference}");

    // NOTE: If we assign a variable's value to the HashMap, it will be moved
    // (a.k.a. variable no longer usable)
    let field_name = String::from("Windows");

    let mut os_preferences: HashMap<String, PreferenceData> = HashMap::new();

    // NOTE: Variable moved here
    os_preferences.insert(
        field_name,
        PreferenceData { users_count: 32 });
    os_preferences.insert(
        String::from("MacOS"),
        PreferenceData { users_count: 100 });
}